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

#[system]
pub fn world_gen(
    #[resource] rng: &mut WorldGenRng,
    #[resource] generator: &mut GeneraotrRunner,
    #[resource] builder: &mut MapBuilder,
    commands: &mut CommandBuffer,
) {
    if !generator.is_finished() {
        generator.next(builder, rng);
        return;
    }
    let MapResult { map, player } = builder.build_map();
    let player = player.expect("Failed to place player in worlds");
    spawn_player(commands, player);
    builder
        .rooms
        .iter()
        .filter(|room| room.center() != player)
        .map(|r| r.center())
        .for_each(|pos| {
            spawn_monster(commands, rng, pos);
        });
    commands.exec_mut(move |_, resources| {
        resources.insert(map.clone());
        resources.insert(Camera::new(player));
    });
}

#[system]
pub fn builder_render(#[resource] builder: &mut MapBuilder) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    for tile in builder.fill_tile.iter() {
        let region = Rect::with_size(0, 0, builder.width, builder.height);
        draw_batch.fill_region(region, ColorPair::new(YELLOW, BLACK), *tile);
    }
    for room in builder.rooms.iter() {
        draw_batch.fill_region(*room, ColorPair::new(RED, BLACK), TileType::Floor);
    }
    for tunnel in builder.tunnels.iter() {
        tunnel.render(&mut draw_batch);
    }
    for player in builder.player.iter() {
        draw_batch.set(*player, ColorPair::new(GREEN, BLACK), to_cp437('@'));
    }
    draw_batch.submit(0).expect("Batch Error");
}

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(systems::player_input_system())
        .add_system(systems::map_render_system())
        .add_system(systems::entity_render_system())
        .add_system(systems::collisions_system())
        .build()
}

pub fn build_build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(world_gen_system())
        .add_system(builder_render_system())
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
