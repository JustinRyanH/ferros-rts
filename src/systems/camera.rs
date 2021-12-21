use crate::prelude::*;

#[system]
pub fn keep_camera_on_map(#[resource] map: &Map, #[resource] camera: &mut Camera) {
    let mut new_x = camera.left_x.max(0);
    if new_x + SCREEN_WIDTH > map.width {
        new_x = map.width - SCREEN_WIDTH;
    }
    let mut new_y = camera.top_y.max(0);
    if new_y + SCREEN_HEIGHT > map.height {
        new_y = map.height - SCREEN_HEIGHT;
    }

    camera.set_top_left_corner(Point::new(new_x, new_y));
}
