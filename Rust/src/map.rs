use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::*;
use std::time::Duration;
use std::collections::BTreeMap;

#[derive(Copy,Clone)]
pub enum Direction {
    UL,
    U,
    UR,
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

    pub fn angle(self) -> f64 {
        let a = self.to_num();
        (-30.0 + (a as f64) * -60.0) * std::f64::consts::PI / 180.0
        // 30.0 * std::f64::consts::PI / 180.0
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
    chikane : bool,
    blue : bool,
    start_field : bool,
    directions : Vec<Direction>,
    choice: bool,
    forced: bool,
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            chikane: false,
            blue: false,
            start_field: false,
            directions: vec![],
            choice: false,
            forced: false,
        }
    }
}

impl Tile {
    pub fn new(directions : Vec<Direction>) -> Self {
        let mut t = Tile::default();
        t.directions = directions;
        t
    }

    pub fn start(self) -> Self {
        Tile { start_field: true, ..self }
    }

    pub fn blue(self) -> Self {
        Tile { blue: true, ..self }
    }

    pub fn chikane(self) -> Self {
        Tile { chikane: true, ..self }
    }

    pub fn choice(self) -> Self {
        Tile { choice: true, ..self }
    }

    pub fn forced(self) -> Self {
        Tile { forced: true, ..self }
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

        canvas.set_draw_color(if self.forced { Color::RGB(70, 70, 70) } else if self.choice { Color::RGB(170, 170, 170) } else { Color::RGB(255, 255, 255) });
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
        draw_hexagon(canvas, c.x()+1, c.y(), scale);
        draw_hexagon(canvas, c.x()+1, c.y()+1, scale);
    }
}

// q down right
// r down
// s down left
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Coord {
    pub q : i32,
    pub r : i32,
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
        Coord { q:q-s, r:r+s }
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
    pub color : Color,
    pub radius : f64,

    pub position : Coord,
    pub direction: Direction,
}

