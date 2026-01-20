use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::*;
use std::collections::{BTreeMap, BTreeSet};
use std::time::Duration;

#[derive(Copy, Clone, PartialOrd, Ord, Eq, PartialEq, Debug)]
pub enum Direction {
    UR,
    U,
    UL,
    DL,
    D,
    DR,
}

impl Direction {
    pub fn to_num(self) -> u8 {
        match self {
            Direction::UR => 0,
            Direction::U => 1,
            Direction::UL => 2,
            Direction::DL => 3,
            Direction::D => 4,
            Direction::DR => 5,
        }
    }

    pub fn from_num(n: u8) -> Self {
        match n {
            0 => Direction::UR,
            1 => Direction::U,
            2 => Direction::UL,
            3 => Direction::DL,
            4 => Direction::D,
            5 => Direction::DR,
            _ => panic!("Invalid number"),
        }
    }

    pub fn turn_to_dir(self, d: Direction) -> Option<Turn> {
        if self == d {
            Some(Turn::Straight)
        } else if (self.to_num() + 1) % 6 == d.to_num() {
            Some(Turn::Left)
        } else if self.to_num() == (d.to_num() + 1) % 6 {
            Some(Turn::Right)
        } else {
            None
        }
    }

    pub fn to_coord(self) -> Coord {
        match self {
            Direction::UR => Coord::tri(0, 0, -1),
            Direction::U => Coord::tri(0, -1, 0),
            Direction::UL => Coord::tri(-1, 0, 0),
            Direction::DL => Coord::tri(0, 0, 1),
            Direction::D => Coord::tri(0, 1, 0),
            Direction::DR => Coord::tri(1, 0, 0),
        }
    }

    pub fn angle(self) -> f64 {
        let a = self.to_num();
        (-30.0 + (a as f64) * -60.0) * std::f64::consts::PI / 180.0
        // 30.0 * std::f64::consts::PI / 180.0
    }
}

#[derive(Copy, Clone, PartialOrd, Ord, Eq, PartialEq, Debug)]
pub enum Turn {
    Straight,
    Left,
    Right,
}

impl Turn {
    pub fn to_num(self) -> i8 {
        match self {
            Turn::Left => 1,
            Turn::Straight => 0,
            Turn::Right => -1,
        }
    }
}

impl Add<Turn> for Direction {
    type Output = Self;
    fn add(self, other: Turn) -> Self {
        Direction::from_num((self.to_num() + (6 + other.to_num()) as u8) % 6)
    }
}

// 0 standard
// 1 start fields
// 2 blue
// 3 star
// 4 choice direction
// 5 forced dirs
#[derive(Clone)]
pub struct Tile {
    chikane: bool,
    blue: bool,
    start_field: bool,
    directions: Vec<Direction>,
    choice: bool,
    forced: BTreeMap<Coord, Direction>,
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            chikane: false,
            blue: false,
            start_field: false,
            directions: vec![],
            choice: false,
            forced: BTreeMap::new(),
        }
    }
}

impl Tile {
    pub fn new(directions: Vec<Direction>) -> Self {
        let mut t = Tile::default();
        t.directions = directions;
        t
    }

    pub fn start(self) -> Self {
        Tile {
            start_field: true,
            ..self
        }
    }

    pub fn blue(self) -> Self {
        Tile { blue: true, ..self }
    }

    pub fn chikane(self) -> Self {
        Tile {
            chikane: true,
            ..self
        }
    }

    pub fn choice(self) -> Self {
        Tile {
            choice: true,
            ..self
        }
    }

