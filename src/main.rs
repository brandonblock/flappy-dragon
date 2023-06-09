use crate::game_state::State;

pub mod game_state;
pub mod player;
pub mod terrain;

use bracket_lib::prelude::*;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context, State::new())
}
