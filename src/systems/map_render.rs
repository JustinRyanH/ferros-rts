use crate::prelude::*;

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..=camera.right_x {
            let point = Point { x, y };
            let offset = camera.top_left_corner();
            if let Some(tile) = map.get_tile(point) {
                draw_batch.set(point - offset, ColorPair::new(WHITE, BLACK), tile);
            }
        }
    }
    draw_batch.submit(0).expect("Batch Error");
}
