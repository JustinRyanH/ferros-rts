mod generator;
mod maps;
mod player;
mod states;
mod tools;

mod prelude {
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DIMENSION_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DIMENSION_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::generator::*;
    pub use crate::maps::*;
    pub use crate::player::*;
    pub use crate::states::*;
    pub use crate::tools::*;
    pub use bracket_lib::prelude::*;
}
use crate::prelude::*;

#[derive(Debug)]
enum State {
    WorldGen,
}

struct Game {
    state: State,
    world_gen: MapBuilderState,
}

impl Game {
    fn new() -> Self {
        Self {
            state: State::WorldGen,
            world_gen: MapBuilderState::default(),
        }
    }
}

impl GameState for Game {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.state {
            State::WorldGen => {
                self.world_gen.tick(ctx);
            }
        }
    }
}

fn main() -> BError {
    let mut context = BTermBuilder::simple80x50()
        .with_title("Ferros RTS")
        .with_dimensions(DIMENSION_WIDTH, DIMENSION_HEIGHT)
        .with_fancy_console(SCREEN_WIDTH, SCREEN_HEIGHT, "terminal8x8.png")
        .with_tile_dimensions(28, 28)
        .with_fps_cap(30.0)
        .build()?;

    context.with_post_scanlines(true);

    main_loop(context, Game::new())
}
