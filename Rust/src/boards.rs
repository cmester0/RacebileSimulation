use crate::map::*;
use crate::util::{*, Direction::*};
use std::collections::BTreeMap;
use sdl2::pixels::Color;

pub fn rtfm() -> HexMap {
    HexMap {
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
            (Coord::tri(4, -3, -4), Tile::new(vec![U]).chikane().forced(BTreeMap::from([
                (Coord::tri(5,-3,-4), U),
                (Coord::tri(4,-3,-3), U)
            ]))),
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
            (Coord::tri(-2, -6, 1), Tile::new(vec![UL]).chikane().forced(BTreeMap::from([(Coord::tri(-1,-6,1), UL)]))),
            (Coord::tri(-3, -6, 1), Tile::new(vec![U])),
            (Coord::tri(-3, -7, 1), Tile::new(vec![UR]).blue()),
            (Coord::tri(-3, -7, 0), Tile::new(vec![DR]).blue()),
            (Coord::tri(-2, -7, 0), Tile::new(vec![D]).blue()),
            (
                Coord::tri(-2, -6, 0),
                Tile::new(vec![D]).forced(BTreeMap::from([(Coord::tri(-2, -7, 0), D)])),
            ),
            (Coord::tri(-2, -5, 1), Tile::new(vec![D]).chikane().forced(BTreeMap::from([(Coord::tri(-1,-6,1), D)]))),
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
        player_builder: PlayerBuilder::new(
            8,
            vec![
                Color::RGB(255, 0, 0),
                Color::RGB(0, 0, 255),
                Color::RGB(0, 255, 0),
                Color::RGB(255, 255, 0),
                Color::RGB(255, 0, 255),
                Color::RGB(0, 255, 255),
                Color::RGB(255, 100, 100),
                Color::RGB(100, 100, 255),
            ],
            vec![
                (Coord::tri(3, 0, 0), DR),
                (Coord::tri(3, 0, 1), DR),
                (Coord::tri(2, 0, 0), DR),
                (Coord::tri(2, 0, 1), DR),
                (Coord::tri(1, 0, 0), DR),
                (Coord::tri(1, 0, 1), DR),
                (Coord::tri(0, 0, 0), DR),
                (Coord::tri(0, 0, 1), DR),
            ],
        ),
    }
}

