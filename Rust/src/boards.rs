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

// TODO: Choice fields are no longer guaranteed to have a previous round, as players can skip the choice in an earlier round! (Choice should be a map from round?)
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

// TODO: Choice fields are no longer guaranteed to have a previous round, as players can skip the choice in an earlier round! (Choice should be a map from round?)
pub fn hourglass_loop() -> HexMap {
    HexMap {
        tiles: BTreeMap::from([
            // Start
            (Coord::tri(0,0,0), Tile::new(vec![D,DR]).start()),
            (Coord::tri(0,0,-1), Tile::new(vec![D,DL]).start()),
            (Coord::tri(0,-1,0), Tile::new(vec![D,DR]).start()),
            (Coord::tri(0,-1,-1), Tile::new(vec![D,DL]).start()),
            (Coord::tri(0,-2,0), Tile::new(vec![D,DR]).start()),
            (Coord::tri(0,-2,-1), Tile::new(vec![D,DL]).start()),
            (Coord::tri(0,-3,0), Tile::new(vec![D,DR]).start()),
            (Coord::tri(0,-3,-1), Tile::new(vec![D,DL]).start()),
            (Coord::tri(0,-4,0), Tile::new(vec![D,DR]).start()),
            (Coord::tri(0,-4,-1), Tile::new(vec![D,DL]).start()),

            // Board
            (Coord::tri(0,1,0), Tile::new(vec![DR])),
            (Coord::tri(0,1,-1), Tile::new(vec![D,DR])),
            (Coord::tri(0,2,-1), Tile::new(vec![UR,DR])),
            (Coord::tri(1,1,-1), Tile::new(vec![UR])),
            (Coord::tri(1,2,-1), Tile::new(vec![UR])),
            (Coord::tri(1,1,-2), Tile::new(vec![UR])),
            (Coord::tri(1,2,-2), Tile::new(vec![U])),
            (Coord::tri(1,1,-3), Tile::new(vec![UR,DR])),
            (Coord::tri(1,1,-4), Tile::new(vec![DR]).blue()),
            (Coord::tri(2,1,-3), Tile::new(vec![UR])),
            (Coord::tri(2,1,-4), Tile::new(vec![UR]).chikane()),
            (Coord::tri(2,1,-5), Tile::new(vec![UR])),
            (Coord::tri(2,1,-6), Tile::new(vec![UR,U])),
            (Coord::tri(2,0,-6), Tile::new(vec![UR]).chikane()),
            (Coord::tri(2,1,-7), Tile::new(vec![U])),
            (Coord::tri(2,0,-7), Tile::new(vec![UR]).blockage(vec![U])),

            // mid chikane
            (Coord::tri(2,0,-8), Tile::new(vec![UR]).chikane().forced(BTreeMap::from([
                (Coord::tri(2,0,-7),UR),
                (Coord::tri(-1,4,-12),UL),
            ]))),

            (Coord::tri(2,0,-9), Tile::new(vec![UR]).blockage(vec![D])),
            (Coord::tri(2,0,-10), Tile::new(vec![U])),
            (Coord::tri(2,-1,-10), Tile::new(vec![U,UR]).choice()),

            // Up path
            (Coord::tri(2,-2,-10), Tile::new(vec![U])),
            (Coord::tri(2,-3,-10), Tile::new(vec![UR])),
            (Coord::tri(2,-3,-11), Tile::new(vec![DR]).blue()),
            (Coord::tri(3,-3,-11), Tile::new(vec![UR]).chikane()),

            // Right path
            (Coord::tri(2,-1,-11), Tile::new(vec![DR])),
            (Coord::tri(3,-1,-11), Tile::new(vec![UR]).chikane()),
            (Coord::tri(3,-1,-12), Tile::new(vec![U]).blue()),
            (Coord::tri(3,-2,-12), Tile::new(vec![U])),

            // Continue
            (Coord::tri(3,-3,-12), Tile::new(vec![UR])),
            (Coord::tri(3,-3,-13), Tile::new(vec![DR])),
            (Coord::tri(4,-3,-13), Tile::new(vec![D])),
            (Coord::tri(4,-2,-13), Tile::new(vec![D,DR])),
            (Coord::tri(5,-2,-13), Tile::new(vec![D,DL])),
            (Coord::tri(4,-1,-13), Tile::new(vec![D,DR])),
            (Coord::tri(5,-1,-13), Tile::new(vec![D,DL])),
            (Coord::tri(4,0,-13), Tile::new(vec![D,DR]).chikane()),
            (Coord::tri(5,0,-13), Tile::new(vec![D,DL]).blue()),
            (Coord::tri(4,1,-13), Tile::new(vec![D,DR])),
            (Coord::tri(5,1,-13), Tile::new(vec![D,DL])),
            (Coord::tri(4,2,-13), Tile::new(vec![DR])),
            (Coord::tri(5,2,-13), Tile::new(vec![D])),
            (Coord::tri(5,3,-13), Tile::new(vec![D])),
            (Coord::tri(5,4,-13), Tile::new(vec![D,DL])),
            (Coord::tri(5,5,-13), Tile::new(vec![DL])),
            (Coord::tri(5,4,-12), Tile::new(vec![UL,DL]).blue()),
            (Coord::tri(5,5,-12), Tile::new(vec![UL])),
            (Coord::tri(4,4,-12), Tile::new(vec![UL])),
            (Coord::tri(4,5,-12), Tile::new(vec![UL,U])),
            (Coord::tri(3,4,-12), Tile::new(vec![UL])),
            (Coord::tri(3,5,-12), Tile::new(vec![U])),
            (Coord::tri(2,4,-12), Tile::new(vec![UL]).chikane()),
            (Coord::tri(1,4,-12), Tile::new(vec![UL])),
            (Coord::tri(0,4,-12), Tile::new(vec![UL])),
            (Coord::tri(-1,4,-12), Tile::new(vec![UL]).blockage(vec![U])),
            // <-- mid chikane from before is here
            (Coord::tri(-3,4,-12), Tile::new(vec![UL]).blockage(vec![D])),
            (Coord::tri(-4,4,-12), Tile::new(vec![UL])),
            (Coord::tri(-5,4,-12), Tile::new(vec![UL])),
            (Coord::tri(-6,4,-12), Tile::new(vec![UL]).chikane()),
            (Coord::tri(-7,4,-12), Tile::new(vec![UL])),
            (Coord::tri(-8,4,-12), Tile::new(vec![UL])),
            (Coord::tri(-9,4,-12), Tile::new(vec![UL,DL])),

            // Blue speed-down zone
            (Coord::tri(-10,4,-12), Tile::new(vec![UL,DL]).blue().chikane()),
            (Coord::tri(-11,4,-12), Tile::new(vec![DL]).blue()),
            (Coord::tri(-11,4,-11), Tile::new(vec![D]).blue()),

            (Coord::tri(-10,5,-12), Tile::new(vec![UL,DL]).chikane()),
            (Coord::tri(-11,5,-12), Tile::new(vec![DL,D])),
            (Coord::tri(-11,5,-11), Tile::new(vec![D])),

            (Coord::tri(-11,6,-12), Tile::new(vec![DL,D])),
            (Coord::tri(-11,6,-11), Tile::new(vec![DR,D])),
        ]),
        start_line: vec![
            (Coord::tri(0,0,0), vec![D]),
            (Coord::tri(0,1,-1), vec![DL,D,DR]),
        ],
        mid_line: vec![
            (Coord::tri(4,-1,-13), vec![D]),
            (Coord::tri(5,-1,-13), vec![D,DL]),
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
                (Coord::tri(0,0,0), D),
                (Coord::tri(0,0,-1), D),
                (Coord::tri(0,-1,0), D),
                (Coord::tri(0,-1,-1), D),
                (Coord::tri(0,-2,0), D),
                (Coord::tri(0,-2,-1), D),
                (Coord::tri(0,-3,0), D),
                (Coord::tri(0,-3,-1), D),
                (Coord::tri(0,-4,0), D),
                (Coord::tri(0,-4,-1), D),
            ],
        )
    }
}

