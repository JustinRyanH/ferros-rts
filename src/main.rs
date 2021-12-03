mod generator;
mod maps;
mod player;
mod tools;

mod prelude {
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DIMENSION_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DIMENSION_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::generator::*;
    pub use crate::maps::*;
    pub use crate::player::*;
    pub use crate::tools::*;
    pub use bracket_lib::prelude::*;
}
use prelude::*;
pub struct MapBuilderState {
    generator: GeneraotrRunner,
    builder: MapBuilder,
    rng: RandomNumberGenerator,
    render_map: Option<()>,
    show_menu: bool,
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
}

impl GameState for MapBuilderState {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut draw = DrawBatch::new();
        Self::clear_batch(&mut draw);

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.generator.next(&mut self.builder, &mut self.rng);
        }

        draw.target(0);
        if let Some(()) = self.render_map {
            let MapResult { map, player } = self.builder.build_map();
            map.render(&mut draw);
            if let Some(player) = player {
                draw.target(1);
                player.render(&mut draw);
            }
        } else {
            self.builder.render(&mut draw);
            draw.target(1);
            self.generator.render_menu(&mut draw);
        }

        Self::submit_batch(ctx, &mut draw).unwrap();
        if self.generator.is_finished() {
            self.render_map = Some(());
        }
    }
}

impl Default for MapBuilderState {
    fn default() -> Self {
        let generator = GeneraotrRunner::new(vec![
            GeneratorCommand::FillMap(TileType::Wall),
            GeneratorCommand::GenerateRooms {
                num_of_rooms: 10,
                max_room_size: 10,
            },
            GeneratorCommand::Tunnel,
            GeneratorCommand::PlacePlayerInRoom,
        ]);
        Self {
            builder: MapBuilder::new(SCREEN_WIDTH, SCREEN_HEIGHT, 10),
            rng: RandomNumberGenerator::new(),
            generator,
            render_map: None,
            show_menu: false,
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
