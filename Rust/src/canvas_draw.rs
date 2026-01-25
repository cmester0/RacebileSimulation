use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::Direction;

pub fn draw_hexagon_side_checkerboard(
    canvas: &mut Canvas<Window>,
    x: i32,
    y: i32,
    scale: f64,
    d: Direction,
    color1: Color,
    color2: Color,
) {
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

    let amount = 3;
    canvas.set_draw_color(color1);
    for i in 0..amount {
        let part: Vec<Point> = (i * 2..=i * 2 + 1)
            .map(|x| (points[1] - points[0]) * x / (amount*2) + points[0])
            .collect();
        let _ = canvas.draw_lines(&part[..]);
    }
    canvas.set_draw_color(color2);
    for i in 0..amount {
        let part: Vec<Point> = (i * 2+1..=(i+1) * 2)
            .map(|x| (points[1] - points[0]) * x / (amount*2) + points[0])
            .collect();
        let _ = canvas.draw_lines(&part[..]);
    }
}

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
