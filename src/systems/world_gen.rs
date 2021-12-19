use crate::prelude::*;

#[system]
pub fn world_gen(
    #[resource] rng: &mut WorldGenRng,
    #[resource] generator: &mut GeneraotrRunner,
    #[resource] builder: &mut MapBuilder,
) {
    if generator.is_finished() {
        return;
    }
    generator.next(builder, rng);
}

#[system]
pub fn finish_world_gen(
    #[resource] rng: &mut WorldGenRng,
    #[resource] generator: &mut GeneraotrRunner,
    #[resource] builder: &mut MapBuilder,
    commands: &mut CommandBuffer,
) {
    if !generator.is_finished() {
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
pub fn world_gen_progress(
    #[resource] bar: &mut Option<ProgressBar>,
    #[resource] runner: &GeneraotrRunner,
) {
    if let Some(progress) = bar {
        progress.total = runner.total_steps();
        progress.current = runner.current_step();
        progress.label = runner.get_render_text();
    }
}
