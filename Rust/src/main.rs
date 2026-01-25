pub mod game;
pub mod map;
pub mod util;
pub mod canvas_draw;
pub mod player;
pub mod boards;

use crate::map::*;
use crate::util::Direction::*;
use crate::util::*;
use crate::boards::*;

fn main() {
    let mut rtfm = rtfm();

    use crate::map::{PlayerGearStrategy as PGS};
    use crate::map::{PlayerStepStrategy as PSS};

    GameState::new(rtfm, vec![
        (PGS::Manual, PSS::Manual),
        (PGS::Best, PSS::Best),
        (PGS::Best, PSS::Best),
        (PGS::Best, PSS::Best),
        (PGS::Best, PSS::Best),
        (PGS::Best, PSS::Best),
        (PGS::Best, PSS::Best),
        (PGS::Best, PSS::Best),
        (PGS::Best, PSS::Best),
    ]); // .display();
}
