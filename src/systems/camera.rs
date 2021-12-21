use crate::prelude::*;

#[system]
pub fn keep_camera_on_map(#[resource] map: &Map, #[resource] camera: &mut Camera) {
    let right_most_allowed_point = map.width - SCREEN_WIDTH;
    let bottom_most_allowed_point = map.height - SCREEN_HEIGHT;
    let new_x = camera.left_x.max(0).min(right_most_allowed_point);
    let new_y = camera.top_y.max(0).min(bottom_most_allowed_point);
    camera.set_top_left_corner(Point::new(new_x, new_y));
}