    pub fn forced(self, forced: BTreeMap<Coord, Direction>) -> Self {
        Tile { forced, ..self }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, c: Coord, scale: f64) {
        let intensity = if self.start_field { 120 } else { 255 };

        canvas.set_draw_color(if self.blue {
            Color::RGB(0, 0, intensity)
        } else {
            Color::RGB(intensity, intensity, 0)
        });
        fill_hexagon(canvas, c.x(), c.y(), scale);

        if self.chikane {
            canvas.set_draw_color(Color::RGB(255, 0, 0));
            fill_hexagon(canvas, c.x(), c.y(), scale / 2.0);
        }

        canvas.set_draw_color(if !self.forced.is_empty() {
            Color::RGB(70, 70, 70)
        } else if self.choice {
            Color::RGB(170, 170, 170)
        } else {
            Color::RGB(255, 255, 255)
        });
        for d in &self.directions {
            let angle = d.angle();
            let cx = c.x() + (scale / 2_f64.sqrt() * angle.cos()) as i32;
            let cy = c.y() + (scale / 2_f64.sqrt() * angle.sin()) as i32;

            for i in -1..=1 {
                let _ = canvas.draw_line((c.x() + i, c.y()), (cx + i, cy));
            }
            for i in -1..=1 {
                let _ = canvas.draw_line((c.x(), c.y() + i), (cx, cy + i));
            }
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        draw_hexagon(canvas, c.x(), c.y(), scale);
        draw_hexagon(canvas, c.x() + 1, c.y(), scale);
        draw_hexagon(canvas, c.x() + 1, c.y() + 1, scale);
    }
}

// q down right
// r down
// s down left
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Coord {
    pub q: i32,
    pub r: i32,
}

use std::ops::Add;

impl Add for Coord {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Coord {
            q: self.q + other.q,
            r: self.r + other.r,
        }
    }
}

use std::ops::Sub;

impl Sub for Coord {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Coord {
            q: self.q - other.q,
            r: self.r - other.r,
        }
    }
}

use std::ops::Mul;

impl Mul<i32> for Coord {
    type Output = Self;
    fn mul(self, other: i32) -> Self {
        Coord {
            q: self.q * other,
            r: self.r * other,
        }
    }
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Coord {
            q: x * 2 / 3,
            r: (((y as f64) * 3_f64.sqrt() - (x as f64)) / 3.0) as i32,
        }
    }

    pub fn axel(q: i32, r: i32) -> Self {
        Coord { q, r }
    }

    pub fn tri(q: i32, r: i32, s: i32) -> Self {
        Coord { q: q - s, r: r + s }
    }

    pub fn s(&self) -> i32 {
        -self.q - self.r
    }

    pub fn q(&self) -> i32 {
        self.q
    }

    pub fn r(&self) -> i32 {
        self.r
    }

    // q = <3/2, sqrt 3/2>
    // r = <  0, sqrt 3>
    pub fn x(&self) -> i32 {
        self.q * 3 / 2
    }

    pub fn y(&self) -> i32 {
        ((2 * self.r + self.q) as f64 * 3_f64.sqrt() / 2.0) as i32
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
        let cx = c.x() + (scale / 2_f64.sqrt() * angle.cos()) as i32;
        let cy = c.y() + (scale / 2_f64.sqrt() * angle.sin()) as i32;

        for i in -1..=1 {
            let _ = canvas.draw_line((c.x() + i, c.y()), (cx + i, cy));
        }
        for i in -1..=1 {
            let _ = canvas.draw_line((c.x(), c.y() + i), (cx, cy + i));
        }
    }

    pub fn step_possibilities(&self, tiles: &BTreeMap<Coord, Tile>) -> Vec<Turn> {
        if !tiles.contains_key(&self.position) {
            return vec![Turn::Straight];
        }

        let curr_tile = tiles[&self.position].clone();
        let dir = if curr_tile.forced.is_empty() {
            curr_tile.directions
        } else {
            vec![curr_tile.forced[&self.old_position]]
        };
        dir.into_iter()
            .filter_map(|x| self.direction.turn_to_dir(x))
            .collect()
    }

    pub fn step<F: Fn(&Vec<Turn>, Direction, Coord, &Tile) -> Turn>(
        &mut self,
        tiles: &BTreeMap<Coord, Tile>,
        strategy: F,
    ) {
        if self.outside_board {
            self.outside_board = false;
            self.position = self.old_position;
            self.direction = tiles[&self.position].directions[0];
        } else {
            self.old_position = self.position;
            self.position = self.position + self.direction.to_coord();
        }

        // Outside bord
        if !tiles.contains_key(&self.position) {
            self.outside_board = true;
            return;
        }

        let turns = self.step_possibilities(tiles);

        let curr_tile = tiles[&self.position].clone();
        let mut turn = strategy(&turns, self.direction, self.position, &curr_tile);

        if !curr_tile.forced.is_empty() && !turns.contains(&turn) {
            // Force valid
            turn = turns[0];
        };

        // if !turns.contains(&turn) {
        //     panic!("invalid turn");
        // }

        self.direction = self.direction + turn;
    }
}

#[derive(Clone)]
pub struct HexMap {
    pub tiles: BTreeMap<Coord, Tile>,
    pub start_line: Vec<(Coord, Vec<Direction>)>,
    pub mid_line: Vec<(Coord, Vec<Direction>)>,
    pub players: Vec<Player>,
}

