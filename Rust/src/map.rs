use crate::canvas_draw::*;
use crate::player::*;
use crate::util::*;
use rand::seq::IndexedRandom;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::*;
use std::collections::{BTreeMap, BTreeSet};
use std::time::Duration;

#[derive(Clone)]
pub struct PlayerBuilder {
    pub total: usize,
    pub index: usize,
    pub colors: Vec<Color>,
    pub placements: Vec<(Coord, Direction)>,
}

impl PlayerBuilder {
    pub fn new(
        total: usize,
        colors: Vec<Color>,
        placements: Vec<(Coord, Direction)>,
    ) -> PlayerBuilder {
        PlayerBuilder {
            index: 0,
            total,
            colors,
            placements,
        }
    }

    pub fn next_player(&mut self) -> Player {
        let color = self.colors[self.index % self.colors.len()];
        let radius = 0.2 + (0.9 - 0.2) * (self.index as f64) / (self.total as f64);

        let (position, direction) = self.placements[self.index];

        self.index += 1;

        Player {
            position,
            old_position: position,
            direction,
            color,
            radius,
            outside_board: false,
            gear: 1,
            stalled: true,
            roll: vec![], // Set by roll_dice
            steps: 0,     // Set by roll_dice
            bonked: false,
            finished: false,
            forced_gear_down: false,
            next_falls_off: false,
            round: 1,
            first_half: true,
        }
    }

    pub fn all_players(&mut self) -> Vec<Player> {
        (self.index..self.total)
            .map(|_| self.next_player())
            .collect()
    }
}

#[derive(Clone)]
pub struct HexMap {
    pub tiles: BTreeMap<Coord, Tile>,
    pub start_line: Vec<(Coord, Vec<Direction>)>,
    pub mid_line: Vec<(Coord, Vec<Direction>)>,
    pub player_builder: PlayerBuilder,
}