impl Player {
    pub fn draw(&self, canvas: &mut Canvas<Window>, start: Coord, scale: f64) {
        let c = start + self.position * (scale as i32);
        canvas.set_draw_color(self.color);

        let _ = draw_hexagon(canvas, c.x(), c.y(), scale * self.radius);
        let _ = draw_hexagon(canvas, c.x()+1, c.y(), scale * self.radius);
        let _ = draw_hexagon(canvas, c.x()+1, c.y()+1, scale * self.radius);

        let _ = draw_hexagon(canvas, c.x(), c.y(), 0.5 * scale * self.radius);
        let _ = draw_hexagon(canvas, c.x()+1, c.y(), 0.5 * scale * self.radius);
        let _ = draw_hexagon(canvas, c.x()+1, c.y()+1, 0.5 * scale * self.radius);

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

    pub fn step(&mut self, tiles: &BTreeMap<Coord,Tile>) {
        
    }
}

#[derive(Clone)]
pub struct HexMap {
    pub tiles : BTreeMap<Coord,Tile>,
    pub start_line : Vec<(Coord, Vec<Direction>)>,
    pub mid_line : Vec<(Coord, Vec<Direction>)>,
    pub players : Vec<Player>,
}

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Point;

pub fn draw_hexagon_side(canvas : &mut Canvas<Window>, x : i32, y : i32, scale : f64, d: Direction) {
    let a = match d {
        Direction::UR => 0,
        Direction::U => 1,
        Direction::UL => 2,
        Direction::DL => 3,
        Direction::D => 4,
        Direction::DR => 5,
    };
    let points : Vec<Point> =
        (a..=(a+1))
        .map(|a| (a as f64) * std::f64::consts::PI / 180.0 * -60.0)
        .map(|angle: f64|
             (x + (scale * angle.cos()) as i32,
              y + (scale * angle.sin()) as i32).into()
        ).collect();
    let _ = canvas.draw_lines(&points[..]);
}

pub fn draw_hexagon(canvas : &mut Canvas<Window>, x : i32, y : i32, scale : f64) {
    let points : Vec<Point> =
        (0..=6)
        .map(|a| (a as f64) * std::f64::consts::PI / 180.0 * 60.0)
        .map(|angle: f64|
             (x + (scale * angle.cos()) as i32,
              y + (scale * angle.sin()) as i32).into()
        ).collect();
    let _ = canvas.draw_lines(&points[..]);
}

pub fn fill_hexagon(canvas : &mut Canvas<Window>, x : i32, y : i32, scale : f64) {
    for i in 0..6*(scale as i32) {
        draw_hexagon(canvas, x+i%2, y, scale-(i as f64)/6.0);
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
    pub fn display(self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let screen_width = 1000;

        let mut window = video_subsystem
            .window("rust-sdl2 demo", screen_width, screen_width)
            .position_centered()
            .build()
            .unwrap();
        window.set_bordered(false);

        // window.set_position(
        //     video::WindowPos::Positioned(0),
        //     video::WindowPos::Positioned(0),
        // );

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.window_mut().set_position(
            video::WindowPos::Positioned(0),
            video::WindowPos::Positioned(0),
        );

        canvas.clear();

        // let scale : f64 = 36.0;
        let scale : f64 = 42.0;
        // let players = 
        //     self.players.iter().enumerate().map(|i,player| {
        //         let c = start + player.position * (scale as i32);
        //         canvas.set_draw_color(player_colors[i%player_colors.len()]);

        //         let _ = draw_hexagon(&mut canvas, c.x(), c.y(), scale * ((i as f64)+3.0) / (self.players.len() as f64+4.0));
        //         let _ = draw_hexagon(&mut canvas, c.x()+1, c.y(), scale * ((i as f64)+3.0) / (self.players.len() as f64+4.0));
        //         let _ = draw_hexagon(&mut canvas, c.x()+1, c.y()+1, scale * ((i as f64)+3.0) / (self.players.len() as f64+4.0));

        //         let _ = draw_hexagon(&mut canvas, c.x(), c.y(), 0.5 * scale * ((i as f64)+3.0) / (self.players.len() as f64+4.0));
        //         let _ = draw_hexagon(&mut canvas, c.x()+1, c.y(), 0.5 * scale * ((i as f64)+3.0) / (self.players.len() as f64+4.0));
        //         let _ = draw_hexagon(&mut canvas, c.x()+1, c.y()+1, 0.5 * scale * ((i as f64)+3.0) / (self.players.len() as f64+4.0));

        //         let angle = player.direction.angle();
        //         let cx = c.x() + (scale / 2_f64.sqrt() * angle.cos()) as i32;
        //         let cy = c.y() + (scale / 2_f64.sqrt() * angle.sin()) as i32;

        //         for i in -1..=1 {
        //             let _ = canvas.draw_line((c.x() + i, c.y()), (cx + i, cy));
        //         }
        //         for i in -1..=1 {
        //             let _ = canvas.draw_line((c.x(), c.y() + i), (cx, cy + i));
        //         }

        //     }

        // let c = start + player.position * (scale as i32);
        // canvas.set_draw_color(player[i%player_colors.len()]);

        loop {
            let start = Coord::new(360,700);
            for (c,t) in &self.tiles {
                let c = start + *c * (scale as i32);
                t.draw(&mut canvas, c, scale);
            }

            for (c, dirs) in &self.start_line {
                let c = start + *c * (scale as i32);
                for d in dirs {
                    canvas.set_draw_color(Color::RGB(255, 0, 255));
                    let _ = draw_hexagon_side(&mut canvas, c.x(), c.y(), scale, *d);
                    let _ = draw_hexagon_side(&mut canvas, c.x()+1, c.y(), scale, *d);
                    let _ = draw_hexagon_side(&mut canvas, c.x()+1, c.y()+1, scale, *d);
                }
            }

            for (c, dirs) in &self.mid_line {
                let c = start + *c * (scale as i32);
                for d in dirs {
                    canvas.set_draw_color(Color::RGB(255, 0, 255));
                    let _ = draw_hexagon_side(&mut canvas, c.x(), c.y(), scale, *d);
                    let _ = draw_hexagon_side(&mut canvas, c.x()+1, c.y(), scale, *d);
                    let _ = draw_hexagon_side(&mut canvas, c.x()+1, c.y()+1, scale, *d);
                }
            }

            for p in &self.players {
                p.draw(&mut canvas, start, scale);
            }

            canvas.present();
            // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
