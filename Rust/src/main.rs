pub mod game;
pub mod map;

use crate::map::*;
use crate::Direction::*;

use sdl2::pixels::Color;
use std::collections::BTreeMap;

fn main() {
    let mut rtfm = HexMap {
        tiles: BTreeMap::from([
            (Coord::tri(0, 0, 0), Tile::new(vec![DR]).start()),
            (Coord::tri(0, 0, 1), Tile::new(vec![UR, DR]).start()),
            (Coord::tri(1, 0, 0), Tile::new(vec![D, DR]).start()),
            (Coord::tri(1, 0, 1), Tile::new(vec![UR, DR]).start()),
            (Coord::tri(2, 0, 0), Tile::new(vec![D, DR]).start()),
            (Coord::tri(2, 0, 1), Tile::new(vec![UR, DR]).start()),
            (Coord::tri(3, 0, 0), Tile::new(vec![D, DR]).start()),
            (Coord::tri(3, 0, 1), Tile::new(vec![UR, DR]).start()),
            (Coord::tri(4, 0, 0), Tile::new(vec![DR])),
            (Coord::tri(4, 0, 1), Tile::new(vec![UR])),
            (Coord::tri(5, 0, 0), Tile::new(vec![UR]).blue()),
            (Coord::tri(5, 0, -1), Tile::new(vec![UR])),
            (Coord::tri(5, 0, -2), Tile::new(vec![UR])),
            (Coord::tri(5, 0, -3), Tile::new(vec![U])),
            (Coord::tri(5, -1, -3), Tile::new(vec![U])),
            (Coord::tri(5, -2, -3), Tile::new(vec![UL, UR]).choice()),
            // Left
            (Coord::tri(4, -2, -3), Tile::new(vec![U])),
            (Coord::tri(4, -3, -3), Tile::new(vec![UR])),
            // Right
            (Coord::tri(5, -2, -4), Tile::new(vec![U])),
            (Coord::tri(5, -3, -4), Tile::new(vec![UL])),
            (Coord::tri(4, -3, -4), Tile::new(vec![U]).chikane()),
            (Coord::tri(4, -4, -4), Tile::new(vec![U])),
            (Coord::tri(4, -5, -4), Tile::new(vec![U])),
            (Coord::tri(4, -6, -4), Tile::new(vec![UL])),
            (Coord::tri(3, -6, -4), Tile::new(vec![DL]).chikane()),
            (Coord::tri(3, -6, -3), Tile::new(vec![UL, DL]).blue()),
            // Up
            (Coord::tri(2, -6, -3), Tile::new(vec![DL]).blue()),
            // Down
            (Coord::tri(3, -6, -2), Tile::new(vec![UL]).blue()),
            (Coord::tri(2, -6, -2), Tile::new(vec![DL])),
            (Coord::tri(2, -6, -1), Tile::new(vec![D, DL])),
            // Blue edge
            (Coord::tri(2, -5, -1), Tile::new(vec![DL]).blue()),
            (Coord::tri(2, -5, 0), Tile::new(vec![UL]).blue()),
            (Coord::tri(2, -6, 0), Tile::new(vec![DL])),
            (Coord::tri(2, -6, 1), Tile::new(vec![UL]).chikane()),
            (Coord::tri(1, -6, 1), Tile::new(vec![UL])),
            (Coord::tri(0, -6, 1), Tile::new(vec![UL])),
            // Loop
            (
                Coord::tri(-1, -6, 1),
                Tile::new(vec![UL, DL]).forced(BTreeMap::from([
                    (Coord::tri(0, -6, 1), UL),
                    (Coord::tri(-2, -6, 0), DL),
                ])),
            ),
            (Coord::tri(-2, -6, 1), Tile::new(vec![UL]).chikane()),
            (Coord::tri(-3, -6, 1), Tile::new(vec![U])),
            (Coord::tri(-3, -7, 1), Tile::new(vec![UR]).blue()),
            (Coord::tri(-3, -7, 0), Tile::new(vec![DR]).blue()),
            (Coord::tri(-2, -7, 0), Tile::new(vec![D]).blue()),
            (
                Coord::tri(-2, -6, 0),
                Tile::new(vec![D]).forced(BTreeMap::from([(Coord::tri(-2, -7, 0), D)])),
            ),
            (Coord::tri(-2, -5, 1), Tile::new(vec![D]).chikane()),
            (Coord::tri(-2, -4, 1), Tile::new(vec![DL])),
            (Coord::tri(-2, -4, 2), Tile::new(vec![D])),
            (Coord::tri(-2, -3, 2), Tile::new(vec![D, DL])),
            // Blue
            (Coord::tri(-2, -3, 3), Tile::new(vec![D]).blue()),
            (Coord::tri(-2, -2, 3), Tile::new(vec![DR]).blue()),
            (Coord::tri(-2, -2, 2), Tile::new(vec![D])),
            (Coord::tri(-2, -1, 2), Tile::new(vec![DR]).chikane()),
            (Coord::tri(-1, -1, 2), Tile::new(vec![DR])),
            (Coord::tri(0, -1, 2), Tile::new(vec![DR])),
        ]),
        start_line: vec![
            (Coord::tri(3, 0, 0), vec![DR]),
            (Coord::tri(4, 0, 1), vec![UR]),
        ],
        mid_line: vec![
            (Coord::tri(2, -5, 0), vec![UL]),
            (Coord::tri(2, -6, 0), vec![DL]),
        ],
        players: vec![
            Player {
                position: Coord::tri(3, 0, 0),
                old_position: Coord::tri(3, 0, 0),
                direction: DR,
                color: Color::RGB(255, 0, 0),
                radius: 0.2,
                outside_board: false,
            },
            Player {
                position: Coord::tri(3, 0, 1),
                old_position: Coord::tri(3, 0, 1),
                direction: DR,
                color: Color::RGB(0, 0, 255),
                radius: 0.3,
                outside_board: false,
            },
            Player {
                position: Coord::tri(2, 0, 0),
                old_position: Coord::tri(2, 0, 0),
                direction: DR,
                color: Color::RGB(0, 255, 0),
                radius: 0.4,
                outside_board: false,
            },
            Player {
                position: Coord::tri(2, 0, 1),
                old_position: Coord::tri(2, 0, 1),
                direction: DR,
                color: Color::RGB(255, 255, 0),
                radius: 0.5,
                outside_board: false,
            },
            Player {
                position: Coord::tri(1, 0, 0),
                old_position: Coord::tri(1, 0, 0),
                direction: DR,
                color: Color::RGB(255, 0, 255),
                radius: 0.6,
                outside_board: false,
            },
            Player {
                position: Coord::tri(1, 0, 1),
                old_position: Coord::tri(1, 0, 1),
                direction: DR,
                color: Color::RGB(0, 255, 255),
                radius: 0.7,
                outside_board: false,
            },
            Player {
                position: Coord::tri(0, 0, 0),
                old_position: Coord::tri(0, 0, 0),
                direction: DR,
                color: Color::RGB(255, 100, 100),
                radius: 0.8,
                outside_board: false,
            },
            Player {
                position: Coord::tri(0, 0, 1),
                old_position: Coord::tri(0, 0, 1),
                direction: DR,
                color: Color::RGB(100, 100, 255),
                radius: 0.9,
                outside_board: false,
            },
        ],
    };

    rtfm.display();

    println!("Hello, world!");
}
