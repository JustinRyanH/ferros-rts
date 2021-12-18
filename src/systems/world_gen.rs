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
