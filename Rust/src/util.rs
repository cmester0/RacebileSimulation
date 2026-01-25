use crate::canvas_draw::*;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::cmp::{max, min};
use std::collections::BTreeMap;
use std::ops::{Add, Mul, Sub};

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

    pub fn from_coord(c: Coord) -> Self {
        match c {
            _ if c == Coord::tri(0, 0, -1) => Direction::UR,
            _ if c == Coord::tri(0, -1, 0) => Direction::U,
            _ if c == Coord::tri(-1, 0, 0) => Direction::UL,
            _ if c == Coord::tri(0, 0, 1) => Direction::DL,
            _ if c == Coord::tri(0, 1, 0) => Direction::D,
            _ if c == Coord::tri(1, 0, 0) => Direction::DR,
            _ => unimplemented!("{:?}", c) // TODO: general direction? Or reject?
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

#[derive(Copy, Clone, PartialOrd, Ord, Eq, PartialEq, Debug)]
pub enum ChangeGear {
    Down,
    Stay,
    Up,
}

impl ChangeGear {
    pub fn update_gear(self, gear: u8) -> u8 {
        match self {
            ChangeGear::Down => max(gear - 1, 1),
            ChangeGear::Stay => gear,
            ChangeGear::Up => min(gear + 1, 3),
        }
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
    pub chikane: bool,
    pub blue: bool,
    pub start_field: bool,
    pub directions: Vec<Direction>,
    pub choice: bool,
    pub forced: BTreeMap<Coord, Direction>,
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
        let intensity = if self.start_field { 120 } else { 200 };

        canvas.set_draw_color(if self.blue {
            Color::RGB(0, 0, intensity)
        } else {
            Color::RGB(intensity, intensity, 0)
        });
        fill_hexagon(canvas, c.x(), c.y(), scale);

        if self.chikane {
            canvas.set_draw_color(Color::RGB(200, 0, 0));
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
        for i in -1..=1 {
            draw_hexagon(canvas, c.x() + i, c.y(), scale);
        }
        for i in -1..=1 {
            draw_hexagon(canvas, c.x(), c.y() + i, scale);
        }
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

impl Add for Coord {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Coord {
            q: self.q + other.q,
            r: self.r + other.r,
        }
    }
}

impl Sub for Coord {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Coord {
            q: self.q - other.q,
            r: self.r - other.r,
        }
    }
}

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
