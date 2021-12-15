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