impl HexMap {
    pub fn shortest_path(&mut self) -> BTreeMap<Coord, Vec<Direction>> {
        let mut shortest_dist_map: BTreeMap<Coord, Vec<Direction>> = BTreeMap::new();

        let mut stk = vec![];
        {
            for (c, dirs) in &self.start_line {
                for d in dirs {
                    for t in [Turn::Straight, Turn::Left, Turn::Right] {
                        let new_d = *d + t;
                        let new_c = *c - new_d.to_coord();
                        stk.push(((new_c, new_d), (*c, *d), 0));
                    }
                }
            }
        }

        let mut visited: BTreeSet<(Coord, Direction)> = BTreeSet::new();
        let mut index = 0;
        while index < stk.len() {
            let ((c, d), (nc, _), _) = stk[index];
            index += 1;

            if visited.contains(&(c, d)) {
                continue;
            }
            visited.insert((c, d));

            // Fell outside map (TODO: Allow this to reset direction!)
            if !self.tiles.contains_key(&c) {
                continue;
            }

            for t in [Turn::Straight, Turn::Left, Turn::Right] {
                let new_d = d + t;
                let new_c = c - new_d.to_coord();
                stk.push(((new_c, new_d), (c, d), index - 1));
            }
        }

        for (c, _t) in &self.tiles {
            let mut dirs = vec![];
            for ((sc, sd), _, _) in &stk {
                if *c == *sc {
                    dirs.push(*sd);
                }
            }
            shortest_dist_map.insert(*c, dirs);
        }

        shortest_dist_map
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, start: Coord, scale: f64) {
        // Draw
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for (c, t) in &self.tiles {
            let c = start + *c * (scale as i32);
            t.draw(canvas, c, scale);
        }

        for (c, dirs) in &self.start_line {
            let c = start + *c * (scale as i32);
            for d in dirs {
                for i in -2..=2 {
                    for j in -2..=2 {
                        if (((c.x() + i + 2) + (c.y() + j + 2) + 3) / 4) % 2 == 0 {
                            let _ = draw_hexagon_side_checkerboard(
                                canvas,
                                c.x() + i,
                                c.y() + j,
                                scale,
                                *d,
                                Color::RGB(0, 0, 0),
                                Color::RGB(255, 255, 255),
                            );
                        } else {
                            let _ = draw_hexagon_side_checkerboard(
                                canvas,
                                c.x() + i,
                                c.y() + j,
                                scale,
                                *d,
                                Color::RGB(255, 255, 255),
                                Color::RGB(0, 0, 0),
                            );
                        }
                    }
                }
            }
        }

        for (c, dirs) in &self.mid_line {
            let c = start + *c * (scale as i32);
            for d in dirs {
                // canvas.set_draw_color(Color::RGB(255, 0, 255));
                for i in -2..=2 {
                    for j in -2..=2 {
                        if (((c.x() + i + 2) + (c.y() + j + 2) + 3) / 4) % 2 == 0 {
                            let _ = draw_hexagon_side_checkerboard(
                                canvas,
                                c.x() + i,
                                c.y() + j,
                                scale,
                                *d,
                                Color::RGB(0, 0, 0),
                                Color::RGB(255, 255, 255),
                            );
                        } else {
                            let _ = draw_hexagon_side_checkerboard(
                                canvas,
                                c.x() + i,
                                c.y() + j,
                                scale,
                                *d,
                                Color::RGB(255, 255, 255),
                                Color::RGB(0, 0, 0),
                            );
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum PlayerStepStrategy {
    Best,
    Manual,
}

#[derive(Clone, Copy)]
pub enum PlayerGearStrategy {
    Best,
    Manual,
}

pub struct GameState<'a> {
    pub map: HexMap,
    pub players: Vec<Player>,
    pub player_strategies: Vec<(PlayerGearStrategy, PlayerStepStrategy)>,

    pub player_index: usize,
    pub rolling: bool,
    pub blockages: Vec<Coord>,
    pub shortest_dist_map: BTreeMap<Coord, Vec<Direction>>,

    pub start: Coord,
    pub scale: f64,
    pub canvas: &'a mut Canvas<Window>,

    pub choice_tile_selections: BTreeMap<Coord, Vec<Direction>>,
}

pub struct BestStepStategy<'a> {
    pub blockages: &'a Vec<Coord>,
    pub shortest_dist_map: &'a BTreeMap<Coord, Vec<Direction>>,
}

impl<'a> StepStrategy for BestStepStategy<'a> {
    fn step_strategy(&mut self, player: &Player, turns: &Vec<Turn>, tile: &Tile) -> Option<Turn> {
        let dir: Direction = player.direction;
        let pos: Coord = player.position;

        let best_dirs: Vec<Turn> = self.shortest_dist_map[&pos]
            .clone()
            .into_iter()
            .filter_map(|d| dir.turn_to_dir(d))
            .collect();
        println!("best dirs {:?}", best_dirs);
        Some(if best_dirs.is_empty() {
            if turns.is_empty() {
                Turn::Straight
            } else {
                for t in turns {
                    let next_pos = pos + (dir + *t).to_coord();
                    if self.blockages.contains(&next_pos) {
                        continue;
                    }

                    return Some(*t);
                }

                turns[0]
            }
        } else {
            if tile.choice {
                *turns.choose(&mut rand::rng()).unwrap()
            } else {
                best_dirs[0]
            }
        })
    }
}

pub struct BestGearStrategy {}

impl GearStrategy for BestGearStrategy {
    fn gear_strategy(&mut self, _: &Player) -> ChangeGear {
        ChangeGear::Up
    }
}

pub struct ManualGearStrategy<'a> {
    event_pump: &'a mut EventPump,
}

impl<'a> GearStrategy for ManualGearStrategy<'a> {
    fn gear_strategy(&mut self, player: &Player) -> ChangeGear {
        let gear = player.gear;
        let mut gear_change = ChangeGear::Up;
        println!("Gear {:?}, Gear change: {:?}", gear, gear_change);

        'selection: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => panic!(),
                    Event::KeyDown {
                        keycode: Some(Keycode::S),
                        ..
                    } => {
                        gear_change = match gear_change {
                            ChangeGear::Up => ChangeGear::Stay,
                            _ => ChangeGear::Down,
                        };
                        println!("Gear {:?}, Gear change: {:?}", gear, gear_change);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::W),
                        ..
                    } => {
                        gear_change = match gear_change {
                            ChangeGear::Down => ChangeGear::Stay,
                            _ => ChangeGear::Up,
                        };
                        println!("Gear {:?}, Gear change: {:?}", gear, gear_change);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::SPACE),
                        ..
                    } => {
                        println!("Enter pressed?");
                        break 'selection;
                    }
                    _ => {}
                }
            }

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
        gear_change
    }
}

