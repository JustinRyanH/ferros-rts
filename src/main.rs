mod components;
mod generator;
mod maps;
mod player;
mod resources;
mod spawner;
mod systems;
mod tools;

mod prelude {
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DIMENSION_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DIMENSION_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub const UI_LAYER: usize = 10_000;
    pub use crate::components::*;
    pub use crate::generator::*;
    pub use crate::maps::*;
    pub use crate::player::*;
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
        .add_system(systems::collisions_system())
        .flush()
        .add_system(systems::render::map_system())
        .add_system(systems::render::characters_system())
        .build()
}

pub fn build_build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(systems::world_gen_system())
        .add_system(systems::world_gen_progress_system())
        .flush()
        .add_system(systems::render::builder_system())
        .add_system(systems::render::progress_bar_system())
        .build()
}

struct Game {
    ecs: World,
    resources: Resources,
    gameplay_systems: Schedule,
    build_systems: Schedule,
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
        resources.insert(Some(ProgressBar::new(SCREEN_HEIGHT - ProgressBar::HEIGHT)));

        Self {
            ecs,
            resources,
            gameplay_systems: build_scheduler(),
            build_systems: build_build_scheduler(),
        }
    }

    fn tick_on_command(&mut self) {
        if self.resources.get::<Camera>().is_some() && self.resources.get::<Map>().is_some() {
            self.gameplay_systems
                .execute(&mut self.ecs, &mut self.resources);
        } else {
            self.build_systems
                .execute(&mut self.ecs, &mut self.resources);
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
