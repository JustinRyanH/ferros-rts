mod components;
mod generator;
mod maps;
mod player;
mod progress;
mod spawner;
mod states;
mod systems;
mod tools;

mod prelude {
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DIMENSION_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DIMENSION_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::components::*;
    pub use crate::generator::*;
    pub use crate::maps::*;
    pub use crate::player::*;
    pub use crate::progress::*;
    pub use crate::spawner::*;
    pub use crate::states::*;
    pub use crate::tools::*;
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
}
use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(systems::player_input_system())
        .add_system(systems::map_render_system())
        .add_system(systems::entity_render_system())
        .add_system(systems::collisions_system())
        .build()
}

struct Game {
    ecs: World,
    resources: Resources,
    systems: Schedule,
}

impl Game {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut builder = MapBuilderState::default();
        while !builder.is_finished() {
            builder.build_world(&mut rng);
        }
        let MapResult { map, player } = builder.builder.build_map();
        let player = player.expect("Failed to place player in worlds");
        spawn_player(&mut ecs, player);
        builder
            .builder
            .rooms
            .iter()
            .filter(|room| room.center() != player)
            .map(|r| r.center())
            .for_each(|pos| {
                spawn_monster(&mut ecs, &mut rng, pos);
            });
        resources.insert(map);
        resources.insert(Camera::new(player));

        Self {
            ecs,
            resources,
            systems: build_scheduler(),
        }
    }
}

impl GameState for Game {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();

        self.resources.insert(ctx.key);
        self.systems.execute(&mut self.ecs, &mut self.resources);

        render_draw_buffer(ctx).expect("Render Error");
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