pub fn hyper_chikane() -> HexMap {
    HexMap {
        tiles: BTreeMap::from([
            // Start
            (Coord::tri(0,0,0), Tile::new(vec![U,UL]).start()),
            (Coord::tri(0,0,1), Tile::new(vec![U,UR]).start()),
            (Coord::tri(0,1,0), Tile::new(vec![U,UL]).start()),
            (Coord::tri(0,1,1), Tile::new(vec![U,UR]).start()),
            (Coord::tri(0,2,0), Tile::new(vec![U,UL]).start()),
            (Coord::tri(0,2,1), Tile::new(vec![U,UR]).start()),
            (Coord::tri(0,3,0), Tile::new(vec![U,UL]).start()),

            // Board
            (Coord::tri(0,-1,1), Tile::new(vec![U,UR])),
            (Coord::tri(0,-1,0), Tile::new(vec![U,UL])),
            (Coord::tri(0,-2,1), Tile::new(vec![UL]).chikane()),
            (Coord::tri(0,-2,0), Tile::new(vec![UL])),
            (Coord::tri(0,-3,1), Tile::new(vec![DL])),
            (Coord::tri(0,-3,2), Tile::new(vec![DL])),
            (Coord::tri(0,-3,3), Tile::new(vec![UL]).chikane()),
            (Coord::tri(-1,-3,3), Tile::new(vec![U,DL]).choice()),

            // Up
            (Coord::tri(-1,-4,3), Tile::new(vec![UL]).chikane()),
            (Coord::tri(-2,-4,3), Tile::new(vec![UL])),
            (Coord::tri(-3,-4,3), Tile::new(vec![DL]).blue()),

            // Left
            (Coord::tri(-1,-3,4), Tile::new(vec![UL])),
            (Coord::tri(-2,-3,4), Tile::new(vec![UL]).blue()),
            (Coord::tri(-3,-3,4), Tile::new(vec![U]).chikane()),

            // Continue
            (Coord::tri(-3,-4,4), Tile::new(vec![UL]).chikane()),
            (Coord::tri(-4,-4,4), Tile::new(vec![UL,DL])),
            (Coord::tri(-5,-4,4), Tile::new(vec![DL])),
            (Coord::tri(-4,-4,5), Tile::new(vec![DL]).blue()),
            (Coord::tri(-5,-4,5), Tile::new(vec![D]).chikane()),
            (Coord::tri(-5,-3,5), Tile::new(vec![D])),
            (Coord::tri(-5,-2,5), Tile::new(vec![D]).chikane()),
            (Coord::tri(-5,-1,5), Tile::new(vec![D,DR])),

            // blue offshute
            (Coord::tri(-4,-1,5), Tile::new(vec![D]).blue()),
            (Coord::tri(-4,0,5), Tile::new(vec![DL]).blue()),

            (Coord::tri(-5,0,5), Tile::new(vec![D]).chikane()),
            (Coord::tri(-5,1,5), Tile::new(vec![D])),
            (Coord::tri(-5,2,5), Tile::new(vec![D]).chikane()),
            (Coord::tri(-5,3,5), Tile::new(vec![D])),
            (Coord::tri(-5,4,5), Tile::new(vec![D]).chikane()),
            (Coord::tri(-5,5,5), Tile::new(vec![D,DR])),

            // blue offshute
            (Coord::tri(-4,5,5), Tile::new(vec![D]).blue()),
            (Coord::tri(-4,6,5), Tile::new(vec![DL]).blue()),

            (Coord::tri(-5,6,5), Tile::new(vec![D]).chikane()),
            (Coord::tri(-5,7,5), Tile::new(vec![D])),
            (Coord::tri(-5,8,5), Tile::new(vec![DR]).chikane()),
            (Coord::tri(-4,8,5), Tile::new(vec![UR])),
            (Coord::tri(-4,8,4), Tile::new(vec![UR,DR])),
            (Coord::tri(-3,8,4), Tile::new(vec![UR])),
            (Coord::tri(-3,7,4), Tile::new(vec![DR]).blue()),
            (Coord::tri(-3,8,3), Tile::new(vec![UR]).chikane()),
            (Coord::tri(-3,8,2), Tile::new(vec![UR]).forced(BTreeMap::from([
                (Coord::tri(-3,8,3), UR),
            ]))),
            (Coord::tri(-3,8,1), Tile::new(vec![UR,UL]).forced(BTreeMap::from([
                (Coord::tri(-3,8,2), UR),
                (Coord::tri(-4,10,-1), UL),
            ]))),

            // Loop
            (Coord::tri(-3,8,0), Tile::new(vec![UR])), // TODO: .blockage(vec![D]) ?
            (Coord::tri(-3,8,-1), Tile::new(vec![UR])),
            (Coord::tri(-3,8,-2), Tile::new(vec![DR]).blue().chikane()),
            (Coord::tri(-2,8,-2), Tile::new(vec![D]).chikane()),
            (Coord::tri(-2,9,-2), Tile::new(vec![D])),
            (Coord::tri(-2,10,-2), Tile::new(vec![DL]).chikane()),
            (Coord::tri(-2,10,-1), Tile::new(vec![UL]).chikane()),
            (Coord::tri(-3,10,-1), Tile::new(vec![UL]).chikane()),
            (Coord::tri(-4,10,-1), Tile::new(vec![UL]).forced(BTreeMap::from([
                (Coord::tri(-3,10,-1), UL),
            ]))),

            // After loop
            (Coord::tri(-6,10,-1), Tile::new(vec![UL])),  // TODO: .blockage(vec![D]) ?
            (Coord::tri(-7,10,-1), Tile::new(vec![UL])),
            (Coord::tri(-8,10,-1), Tile::new(vec![U]).chikane()),
            (Coord::tri(-8,9,-1), Tile::new(vec![UL,UR]).chikane().choice()),

            // Left
            (Coord::tri(-9,9,-1), Tile::new(vec![U]).chikane()),
            (Coord::tri(-9,8,-1), Tile::new(vec![U])),
            (Coord::tri(-9,7,-1), Tile::new(vec![UR]).blue()),

            // Right
            (Coord::tri(-8,9,-2), Tile::new(vec![U])),
            (Coord::tri(-8,8,-2), Tile::new(vec![U]).chikane().blue()),
            (Coord::tri(-8,7,-2), Tile::new(vec![UL])),

            // Continue
            (Coord::tri(-9,7,-2), Tile::new(vec![U]).chikane()),
            (Coord::tri(-9,6,-2), Tile::new(vec![UR])),
            (Coord::tri(-9,6,-3), Tile::new(vec![DR])),
            (Coord::tri(-8,6,-3), Tile::new(vec![DR]).chikane()),
            (Coord::tri(-7,6,-3), Tile::new(vec![D]).chikane()),
            (Coord::tri(-7,7,-3), Tile::new(vec![D]).chikane()),
            (Coord::tri(-7,8,-3), Tile::new(vec![D,DR])),
            (Coord::tri(-7,9,-3), Tile::new(vec![DR])),

            (Coord::tri(-6,8,-3), Tile::new(vec![DR,UR]).blue()),
            (Coord::tri(-6,9,-3), Tile::new(vec![UR]).chikane()),

            (Coord::tri(-6,8,-4), Tile::new(vec![DR,UR]).chikane()),
            (Coord::tri(-6,9,-4), Tile::new(vec![U,UR]).chikane()),
            (Coord::tri(-6,9,-5), Tile::new(vec![U,UR]).chikane()),

        ]),
        start_line: vec![
            (Coord::tri(0,0,0), vec![U]),
            (Coord::tri(0,-1,1), vec![U,UR]),
        ],
        mid_line: vec![
            (Coord::tri(-3,8,4), vec![UR]),
            (Coord::tri(-3,7,4), vec![DR]),
        ],
        player_builder: PlayerBuilder::new(
            7,
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
                (Coord::tri(0,0,0), U),
                (Coord::tri(0,0,1), U),
                (Coord::tri(0,1,0), U),
                (Coord::tri(0,1,1), U),
                (Coord::tri(0,2,0), U),
                (Coord::tri(0,2,1), U),
                (Coord::tri(0,3,0), U),
            ],
        )
    }
}
