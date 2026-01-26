use crate::canvas_draw::*;
use crate::util::*;
use rand::seq::IndexedRandom;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::collections::BTreeMap;
use std::ops::Add;

pub struct GeneralError {}

pub struct TechnicalError {
    // Move bottle cap on the side (two wheels required)
}

pub struct Sips {
    pub start_last: bool,
    pub end_first: bool,
    pub whines: u8,
    pub remaining_steps: u8,
    pub fell_out: bool,
    pub back_on: bool,
    pub ones: u8, // Hån - ridicule
}

impl Default for Sips {
    fn default() -> Sips {
        Sips {
            start_last: false,
            end_first: false,
            whines: 0,
            remaining_steps: 0,
            fell_out: false,
            back_on: false,
            ones: 0,
        }
    }
}

#[derive(Clone)]
pub struct Player {
    pub color: Color,
    pub radius: f64,
    pub position: Coord,
    pub old_position: Coord,
    pub direction: Direction,

    pub outside_board: bool,
    pub next_falls_off: bool,
    pub forced_gear_down: bool,

    pub gear: u8,

    pub roll: Vec<u8>,
    pub steps: u8,
    pub stalled: bool,

    pub bonked: bool,
    pub finished: bool,
    pub round: usize,
    pub first_half: bool,
}

pub trait StepStrategy {
    fn step_strategy(&mut self, player: &Player, turns: &Vec<Turn>, tile: &Tile) -> Option<Turn>;
}

pub trait GearStrategy {
    fn gear_strategy(&mut self, player: &Player) -> ChangeGear;
}

impl Player {
    // pub fn reset(&mut self) {
    //     self.stalled = false;
    //     // pub roll: Vec<u8>,
    //     // pub steps: u8,
    //     // pub stalled: bool,

    //     // pub bonked: bool,
    //     // pub finished: bool,

    // }

    pub fn draw(&self, canvas: &mut Canvas<Window>, start: Coord, scale: f64) {
        let c = start + self.position * (scale as i32);
        canvas.set_draw_color(self.color);

        for i in -1..=1 {
            for j in -1..=1 {
                let _ = draw_hexagon(canvas, c.x() + i, c.y() + j, scale * self.radius);
            }
        }

        for i in -1..=1 {
            for j in -1..=1 {
                let _ = draw_hexagon(canvas, c.x() + i, c.y() + j, 0.5 * scale * self.radius);
            }
        }

        let angle = self.direction.angle();
        let cx = c.x() + (0.85 * scale / 2_f64.sqrt() * angle.cos()) as i32;
        let cy = c.y() + (0.85 * scale / 2_f64.sqrt() * angle.sin()) as i32;

        for i in -1..=1 {
            for j in -1..=1 {
                let _ = canvas.draw_line((c.x() + i, c.y() + j), (cx + i, cy + j));
            }
        }
    }

    pub fn step_possibilities(
        &self,
        tiles: &BTreeMap<Coord, Tile>,
        choice_vec: &Option<&Vec<Direction>>,
    ) -> Vec<Turn> {
        if !tiles.contains_key(&self.position) {
            return vec![Turn::Straight];
        }

        let curr_tile = tiles[&self.position].clone();
        let dir = if !curr_tile.forced.is_empty() {
            println!(
                "Bug??: {:?} {:?} {:?}",
                curr_tile.forced, self.position, self.old_position
            );

            // best dirs [Right, Straight, Straight, Right]
            // ITER
            // Bug??: {Coord { q: -2, r: -6 }: DL, Coord { q: -1, r: -5 }: UL} Coord { q: -2, r: -5 } Coord { q: -3, r: -4 }

            vec![curr_tile.forced[&self.old_position]] // Old position need not be there?
        } else if let Some(choice_vec) = choice_vec && self.round <= choice_vec.len() {
            println!("Choice vec: {:?}", choice_vec);
            println!("Directions {:?} vs default {:?}: ", vec![choice_vec[self.round-1]], curr_tile.directions);
            vec![choice_vec[self.round-1]] // Only one?
        } else {
            curr_tile.directions
        };

        let result = dir.into_iter()
            .filter_map(|x| self.direction.turn_to_dir(x))
            .filter(|x| !(self.roll.iter().fold(0, u8::add) > 9) || *x == Turn::Straight) // If too fast to turn
            .collect();

        println!("Possible turns: {:?}", result);

        result
    }

