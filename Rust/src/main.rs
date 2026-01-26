pub mod game;
pub mod map;
pub mod util;
pub mod canvas_draw;
pub mod player;
pub mod boards;

use crate::map::*;
use crate::util::*;
use crate::boards::*;

fn main() {
    // let board = rtfm();
    // let scale: f64 = 42.0;
    // let start = Coord::new(360, 700);

    // let board = random_direction();
    // let scale: f64 = 36.0;
    // let start = Coord::new(100, 500);

    // let board = hourglass_loop();
    // let scale: f64 = 32.0;
    // let start = Coord::new(50, 650);

    let board = hyper_chikane();
    let scale: f64 = 32.0;
    let start = Coord::new(850, 450);

    use crate::map::{PlayerGearStrategy as PGS};
    use crate::map::{PlayerStepStrategy as PSS};

    GameState::new(board, vec![
        (PGS::Best, PSS::Best),
        // (PGS::Manual, PSS::Manual),
        (PGS::Best, PSS::Best),
        (PGS::Best, PSS::Best),
        (PGS::Best, PSS::Best),
        (PGS::Best, PSS::Best),
        (PGS::Best, PSS::Best),
        (PGS::Best, PSS::Best),
        (PGS::Best, PSS::Best),
        (PGS::Best, PSS::Best),
    ], scale, start); // .display();
}