pub fn random_direction() -> HexMap {
    HexMap {
        tiles: BTreeMap::from([
            // Start
            (Coord::tri(-1,-3,0), Tile::new(vec![D,DR]).start()),
            (Coord::tri(0,-3,0), Tile::new(vec![D,DL]).start()),
            (Coord::tri(-1,-2,0), Tile::new(vec![D,DR]).start()),
            (Coord::tri(0,-2,0), Tile::new(vec![D,DL]).start()),
            (Coord::tri(-1,-1,0), Tile::new(vec![D,DR]).start()),
            (Coord::tri(0,-1,0), Tile::new(vec![D,DL]).start()),
            (Coord::tri(-1,0,0), Tile::new(vec![D,DR]).start()),
            (Coord::tri(0,0,0), Tile::new(vec![D,DL]).start()),

            // Board
            (Coord::tri(-1,1,0), Tile::new(vec![D,DR])),
            (Coord::tri(0,1,0), Tile::new(vec![D,DR])),
            (Coord::tri(-1,2,0), Tile::new(vec![DR])),
            (Coord::tri(0,2,0), Tile::new(vec![DR,UR])),
            (Coord::tri(1,1,0), Tile::new(vec![DR])),
            (Coord::tri(1,2,0), Tile::new(vec![UR])),
            (Coord::tri(2,1,0), Tile::new(vec![UR]).chikane().blockage(vec![DR])),
            (Coord::tri(2,1,-1), Tile::new(vec![U])),

            // Loop
            (Coord::tri(2,0,-1), Tile::new(vec![UR]).forced(BTreeMap::from([
                (Coord::tri(2,1,-1), UR)
            ]))),
            (Coord::tri(2,0,-2), Tile::new(vec![UR,DR]).forced(BTreeMap::from([
                (Coord::tri(2,0,-1), UR),
                (Coord::tri(1,0,-2), DR),
            ])).chikane()),
            (Coord::tri(2,0,-3), Tile::new(vec![UR]).forced(BTreeMap::from([
                (Coord::tri(2,0,-2), UR),
            ]))),
            (Coord::tri(2,0,-4), Tile::new(vec![U]).blockage(vec![UR])),
            (Coord::tri(2,-1,-4), Tile::new(vec![U,UL])),
            (Coord::tri(2,-2,-4), Tile::new(vec![UL])),
            (Coord::tri(1,-1,-4), Tile::new(vec![U,UL]).blue()),
            (Coord::tri(1,-2,-4), Tile::new(vec![DL,UL]).rotate()),
            (Coord::tri(0,-1,-4), Tile::new(vec![UL,DL]).blue()),
            (Coord::tri(0,-2,-4), Tile::new(vec![DL]).chikane()),
            (Coord::tri(0,-1,-3), Tile::new(vec![DL]).blue()),
            (Coord::tri(0,-2,-3), Tile::new(vec![D,DL]).rotate()),
            (Coord::tri(0,-1,-2), Tile::new(vec![D])),
            (Coord::tri(0,-2,-2), Tile::new(vec![D])),
            (Coord::tri(0,0,-2), Tile::new(vec![DR])),
            (Coord::tri(1,0,-2), Tile::new(vec![DR]).forced(BTreeMap::from([
                (Coord::tri(0,-0,-2), DR)
            ]))),

            (Coord::tri(3,0,-2), Tile::new(vec![DR]).forced(BTreeMap::from([
                (Coord::tri(2,0,-2), DR)
            ]))),
            (Coord::tri(4,0,-2), Tile::new(vec![UR])),
            (Coord::tri(4,0,-3), Tile::new(vec![UR])),
            (Coord::tri(4,0,-4), Tile::new(vec![UR])),
            (Coord::tri(4,0,-5), Tile::new(vec![DR,UL]).chikane().blockage(vec![UR])),

            // Oneway
            (Coord::tri(3,0,-5), Tile::new(vec![UL]).oneway()),
            (Coord::tri(2,0,-5), Tile::new(vec![UL,DL]).oneway()),

            (Coord::tri(5,0,-5), Tile::new(vec![D])),
            (Coord::tri(5,1,-5), Tile::new(vec![DL]).chikane()),
            (Coord::tri(5,1,-4), Tile::new(vec![DL]).blockage(vec![D])),
            (Coord::tri(5,1,-3), Tile::new(vec![DL]).rotate()),
            (Coord::tri(5,1,-2), Tile::new(vec![DL])),

            // loop
            (Coord::tri(5,1,-1), Tile::new(vec![UL]).forced(BTreeMap::from([
                (Coord::tri(5,1,-2), UL)
            ]))),
            (Coord::tri(4,1,-1), Tile::new(vec![DL])),
            (Coord::tri(4,1,0), Tile::new(vec![D,UL]).chikane()),

            // oneway
            (Coord::tri(3,1,0), Tile::new(vec![UL]).oneway()),

            // rest of loop
            (Coord::tri(4,2,0), Tile::new(vec![DR])),
            (Coord::tri(5,2,0), Tile::new(vec![DR,UR])),

            (Coord::tri(5,2,-1), Tile::new(vec![DR])),
            (Coord::tri(6,2,0), Tile::new(vec![DR,UR])),
            (Coord::tri(6,2,-1), Tile::new(vec![DR,D])),
            (Coord::tri(7,2,0), Tile::new(vec![DR,UR]).rotate()),
            (Coord::tri(7,2,-1), Tile::new(vec![DR,D]).chikane()),
            (Coord::tri(8,2,0), Tile::new(vec![DR,UR]).blue()),
            (Coord::tri(8,2,-1), Tile::new(vec![UR,DR])),
            (Coord::tri(9,2,0), Tile::new(vec![UR]).blue()),
            (Coord::tri(9,2,-1), Tile::new(vec![U])),

            // Choice wall
            (Coord::tri(9,1,-1), Tile::new(vec![U,UR]).choice()),
            (Coord::tri(9,1,-2), Tile::new(vec![U])),
            (Coord::tri(9,0,-1), Tile::new(vec![UR])),

            (Coord::tri(9,0,-2), Tile::new(vec![U,UR]).choice()),
            (Coord::tri(9,0,-3), Tile::new(vec![U])),
            (Coord::tri(9,-1,-2), Tile::new(vec![UR])),

            (Coord::tri(9,-1,-3), Tile::new(vec![U,UR]).choice()),
            (Coord::tri(9,-1,-4), Tile::new(vec![U])),
            (Coord::tri(9,-2,-3), Tile::new(vec![UR]).blockage(vec![U])),

            (Coord::tri(9,-2,-4), Tile::new(vec![U,UR]).choice()),
            (Coord::tri(9,-2,-5), Tile::new(vec![U])),
            (Coord::tri(9,-3,-4), Tile::new(vec![UR,U,UL])),

            // oneway and connection
            (Coord::tri(9,-1,-1), Tile::new(vec![UL]).oneway()),
            (Coord::tri(8,-1,-1), Tile::new(vec![UL,U,DL])),
            (Coord::tri(8,-1,0), Tile::new(vec![UL,U]).rotate()),

            // wrap arround
            (Coord::tri(9,-3,-5), Tile::new(vec![UL])),
            (Coord::tri(8,-3,-5), Tile::new(vec![UL,DL])),
            (Coord::tri(8,-3,-4), Tile::new(vec![UL,U])),
            (Coord::tri(8,-3,-4), Tile::new(vec![UL,D,U])),
            (Coord::tri(8,-2,-4), Tile::new(vec![D])),

            // Continue
            (Coord::tri(7,-3,-5), Tile::new(vec![UL,DL])),

            // oneway
            (Coord::tri(7,-3,-4), Tile::new(vec![DL]).oneway()),
            (Coord::tri(7,-3,-3), Tile::new(vec![DL,D]).oneway()),

            // Continue
            (Coord::tri(6,-3,-5), Tile::new(vec![UL]).rotate()),
            (Coord::tri(5,-3,-5), Tile::new(vec![UL]).blue()),
            (Coord::tri(4,-3,-5), Tile::new(vec![UL])),
            (Coord::tri(3,-3,-5), Tile::new(vec![UL])),
            (Coord::tri(2,-3,-5), Tile::new(vec![UL]).rotate()),
            (Coord::tri(1,-3,-5), Tile::new(vec![UL]).chikane()),
            (Coord::tri(0,-3,-5), Tile::new(vec![UL])),
            (Coord::tri(-1,-3,-5), Tile::new(vec![UL]).blockage(vec![DL])),
            (Coord::tri(-2,-3,-5), Tile::new(vec![UL,DL]).blue()),
            (Coord::tri(-3,-3,-5), Tile::new(vec![DL]).blue()),

            (Coord::tri(-2,-2,-5), Tile::new(vec![DL])),
            (Coord::tri(-3,-2,-5), Tile::new(vec![DL,D])),
            (Coord::tri(-4,-2,-5), Tile::new(vec![D]).blue()),

            (Coord::tri(-2,-2,-4), Tile::new(vec![UL,DL])),
            (Coord::tri(-3,-2,-4), Tile::new(vec![DL,D])),

            (Coord::tri(-2,-2,-3), Tile::new(vec![DL]).rotate()),
            (Coord::tri(-3,-2,-3), Tile::new(vec![D]).rotate()),

            (Coord::tri(-2,-2,-2), Tile::new(vec![DL,D]).chikane()),

        ]),
        start_line: vec![
            (Coord::tri(0,0,0), vec![DL,D]),
            (Coord::tri(-1,0,0), vec![D]),
        ],
        mid_line: vec![
            (Coord::tri(5,2,-1), vec![DR]),
            (Coord::tri(6,2,0), vec![DR,UR]),
        ],
        player_builder: PlayerBuilder::new(
            8,
            vec![
                Color::RGB(255, 0, 0),
                Color::RGB(0, 0, 255),
                Color::RGB(0, 255, 0),
                Color::RGB(255, 255, 0),
                Color::RGB(255, 0, 255),
                Color::RGB(0, 255, 255),
                Color::RGB(255, 100, 100),
                Color::RGB(100, 100, 255),
            ],
            vec![
                (Coord::tri(0, 0, 0), D),
                (Coord::tri(-1, 0, 0), D),
                (Coord::tri(0, -1, 0), D),
                (Coord::tri(-1, -1, 0), D),
                (Coord::tri(0, -2, 0), D),
                (Coord::tri(-1, -2, 0), D),
                (Coord::tri(0, -3, 0), D),
                (Coord::tri(-1, -3, 0), D),

            ],
        )
    }
}
