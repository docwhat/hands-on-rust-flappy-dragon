mod obstacle;
mod player;
mod state;

use bracket_lib::prelude::main_loop;
use bracket_lib::prelude::BError;
use bracket_lib::prelude::BTermBuilder;
use state::State;

enum GameMode {
    Menu,
    Playing,
    End,
}
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context, State::new())
}
