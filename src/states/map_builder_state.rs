// pub const BUILD_BUDGET: std::time::Duration = std::time::Duration::from_micros(1);

use crate::prelude::*;

pub struct MapBuilderState {
    pub generator: GeneraotrRunner,
    pub builder: MapBuilder,
    pub rng: RandomNumberGenerator,
    pub render_map: Option<()>,
    pub building: bool,
}

impl MapBuilderState {
    fn build_world(&mut self) {
        if !self.building {
            return;
        }
        // TODO(jhurstwright): Actually do budget thing for more complex builds
        // let start = std::time::Instant::now();
        // while (std::time::Instant::now() - start) < BUILD_BUDGET {
        self.generator.next(&mut self.builder, &mut self.rng);
        // }
    }
}

impl GameState for MapBuilderState {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut draw = DrawBatch::new();
        clear_batch(&mut draw);

        if let Some(code) = ctx.key {
            if code == VirtualKeyCode::Space {
                self.building = !self.building;
            }
        }

        self.build_world();

        draw.target(0);
        self.builder.render(&mut draw);
        self.generator.render_progress(&mut draw);
        submit_batch(ctx, &mut draw).unwrap();
        if self.generator.is_finished() {
            self.render_map = Some(());
        }
    }
}

impl Default for MapBuilderState {
    fn default() -> Self {
        let num_of_rooms = 10;
        let generator = GeneraotrRunner::new(vec![
            GeneratorCommand::FillMap(TileType::Wall),
            GeneratorCommand::GenerateRooms {
                num_of_rooms,
                max_room_size: 10,
            },
            GeneratorCommand::Tunnel {
                num_of_tunnels: (num_of_rooms * 2) - 2,
            },
            GeneratorCommand::PlacePlayerInRoom,
        ]);
        Self {
            builder: MapBuilder::new(SCREEN_WIDTH, SCREEN_HEIGHT, 10),
            rng: RandomNumberGenerator::new(),
            generator,
            render_map: None,
            building: false,
        }
    }
}
