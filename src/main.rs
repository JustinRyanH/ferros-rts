mod components;
mod generator;
mod maps;
mod resources;
mod spawner;
mod systems;
mod tools;
mod turn_state;

mod prelude {
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DIMENSION_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DIMENSION_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub const UI_LAYER: usize = 10_000;
    pub use crate::components::*;
    pub use crate::generator::*;
    pub use crate::maps::*;
    pub use crate::resources::*;
    pub use crate::spawner::*;
    pub use crate::tools::*;
    pub use crate::turn_state::*;
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
}
use crate::prelude::*;

fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(systems::player_input_system())
        .flush()
        .add_system(systems::render::map_system())
        .add_system(systems::render::characters_system())
        .build()
}

fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(systems::movement_system())
        .flush()
        .add_system(systems::collisions_system())
        .flush()
        .add_system(systems::render::map_system())
        .add_system(systems::render::characters_system())
        .add_system(systems::end_turn_system())
        .build()
}

fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(systems::random_move_system())
        .flush()
        .add_system(systems::movement_system())
        .flush()
        .add_system(systems::render::map_system())
        .add_system(systems::render::characters_system())
        .build()
}

fn build_build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(systems::world_gen_system())
        .add_system(systems::world_gen_progress_system())
        .add_system(systems::finish_world_gen_system())
        .flush()
        .add_system(systems::render::builder_system())
        .add_system(systems::render::progress_bar_system())
        .build()
}

struct Game {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
    build_systems: Schedule,
}

impl Game {
    fn new() -> Self {
        let ecs = World::default();
        let mut resources = Resources::default();
        let generator = GeneraotrRunner::default();
        let builder = MapBuilder::new(SCREEN_WIDTH + 20, SCREEN_WIDTH + 20);

        resources.insert(WorldGenRng::new());
        resources.insert(builder);
        resources.insert(generator);
        resources.insert(Some(ProgressBar::new(SCREEN_HEIGHT - ProgressBar::HEIGHT)));
        resources.insert(Camera::new(Point::zero()));
        resources.insert(TurnState::AwaitingInput);

        Self {
            ecs,
            resources,
            build_systems: build_build_scheduler(),
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }

    fn tick_on_command(&mut self) {
        if self.has_map() {
            self.run_game();
        } else {
            self.build_systems
                .execute(&mut self.ecs, &mut self.resources)
        }
    }

    fn run_game(&mut self) {
        let curret_turn_state = *self
            .resources
            .get::<TurnState>()
            .expect("Resources requires TurnState");
        match curret_turn_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.resources),
        }
    }

    fn has_map(&self) -> bool {
        self.resources.get::<Map>().is_some()
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
