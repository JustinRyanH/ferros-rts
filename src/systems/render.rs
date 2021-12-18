use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = camera.top_left_corner();

    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });

    draw_batch.submit(5000).expect("Batch Error");
}

#[system]
pub fn map(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..=camera.right_x {
            let point = Point { x, y };
            let offset = camera.top_left_corner();
            if let Some(tile) = map.get_tile(point) {
                draw_batch.set(point - offset, tile.into(), tile);
            }
        }
    }
    draw_batch.submit(0).expect("Batch Error");
}

#[system]
pub fn builder(#[resource] builder: &mut MapBuilder) {
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
