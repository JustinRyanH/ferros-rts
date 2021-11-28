use bracket_lib::prelude::{BError, BTermBuilder, GameState};

mod prelude {
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DIMENSION_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DIMENSION_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use bracket_lib::prelude::*;
}
use prelude::*;

struct Player {
    pub position: Point,
    pub color: ColorPair,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Self {
            position: Point::new(x, y),
            color: ColorPair::new(GREEN, BLACK),
        }
    }

    fn render(&self, draw: &mut DrawBatch) {
        draw.set(self.position, self.color, to_cp437('@'));
    }
}

struct State {
    player: Player,
}

impl State {
    fn new() -> Self {
        Self {
            player: Player::new(DIMENSION_WIDTH / 2, DIMENSION_HEIGHT / 2),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut bracket_lib::prelude::BTerm) {
        let mut draw = DrawBatch::new();
        draw.target(0);
        draw.cls();

        self.player.render(&mut draw);

        draw.submit(0).expect("Batch Error");
        render_draw_buffer(ctx).expect("Render Error");
    }
}

fn main() -> BError {
    let mut context = BTermBuilder::simple80x50()
        .with_title("Ferros RTS")
        .with_dimensions(DIMENSION_WIDTH, DIMENSION_HEIGHT)
        .with_tile_dimensions(24, 24)
        .with_fps_cap(30.0)
        .build()?;

    context.with_post_scanlines(true);

    main_loop(context, State::new())
}