use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn draw_hexagon_side(canvas: &mut Canvas<Window>, x: i32, y: i32, scale: f64, d: Direction) {
    let a = match d {
        Direction::UR => 0,
        Direction::U => 1,
        Direction::UL => 2,
        Direction::DL => 3,
        Direction::D => 4,
        Direction::DR => 5,
    };
    let points: Vec<Point> = (a..=(a + 1))
        .map(|a| (a as f64) * std::f64::consts::PI / 180.0 * -60.0)
        .map(|angle: f64| {
            (
                x + (scale * angle.cos()) as i32,
                y + (scale * angle.sin()) as i32,
            )
                .into()
        })
        .collect();
    let _ = canvas.draw_lines(&points[..]);
}

pub fn draw_hexagon(canvas: &mut Canvas<Window>, x: i32, y: i32, scale: f64) {
    let points: Vec<Point> = (0..=6)
        .map(|a| (a as f64) * std::f64::consts::PI / 180.0 * 60.0)
        .map(|angle: f64| {
            (
                x + (scale * angle.cos()) as i32,
                y + (scale * angle.sin()) as i32,
            )
                .into()
        })
        .collect();
    let _ = canvas.draw_lines(&points[..]);
}

pub fn fill_hexagon(canvas: &mut Canvas<Window>, x: i32, y: i32, scale: f64) {
    for i in 0..6 * (scale as i32) {
        draw_hexagon(canvas, x + i % 2, y, scale - (i as f64) / 6.0);
    }
    // let (xs,ys) : (Vec<i32>, Vec<i32>) =
    //     (0..=6)
    //     .map(|a| (a as f64) * std::f64::consts::PI / 180.0 * 60.0)
    //     .map(|angle: f64|
    //          (x + (scale * angle.cos()) as i32,
    //           y + (scale * angle.sin()) as i32)// .into()
    //     ).unzip();
    // let _ = <Canvas<Window> as DrawRenderer>::filled_polygon(xs, ys, Color::RGB(255, 255, 0));
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
            let ((c, d), _, _) = stk[index];
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

    pub fn display(&mut self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let screen_width = 1000;

        let mut window = video_subsystem
            .window("rust-sdl2 demo", screen_width, screen_width)
            .position_centered()
            .build()
            .unwrap();
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
        let mut event_pump = sdl_context.event_pump().unwrap();

        let shortest_dist_map: BTreeMap<Coord, Vec<Direction>> = self.shortest_path();

        'game: loop {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            let start = Coord::new(360, 700);
            for (c, t) in &self.tiles {
                let c = start + *c * (scale as i32);
                t.draw(&mut canvas, c, scale);
            }

            for (c, dirs) in &self.start_line {
                let c = start + *c * (scale as i32);
                for d in dirs {
                    canvas.set_draw_color(Color::RGB(255, 0, 255));
                    let _ = draw_hexagon_side(&mut canvas, c.x(), c.y(), scale, *d);
                    let _ = draw_hexagon_side(&mut canvas, c.x() + 1, c.y(), scale, *d);
                    let _ = draw_hexagon_side(&mut canvas, c.x() + 1, c.y() + 1, scale, *d);
                }
            }

            for (c, dirs) in &self.mid_line {
                let c = start + *c * (scale as i32);
                for d in dirs {
                    canvas.set_draw_color(Color::RGB(255, 0, 255));
                    for i in -1..=1 {
                        for j in -1..=1 {
                            let _ = draw_hexagon_side(&mut canvas, c.x() + i, c.y() + j, scale, *d);
                        }
                    }
                }
            }

            for p in &self.players {
                p.draw(&mut canvas, start, scale);
            }

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'game,
                    Event::KeyDown {
                        keycode: Some(Keycode::Space),
                        ..
                    } => {
                        for p in &mut self.players {
                            p.step(&self.tiles, |turns, dir, pos, tile| {
                                let best_dirs: Vec<Turn> = shortest_dist_map[&pos]
                                    .clone()
                                    .into_iter()
                                    .filter_map(|d| dir.turn_to_dir(d))
                                    .collect();
                                println!("best dirs {:?}", best_dirs);
                                if best_dirs.is_empty() {
                                    if turns.is_empty() {
                                        Turn::Straight
                                    } else {
                                        turns[0]
                                    }
                                } else {
                                    best_dirs[0]
                                }
                            })
                        }
                    }
                    _ => {}
                }
            }

            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
