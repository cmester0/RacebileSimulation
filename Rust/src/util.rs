use crate::canvas_draw::*;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::cmp::{max, min};
use std::collections::BTreeMap;
use std::f64::consts::PI;
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
            _ => unimplemented!("{:?}", c), // TODO: general direction? Or reject?
        }
    }

    pub fn angle(self) -> f64 {
        let a = self.to_num();
        (-30.0 + (a as f64) * -60.0) * PI / 180.0
        // 30.0 * PI / 180.0
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
    pub rotate: bool,
    pub oneway: bool,
    pub blockage: Vec<Direction>,
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
            rotate: false,
            oneway: false,
            blockage: vec![],
            forced: BTreeMap::new(),
        }
    }
}

fn line_intersection(
    p1: (f64, f64),
    p2: (f64, f64),
    p3: (f64, f64),
    p4: (f64, f64),
) -> Option<(f64, f64)> {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let (x3, y3) = p3;
    let (x4, y4) = p4;

    // Calculate the determinants
    let denominator = (x2 - x1) * (y4 - y3) - (y2 - y1) * (x4 - x3);

    if denominator.abs() < f64::EPSILON {
        // Lines are parallel
        return None;
    }

    // Calculate the parameters t and u
    let t = ((x3 - x1) * (y4 - y3) - (y3 - y1) * (x4 - x3)) / denominator;
    let u = -((x2 - x1) * (y3 - y1) - (y2 - y1) * (x3 - x1)) / denominator;

    // Check if the intersection is within the segments
    if (t >= 0.0 && t <= 1.0) && (u >= 0.0 && u <= 1.0) {
        // Calculate the intersection point
        Some((x1 + t * (x2 - x1), y1 + t * (y2 - y1)))
    } else {
        None // Intersection is outside the segments
    }
}

// fn line_intersection(p1: (f64, f64), p2: (f64, f64), p3: (f64, f64), p4: (f64, f64)) -> Option<(f64, f64)> {
//     let a1 = p2.1 - p1.1;
//     let b1 = p1.0 - p2.0;
//     let c1 = a1 * p1.0 + b1 * p1.1;

//     let a2 = p4.1 - p3.1;
//     let b2 = p3.0 - p4.0;
//     let c2 = a2 * p3.0 + b2 * p3.1;

//     let det = a1 * b2 - a2 * b1;

//     if det.abs() < f64::EPSILON {
//         return None; // Lines are parallel
//     }

//     let intersect_x = (b2 * c1 - b1 * c2) / det;
//     let intersect_y = (a1 * c2 - a2 * c1) / det;

//     Some((intersect_x, intersect_y))

//     // Calculate the parameters t and u
//     let t = ((c.0 - a.0) * (d.1 - c.1) - (c.1 - a.1) * (d.0 - c.0)) / denominator;
//     let u = -((b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)) / denominator;

//     // Check if intersection is within the segments
//     if (t >= 0.0 && t <= 1.0) && (u >= 0.0 && u <= 1.0) {
//         // Calculate the intersection point
//         Some((
//             x: a.0 + t * (b.0 - a.0),
//             y: a.1 + t * (b.1 - a.1),
//         ))
//     } else {
//         None // Intersection is outside the segments
//     }
// }