pub struct ManualStepStrategy<'a> {
    turn: Turn,
    event_pump: &'a mut EventPump,
}

impl<'a> StepStrategy for ManualStepStrategy<'a> {
    fn step_strategy(
        &mut self,
        _player: &Player,
        _turns: &Vec<Turn>,
        _tile: &Tile,
    ) -> Option<Turn> {
        // println!("Turn {:?}", self.turn);

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => panic!(),
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    self.turn = match self.turn {
                        Turn::Right => Turn::Straight,
                        _ => Turn::Left,
                    };
                    // println!("Turn {:?}", self.turn);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    self.turn = match self.turn {
                        Turn::Left => Turn::Straight,
                        _ => Turn::Right,
                    };
                    // println!("Turn {:?}", self.turn);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::SPACE),
                    ..
                } => {
                    // println!("Enter pressed?");
                    return Some(self.turn);
                }
                _ => {}
            }
        }

        return None;
    }
}

impl<'a> GameState<'a> {
    pub fn new(map: HexMap, player_strategies: Vec<(PlayerGearStrategy, PlayerStepStrategy)>) {
        let players = map.player_builder.clone().all_players();
        let player_index = 0;
        let rolling = true;
        let blockages = vec![];
        let shortest_dist_map = BTreeMap::new();

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let screen_width = 1000;

        let window = video_subsystem
            .window("rust-sdl2 demo", screen_width, screen_width)
            .position_centered()
            .build()
            .unwrap();
        // let _ = window.set_opacity(0.1); // Transparent window
        // window.set_bordered(false);

        // window.set_position(
        //     video::WindowPos::Positioned(0),
        //     video::WindowPos::Positioned(0),
        // );

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.window_mut().set_position(
            video::WindowPos::Positioned(10),
            video::WindowPos::Positioned(10),
        );

        canvas.clear();

        let scale: f64 = 42.0;
        let start = Coord::new(360, 700);

        let mut event_pump = sdl_context.event_pump().unwrap();

        let mut game_state = GameState {
            map,
            players,
            player_index,
            rolling,
            blockages,
            shortest_dist_map,
            start,
            scale,
            canvas: &mut canvas,
            choice_tile_selections: BTreeMap::new(),
            player_strategies,
        };

        game_state.display(&mut event_pump);
    }

    pub fn update_gameboard(&mut self) -> Vec<Coord> {
        let mut player_positions = vec![];

        let mut blockages = vec![];
        for p in &self.players {
            if player_positions.contains(&p.position)
                || self.map.tiles.contains_key(&p.position) && self.map.tiles[&p.position].chikane
            {
                blockages.push(p.position);
            }
            player_positions.push(p.position);
        }

        blockages
    }

