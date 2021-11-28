use bracket_lib::prelude::{BError, BTermBuilder, GameState};

mod prelude {
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DIMENSION_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DIMENSION_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use bracket_lib::prelude::*;
}
use prelude::*;

struct State {}

impl State {
    fn new() -> Self {
        Self {}
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut bracket_lib::prelude::BTerm) {
        ctx.set(2, 2, WHITE, BLACK, to_cp437('@'));
    }
}

fn main() -> BError {
    let mut context = BTermBuilder::simple80x50()
        .with_title("Ferros RTS")
        .with_dimensions(DIMENSION_WIDTH, DIMENSION_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_fps_cap(30.0)
        .build()?;

    context.with_post_scanlines(true);

    main_loop(context, State::new())
}
