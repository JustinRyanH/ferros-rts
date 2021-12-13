mod components;
mod generator;
mod maps;
mod player;
mod progress;
mod resources;
mod spawner;
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
    pub use crate::resources::*;
    pub use crate::spawner::*;
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
    gameplay_systems: Schedule,
}

impl Game {
    fn new() -> Self {
        let ecs = World::default();
        let mut resources = Resources::default();
        let generator = GeneraotrRunner::default();
        let builder = MapBuilder::new(SCREEN_WIDTH, SCREEN_WIDTH);
        resources.insert(WorldGenRng::new());
        resources.insert(builder);
        resources.insert(generator);

        Self {
            ecs,
            resources,
            gameplay_systems: build_scheduler(),
        }
    }

    fn tick_on_command(&mut self) {
        if self.resources.get::<Camera>().is_some() && self.resources.get::<Map>().is_some() {
            self.gameplay_systems
                .execute(&mut self.ecs, &mut self.resources);
        } else {
            let (map, player) = {
                let mut rng = self
                    .resources
                    .get_mut::<WorldGenRng>()
                    .expect("Expected a WorldGenRng");
                let mut generator = self
                    .resources
                    .get_mut::<GeneraotrRunner>()
                    .expect("No Generator Resource");
                let mut builder = self
                    .resources
                    .get_mut::<MapBuilder>()
                    .expect("No Generator Runner");

                while !generator.is_finished() {
                    generator.next(&mut builder, &mut rng);
                }
                let MapResult { map, player } = builder.build_map();
                let player = player.expect("Failed to place player in worlds");
                spawn_player(&mut self.ecs, player);
                builder
                    .rooms
                    .iter()
                    .filter(|room| room.center() != player)
                    .map(|r| r.center())
                    .for_each(|pos| {
                        spawn_monster(&mut self.ecs, &mut rng, pos);
                    });
                (map, player)
            };
            self.resources.insert(map);
            self.resources.insert(Camera::new(player));
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
        self.tick_on_command();

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