    pub fn step_game(&mut self, event_pump: &mut EventPump) -> bool {
        if self.rolling {
            // p.roll_dice(BestGearStrategy {});
            {
                let (gear_strat, _) = self.player_strategies[self.player_index].clone();

                match gear_strat {
                    PlayerGearStrategy::Best => {
                        let strategy = BestGearStrategy {};
                        self.players[self.player_index].roll_dice(strategy);
                    }
                    PlayerGearStrategy::Manual => {
                        let strategy = ManualGearStrategy {
                            event_pump: event_pump,
                        };
                        self.players[self.player_index].roll_dice(strategy);
                    }
                };
            }
            self.rolling = false;
        } else {
            let old_dir = self.players[self.player_index].direction;

            if let Some(turns) = self.players[self.player_index].pre_step(
                &self.map.tiles,
                &self.blockages,
                &self.choice_tile_selections,
            ) {
                let (_, step_strat) = self.player_strategies[self.player_index].clone();

                match step_strat {
                    PlayerStepStrategy::Best => {
                        let mut strategy = BestStepStategy {
                            blockages: &self.blockages,
                            shortest_dist_map: &self.shortest_dist_map,
                        };
                        self.players[self.player_index].step(
                            &turns,
                            &self.map.tiles,
                            &mut strategy,
                        );
                    }
                    PlayerStepStrategy::Manual => {
                        let mut strategy = ManualStepStrategy {
                            turn: Turn::Straight,
                            event_pump: event_pump,
                        };

                        loop {
                            if self.players[self.player_index].step(
                                &turns,
                                &self.map.tiles,
                                &mut strategy,
                            ) {
                                break;
                            }

                            // Render only needed for manual strategy
                            let player_dir = self.players[self.player_index].direction;
                            self.players[self.player_index].direction =
                                self.players[self.player_index].direction + strategy.turn;

                            self.render();
                            self.canvas.present();

                            // Reset value
                            self.players[self.player_index].direction = player_dir;

                            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
                        }
                    }
                }
            }

            // Check if old tile is choice tile, and set direction
            let old_pos = self.players[self.player_index].old_position;
            if self.map.tiles.contains_key(&old_pos) && self.map.tiles[&old_pos].choice {
                if !self.choice_tile_selections.contains_key(&old_pos) {
                    self.choice_tile_selections.insert(old_pos, vec![]);
                }

                if self.choice_tile_selections[&old_pos].len()
                    < self.players[self.player_index].round
                {
                    let step_direction = old_dir;
                    if self.map.tiles[&old_pos]
                        .directions
                        .contains(&step_direction)
                    {
                        self.choice_tile_selections
                            .get_mut(&old_pos)
                            .unwrap()
                            .push(step_direction);
                    }
                }
            }

            // Passed midline / goal line
            if self.players[self.player_index].first_half {
                for (c, dirs) in &self.map.mid_line {
                    if old_pos == *c
                        && self.players[self.player_index].position != old_pos
                        && dirs.contains(&old_dir)
                    {
                        // Cross line
                        self.players[self.player_index].first_half = false;
                    }
                }
            } else {
                for (c, dirs) in &self.map.start_line {
                    if old_pos == *c
                        && self.players[self.player_index].position != old_pos
                        && dirs.contains(&old_dir)
                    {
                        // Cross line
                        self.players[self.player_index].round += 1;
                        self.players[self.player_index].first_half = true;
                    }
                }
            }

            // Go to next player
            if self.players[self.player_index].finished {
                if self
                    .map
                    .tiles
                    .contains_key(&self.players[self.player_index].position)
                    && self.map.tiles[&self.players[self.player_index].position].blue
                {
                    self.players[self.player_index].forced_gear_down = true;
                }

                self.rolling = true;
                self.player_index = (self.player_index + 1) % self.players.len();
                self.blockages = self.update_gameboard();

                println!("\nPlayer {}'s turn", self.player_index);

                return true;
            }
        }

        return false;
    }

    pub fn render(&mut self) {
        for p in &self.players {
            p.draw(self.canvas, self.start, self.scale);
        }

        self.map.draw(self.canvas, self.start, self.scale);

        for p in &self.players {
            p.draw(self.canvas, self.start, self.scale);
        }
    }

    pub fn display(&mut self, event_pump: &mut EventPump) {
        self.blockages = self.update_gameboard();
        self.shortest_dist_map = self.map.shortest_path();

        self.render();
        self.canvas.present();

        'game: loop {
            // let mut step_game = false;
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'game,
                    // Event::KeyDown {
                    //     keycode: Some(Keycode::Space),
                    //     ..
                    // } => {
                    //     step_game = true;
                    // }
                    _ => {}
                }
            }

            // if step_game {
            self.step_game(event_pump);
            println!("====");
            // }

            self.render();

            self.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