fn ray_hex_dist(c: Coord, scale: f64, px: f64, py: f64, theta: f64) -> f64 {
    let points: Vec<(f64, f64)> = (0..6)
        .map(|a| (a as f64) * PI / 180.0 * 60.0)
        .map(|angle: f64| {
            (
                c.x() as f64 + (scale * angle.cos()),
                c.y() as f64 + (scale * angle.sin()),
            )
                .into()
        })
        .collect();

    let d = (0..6)
        .filter_map(|i| {
            line_intersection(
                (px, py),
                (
                    px + (2.0 * scale * theta.cos()),
                    py + (2.0 * scale * theta.sin()),
                ),
                points[i],
                points[(i + 1) % 6],
            )
        })
        .map(|(x, y)| (x - px, y - py))
        .map(|(x, y)| (x * x + y * y).sqrt())
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();

    d
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

    pub fn rotate(self) -> Self {
        Tile {
            rotate: true,
            ..self
        }
    }

    pub fn oneway(self) -> Self {
        Tile {
            oneway: true,
            ..self
        }
    }

    pub fn blockage(self, blockage: Vec<Direction>) -> Self {
        Tile {
            blockage,
            ..self
        }
    }

    pub fn forced(self, forced: BTreeMap<Coord, Direction>) -> Self {
        Tile { forced, ..self }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, c: Coord, scale: f64) {
        let intensity = if self.start_field { 120 } else { 200 };
        let fill_color = if self.blue {
            Color::RGB(0, 0, intensity)
        } else {
            Color::RGB(intensity, intensity, 0)
        };

        canvas.set_draw_color(fill_color);
        if self.oneway {
            let divisions = 8;
            for k in 0..divisions {
                let points: Vec<(f64, f64)> = (0..6)
                    .map(|a| (a as f64) * std::f64::consts::PI / 180.0 * 60.0)
                    .map(|angle: f64| {
                        (
                            c.x() as f64 + (scale * angle.cos()),
                            c.y() as f64 + (scale * angle.sin()),
                        )
                            .into()
                    })
                    .collect();

                let k_coord = {
                    let hx_t = points[4].0;
                    let hx_f = points[1].0;

                    let hy_t = points[4].1;
                    let hy_f = points[1].1;

                    (
                        (hx_t - hx_f) * (k as f64 + 0.5) / (divisions as f64) + hx_f,
                        (hy_t - hy_f) * (k as f64 + 0.5) / (divisions as f64) + hy_f,
                    )
                };

                let check_angle = -PI / 5.0;
                let d = ray_hex_dist(c, scale, k_coord.0, k_coord.1, check_angle);

                let cx_f = k_coord.0 as i32 + (d * check_angle.cos()) as i32;
                let cy_f = k_coord.1 as i32 + (d * check_angle.sin()) as i32;

                let check_angle = check_angle + PI;
                let d = ray_hex_dist(c, scale, k_coord.0, k_coord.1, check_angle);

                let cx_t = k_coord.0 as i32 + (d * check_angle.cos()) as i32;
                let cy_t = k_coord.1 as i32 + (d * check_angle.sin()) as i32;

                for i in -1..=1 {
                    for j in -1..=1 {
                        let _ = canvas.draw_line((cx_f + i, cy_f + j), (cx_t + i, cy_t + j));
                    }
                }
            }
        } else {
            fill_hexagon(canvas, c.x(), c.y(), scale);
        }

        if self.chikane {
            canvas.set_draw_color(Color::RGB(200, 0, 0));
            fill_hexagon(canvas, c.x(), c.y(), scale / 2.0);
        }

        if self.rotate {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            for i in -1..=1 {
                for j in -1..=1 {
                    draw_hexagon(canvas, c.x() + i, c.y() + j, scale * 4.0 / 6.0);
                }
            }

            // Arrow at the right side
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            let cx_f = c.x() + (scale * 4.0 / 6.0 * 0_f64.cos()) as i32;
            let cy_f = c.y() + (scale * 4.0 / 6.0 * 0_f64.sin()) as i32;

            let cx_t = cx_f + (scale * 1.5 / 6.0 * (PI * 2.0 / 3.0 - PI * 1.0 / 5.0).cos()) as i32;
            let cy_t = cy_f + (scale * 1.5 / 6.0 * (PI * 2.0 / 3.0 - PI * 1.0 / 5.0).sin()) as i32;

            for i in -1..=1 {
                for j in -1..=1 {
                    let _ = canvas.draw_line((cx_f + i, cy_f + j), (cx_t + i, cy_t + j));
                }
            }

            let cx_t = cx_f + (scale * 1.5 / 6.0 * (PI * 2.0 / 3.0 + PI * 1.0 / 5.0).cos()) as i32;
            let cy_t = cy_f + (scale * 1.5 / 6.0 * (PI * 2.0 / 3.0 + PI * 1.0 / 5.0).sin()) as i32;

            for i in -1..=1 {
                for j in -1..=1 {
                    let _ = canvas.draw_line((cx_f + i, cy_f + j), (cx_t + i, cy_t + j));
                }
            }

            // Arrow at the left side
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            let cx_f = c.x() + (scale * 4.0 / 6.0 * PI.cos()) as i32;
            let cy_f = c.y() + (scale * 4.0 / 6.0 * PI.sin()) as i32;

            let cx_t = cx_f + (scale * 1.5 / 6.0 * (-PI * 1.0 / 3.0 - PI * 1.0 / 5.0).cos()) as i32;
            let cy_t = cy_f + (scale * 1.5 / 6.0 * (-PI * 1.0 / 3.0 - PI * 1.0 / 5.0).sin()) as i32;

            for i in -1..=1 {
                for j in -1..=1 {
                    let _ = canvas.draw_line((cx_f + i, cy_f + j), (cx_t + i, cy_t + j));
                }
            }

            let cx_t = cx_f + (scale * 1.5 / 6.0 * (-PI * 1.0 / 3.0 + PI * 1.0 / 5.0).cos()) as i32;
            let cy_t = cy_f + (scale * 1.5 / 6.0 * (-PI * 1.0 / 3.0 + PI * 1.0 / 5.0).sin()) as i32;

            for i in -1..=1 {
                for j in -1..=1 {
                    let _ = canvas.draw_line((cx_f + i, cy_f + j), (cx_t + i, cy_t + j));
                }
            }
        }

        for d in &self.directions {
            let mut draw_arrows = false;
            canvas.set_draw_color(if !self.forced.is_empty() && self.forced.iter().any(|x| x.1 == d) {
                draw_arrows = true;
                Color::RGB(70, 70, 70)
            } else if self.choice {
                draw_arrows = true;
                Color::RGB(170, 170, 170)
            } else {
                Color::RGB(255, 255, 255)
            });

            if draw_arrows {
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
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for i in -1..=1 {
            draw_hexagon(canvas, c.x() + i, c.y(), scale);
        }
        for i in -1..=1 {
            draw_hexagon(canvas, c.x(), c.y() + i, scale);
        }

        for d in &self.blockage {
            canvas.set_draw_color(Color::RGB(150, 0, 0));

            for i in -1..=1 {
                for j in -1..=1 {
                    draw_hexagon_side(canvas, c.x()+i, c.y()+j, scale * 0.8, *d);
                }
            }
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
