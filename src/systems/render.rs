use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn characters(ecs: &SubWorld, #[resource] camera: &Camera) {
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
pub fn builder(#[resource] builder: &mut MapBuilder, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    let camera_offset = camera.top_left_corner();
    for tile in builder.fill_tile.iter() {
        let region = Rect::with_size(0, 0, builder.width, builder.height);
        draw_batch.fill_region(region, ColorPair::new(YELLOW, BLACK), *tile);
    }
    for room in builder.rooms.iter() {
        let with_size = Rect::with_size(
            room.x1 - camera_offset.x,
            room.y1 - camera_offset.y,
            room.width(),
            room.height(),
        );
        println!("Rect: {:?}", with_size);
        println!("Room: {:?}", room);
        draw_batch.fill_region(with_size, ColorPair::new(RED, BLACK), TileType::Floor);
    }
    for tunnel in builder.tunnels.iter() {
        tunnel.into_iter().for_each(|point| {
            draw_batch.set(
                point - camera_offset,
                ColorPair::new(CYAN, BLACK),
                TileType::Floor,
            );
        });
    }
    for player in builder.player.iter() {
        draw_batch.set(
            *player - camera_offset,
            ColorPair::new(GREEN, BLACK),
            to_cp437('@'),
        );
    }
    draw_batch.submit(0).expect("Batch Error");
}

#[system]
pub fn progress_bar(#[resource] progress_bar: &Option<ProgressBar>) {
    if let Some(ProgressBar {
        current,
        total,
        y,
        label,
    }) = progress_bar
    {
        let mut draw_batch = DrawBatch::new();
        draw_batch.target(1);

        let pos = Rect::with_size(4, *y, ProgressBar::WIDTH + 1, 2);

        draw_batch.draw_double_box(pos, ColorPair::new(YELLOW, BLACK));
        draw_batch.bar_horizontal(
            Point::new(pos.x1 + 1, y + 1),
            ProgressBar::WIDTH,
            *current,
            *total,
            ColorPair::new(WHITE, BLACK),
        );

        if let Some(label) = label {
            draw_batch.print_color_centered(*y, label, ColorPair::new(WHITE, BLACK));
        }

        draw_batch.submit(UI_LAYER + 10).expect("Batch Error");
    }
}
