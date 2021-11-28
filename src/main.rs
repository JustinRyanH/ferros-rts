mod map;
mod player;
mod player_state;

mod prelude {
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DIMENSION_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DIMENSION_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::map::*;
    pub use crate::player::*;
    pub use crate::player_state::*;
    pub use bracket_lib::prelude::*;
}
use prelude::*;

pub struct MapBuilderState {
    builder: MapBuilder,
}

impl MapBuilderState {
    // pub fn render_map(&self, batch: &mut DrawBatch) {}

    fn clear_batch(draw: &mut DrawBatch) {
        draw.target(0);
        draw.cls();
        draw.target(1);
        draw.cls();
    }

    fn submit_batch(ctx: &mut BTerm, draw: &mut DrawBatch) -> BError {
        draw.submit(0)?;
        render_draw_buffer(ctx)
    }

    fn draw_menu(draw: &mut DrawBatch) {
        let margin = 5;
        let menu_width = 20;
        let x = SCREEN_WIDTH - (menu_width + margin);
        let h = 11;
        let modal = Rect::with_size(x, margin, menu_width, h);
        draw.draw_double_box(modal, ColorPair::new(GREY, BLACK));

        let mut buf = TextBuilder::empty();
        let mut block = TextBlock::new(x + 1, margin + 1, menu_width - 1, h - 1);
        buf.fg(RGB::named(WHITE))
            .bg(RGB::named(BLACK))
            .ln()
            .centered("Map Building")
            .ln()
            .ln()
            .append("[ ] Fill Map")
            .ln()
            .append("[ ] Generate Rooms")
            .ln()
            .append("[ ] Connect Rooms")
            .ln()
            .append("[ ] Place Player")
            .ln()
            .append("[ ] Finished")
            .ln()
            .ln()
            .fg(RGB::named(RED))
            .centered("Space to Continue")
            .reset();
        block.print(&buf).expect("Text was too big");
        block.render_to_draw_batch(draw);
    }
}

impl GameState for MapBuilderState {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut draw = DrawBatch::new();
        Self::clear_batch(&mut draw);

        self.builder.map.render(&mut draw);

        Self::draw_menu(&mut draw);

        Self::submit_batch(ctx, &mut draw).unwrap();
    }
}

impl Default for MapBuilderState {
    fn default() -> Self {
        Self {
            builder: MapBuilder::new(SCREEN_WIDTH, SCREEN_HEIGHT, 10),
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

    main_loop(context, MapBuilderState::default())
}