    pub fn pre_step(
        &mut self,
        tiles: &BTreeMap<Coord, Tile>,
        blockages: &Vec<Coord>,
        choice_vec: &BTreeMap<Coord, Vec<Direction>>,
    ) -> Option<Vec<Turn>> {
        if self.next_falls_off {
            self.old_position = self.position;
            self.position = self.position + self.direction.to_coord();

            self.outside_board = true;
            self.finished = true;
            self.stalled = true;
            self.next_falls_off = false;

            return None; // Done
        } else if self.outside_board {
            self.outside_board = false;
            self.position = self.old_position;
            self.old_position = self.position;

            if let Some(choice_vec) =  choice_vec.get(&self.position) && self.round <= choice_vec.len() {
                self.direction = choice_vec[self.round-1];
            } else {
                self.direction = tiles[&self.position].directions[0]; // TODO: allow stategy when re-entering the board
            }
            return None; // Done
        } else {
            self.old_position = self.position;
            self.position = self.position + self.direction.to_coord();
        }

        if blockages.contains(&self.position) {
            self.position = self.old_position;
            self.finished = true;
            self.bonked = true;
            return None; // Done
        }

        // Outside bord
        if !tiles.contains_key(&self.position) {
            self.outside_board = true;
            self.finished = true;
            self.stalled = true;
            return None;
        }

        Some(self.step_possibilities(tiles, &choice_vec.get(&self.position)))
    }

    pub fn step(
        &mut self,
        turns: &Vec<Turn>,
        tiles: &BTreeMap<Coord, Tile>,
        strategy: &mut impl StepStrategy,
    ) -> bool {
        let curr_tile = tiles[&self.position].clone();

        let Some(mut turn) = (if self.roll.iter().fold(0, u8::add) > 9 {
            Some(Turn::Straight)
        } else {
            strategy.step_strategy(self, &turns, &curr_tile)
        }) else {
            return false;
        };

        println!("Possible turns: {:?} chose {:?}", turns, turn);
        println!("Choice? {:?}", curr_tile.choice);

        // Must use valid turns (e.g. forced) when going over choice
        if curr_tile.choice && !turns.contains(&turn) {
            println!("Wrong direction, fell off track!");
            self.next_falls_off = true;
        }

        if (!curr_tile.forced.is_empty() || self.roll.iter().fold(0, u8::add) > 9)
            && !turns.contains(&turn)
        {
            if turns.is_empty() {
                self.next_falls_off = true;
                turn = Turn::Straight;
            } else {
                // Force valid
                turn = turns[0];
            }
        };

        if curr_tile.blockage.contains(&(self.direction + turn)) {
            self.next_falls_off = true;
        }

        self.direction = self.direction + turn;
        self.steps += 1;
        // TODO: Allow player to call finished steps themselves
        if self.steps == self.roll.iter().fold(0, u8::add) {
            self.finished = true;
        }

        return true;
    }

    pub fn roll_dice(&mut self, mut strategy: impl GearStrategy) {
        self.finished = false;

        if self.stalled {
            self.gear = 1;
            self.stalled = false;
        } else if self.forced_gear_down {
            self.gear = ChangeGear::Down.update_gear(self.gear);
            self.forced_gear_down = false;
        } else {
            self.gear = strategy.gear_strategy(&self).update_gear(self.gear);
        }

        let dice_dist: Vec<_> = (1..=4_u8).into_iter().collect();
        let mut rng = rand::rng();
        self.roll = (0..self.gear)
            .map(|_| *dice_dist.choose(&mut rng).unwrap())
            .collect();
        self.steps = 0; // self.roll.iter().fold(0, u8::add);
        println!("Roll: {:?}", self.roll);
    }

    // nøl - hesitation / dither
    pub fn dither() {}
}
