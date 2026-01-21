use crate::canvas_draw::*;
use crate::util::*;
use rand::seq::IndexedRandom;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::*;
use std::cmp::{max, min};
use std::collections::{BTreeMap, BTreeSet};
use std::ops::Add;
use std::time::Duration;

#[derive(Clone)]
pub struct Player {
    pub color: Color,
    pub radius: f64,

    pub position: Coord,
    pub old_position: Coord,
    pub direction: Direction,

    pub outside_board: bool,
    pub gear: u8,
    pub roll: Vec<u8>,
    pub steps: u8,
    pub stalled: bool,

    pub bonked: bool,
    pub finished: bool,

    pub next_falls_off: bool,

    pub forced_gear_down: bool,
}

impl Player {
    pub fn draw(&self, canvas: &mut Canvas<Window>, start: Coord, scale: f64) {
        let c = start + self.position * (scale as i32);
        canvas.set_draw_color(self.color);

        let _ = draw_hexagon(canvas, c.x(), c.y(), scale * self.radius);
        let _ = draw_hexagon(canvas, c.x() + 1, c.y(), scale * self.radius);
        let _ = draw_hexagon(canvas, c.x() + 1, c.y() + 1, scale * self.radius);

        let _ = draw_hexagon(canvas, c.x(), c.y(), 0.5 * scale * self.radius);
        let _ = draw_hexagon(canvas, c.x() + 1, c.y(), 0.5 * scale * self.radius);
        let _ = draw_hexagon(canvas, c.x() + 1, c.y() + 1, 0.5 * scale * self.radius);

        let angle = self.direction.angle();
        let cx = c.x() + (0.85 * scale / 2_f64.sqrt() * angle.cos()) as i32;
        let cy = c.y() + (0.85 * scale / 2_f64.sqrt() * angle.sin()) as i32;

        for i in -1..=1 {
            let _ = canvas.draw_line((c.x() + i, c.y()), (cx + i, cy));
        }
        for i in -1..=1 {
            let _ = canvas.draw_line((c.x(), c.y() + i), (cx, cy + i));
        }
    }

    pub fn step_possibilities(
        &self,
        tiles: &BTreeMap<Coord, Tile>,
        blockages: &Vec<Coord>,
    ) -> Vec<Turn> {
        if !tiles.contains_key(&self.position) {
            return vec![Turn::Straight];
        }

        let curr_tile = tiles[&self.position].clone();
        let dir = if !curr_tile.forced.is_empty() {
            println!("Bug??: {:?} {:?} {:?}", curr_tile.forced, self.position, self.old_position);

            // best dirs [Right, Straight, Straight, Right]
            // ITER
            // Bug??: {Coord { q: -2, r: -6 }: DL, Coord { q: -1, r: -5 }: UL} Coord { q: -2, r: -5 } Coord { q: -3, r: -4 }

            vec![curr_tile.forced[&self.old_position]] // Old position need not be there?
        } else {
            curr_tile.directions
        };

        dir.into_iter()
            .filter_map(|x| self.direction.turn_to_dir(x))
            .filter(|x| !(self.roll.iter().fold(0, u8::add) > 9) || *x == Turn::Straight) // If too fast to turn
            .collect()
    }

    pub fn step<F: Fn(&Vec<Turn>, Direction, Coord, &Tile) -> Turn>(
        &mut self,
        tiles: &BTreeMap<Coord, Tile>,
        blockages: &Vec<Coord>,
        strategy: F,
    ) {
        if self.next_falls_off {
            self.old_position = self.position;
            self.position = self.position + self.direction.to_coord();

            self.outside_board = true;
            self.finished = true;
            self.stalled = true;
            self.next_falls_off = false;

            return;
        } else if self.outside_board {
            self.outside_board = false;
            self.position = self.old_position;
            self.old_position = self.position;
            self.direction = tiles[&self.position].directions[0]; // TODO: allow stategy when re-entering the board
            return;
        } else {
            self.old_position = self.position;
            self.position = self.position + self.direction.to_coord();
        }

        if blockages.contains(&self.position) {
            self.position = self.old_position;
            self.finished = true;
            self.bonked = true;
            return;
        }

        // .filter_map(|x| {
        //     let dir_pos = self.position + x.to_coord();
        //     println!("Blockage {:?} contains {:?}?: {:?}", blockages, dir_pos, blockages.contains(&dir_pos));
        //     if blockages.contains(&dir_pos) {
        //         None
        //     } else {
        //         Some(x)
        //     }
        // })

        // Outside bord
        if !tiles.contains_key(&self.position) {
            self.outside_board = true;
            self.finished = true;
            self.stalled = true;
            return;
        }

        let turns = self.step_possibilities(tiles, blockages);

        let curr_tile = tiles[&self.position].clone();
        let mut turn = strategy(&turns, self.direction, self.position, &curr_tile);

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

        // if !turns.contains(&turn) {
        //     panic!("invalid turn");
        // }

        self.direction = self.direction + turn;
        self.steps += 1;
        // TODO: Allow player to call finished steps themselves
        if self.steps == self.roll.iter().fold(0, u8::add) {
            self.finished = true;
        }
    }

    pub fn roll_dice<F: Fn(u8) -> ChangeGear>(&mut self, strategy: F) {
        self.finished = false;

        if self.stalled {
            self.gear = 1;
            self.stalled = false;
        } else if self.forced_gear_down {
            self.gear = ChangeGear::Down.update_gear(self.gear);
            self.forced_gear_down = false;
        } else {
            self.gear = strategy(self.gear).update_gear(self.gear);
        }

        let dice_dist: Vec<_> = (1..=4_u8).into_iter().collect();
        let mut rng = rand::rng();
        self.roll = (0..self.gear)
            .map(|_| *dice_dist.choose(&mut rng).unwrap())
            .collect();
        self.steps = 0; // self.roll.iter().fold(0, u8::add);
        println!("Roll: {:?}", self.roll);
    }
}
