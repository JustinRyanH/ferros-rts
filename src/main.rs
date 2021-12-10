mod generator;
mod maps;
mod player;
mod progress;
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
    pub use crate::progress::*;
    pub use crate::states::*;
    pub use crate::tools::*;
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
}
use crate::prelude::*;

struct CurrentWorld {
    player: Player,
    map: Map,
    camera: Camera,
}

impl CurrentWorld {
    fn new(player: Player, map: Map) -> Self {
        let camera = Camera::new(player.position);
        Self {
            player,
            map,
            camera,
        }
    }

    fn update(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left | VirtualKeyCode::H => Point::new(-1, 0),
                VirtualKeyCode::Right | VirtualKeyCode::L => Point::new(1, 0),
                VirtualKeyCode::Up | VirtualKeyCode::K => Point::new(0, -1),
                VirtualKeyCode::Down | VirtualKeyCode::J => Point::new(0, 1),
                _ => Point::new(0, 0),
            };
            if self.map.is_floor(self.player.new_position(delta)) {
                self.player.move_position(delta);
                self.camera.on_player_move(self.player.position);
            }
        }
    }
}

impl GameState for CurrentWorld {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut draw = DrawBatch::new();
        clear_batch(&mut draw);

        self.update(ctx);

        self.map.render(&self.camera, &mut draw);
        self.player.render(&self.camera, &mut draw);
        submit_batch(ctx, &mut draw).unwrap();
    }
}

struct Game {
    world_gen: MapBuilderState,
    world: Option<CurrentWorld>,
}

impl Game {
    fn new() -> Self {
        Self {
            world_gen: MapBuilderState::default(),
            world: None,
        }
    }
}

impl GameState for Game {
    fn tick(&mut self, ctx: &mut BTerm) {
        if let Some(world) = &mut self.world {
            world.tick(ctx)
        } else {
            if self.world_gen.generator.is_finished() {
                let MapResult { map, player } = self.world_gen.builder.build_map();
                let player = player.expect("The player was not placed in the world");
                self.world = Some(CurrentWorld::new(player, map));
            }
            self.world_gen.tick(ctx);
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
