mod map;
mod player;

mod prelude {
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DIMENSION_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DIMENSION_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::map::*;
    pub use crate::player::*;
    pub use bracket_lib::prelude::*;
}
use prelude::*;

struct State {
    player: Player,
    map: Map,
}

impl State {
    fn new() -> Self {
        let MapBuilderResult { player, map } =
            MapBuilder::new(SCREEN_WIDTH, SCREEN_HEIGHT, 1).build();
        Self { player, map }
    }

    fn render_tick(&mut self, ctx: &mut BTerm) {
        let mut draw = DrawBatch::new();
        draw.target(0);
        draw.cls();
        draw.target(1);
        draw.cls();

        self.player.render(&mut draw);
        self.map.render(&mut draw);

        draw.submit(0).expect("Batch Error");
        render_draw_buffer(ctx).expect("Render Error");
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut bracket_lib::prelude::BTerm) {
        self.player.update(ctx);
        self.render_tick(ctx);
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

    main_loop(context, State::new())
}
