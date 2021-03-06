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
    let offset = camera.top_left_corner();
    camera.into_iter().for_each(|point| {
        if let Some(tile) = map.get_tile(point) {
            draw_batch.set(point - offset, tile.into(), tile);
        }
    });
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
        let x = room.x1 - camera_offset.x;
        let y = room.y1 - camera_offset.y;
        let with_size = Rect::with_size(x, y, room.width(), room.height());
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

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn hud(ecs: &SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let Health { current, max } = health_query.iter(ecs).next().unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    draw_batch.print_color_centered(
        1,
        "Explore the Dungeon. Cursor keys to move.",
        ColorPair::new(WHITE, BLACK),
    );
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH,
        *current,
        *max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.submit(UI_LAYER + 10).expect("Batch Error");
}

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
pub fn tooltips(ecs: &SubWorld, #[resource] mouse_pos: &MousePoint, #[resource] camera: &Camera) {
    let offset = camera.top_left_corner();
    let map_pos = mouse_pos.0 + offset;

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);

    for (entity, _, name) in <(Entity, &Point, &Name)>::query()
        .iter(ecs)
        .filter(|(_, pos, _)| **pos == map_pos)
    {
        let mut screen_pos = mouse_pos.0;
        if screen_pos.y > SCREEN_HEIGHT / 2 {
            screen_pos.y -= 1;
        } else {
            screen_pos.y += 1;
        }

        let display = if let Ok(health) = ecs.entry_ref(*entity).unwrap().get_component::<Health>()
        {
            format!("{} : {} hp", &name.0, health.current)
        } else {
            name.0.clone()
        };
        draw_batch.print_centered_at(screen_pos, &display);
    }

    draw_batch.submit(UI_LAYER).expect("Batch Error");
}
