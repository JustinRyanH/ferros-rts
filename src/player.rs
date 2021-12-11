use crate::prelude::*;

pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    pub fn new(Point { x, y }: Point) -> Self {
        Self {
            left_x: x - DIMENSION_WIDTH,
            right_x: x + DIMENSION_WIDTH,
            top_y: y - DIMENSION_HEIGHT,
            bottom_y: y + DIMENSION_HEIGHT,
        }
    }

    pub fn on_player_move(&mut self, Point { x, y }: Point) {
        self.left_x = x - DIMENSION_WIDTH;
        self.right_x = x + DIMENSION_WIDTH;
        self.top_y = y - DIMENSION_HEIGHT;
        self.bottom_y = y + DIMENSION_HEIGHT;
    }

    pub fn top_left_corner(&self) -> Point {
        Point::new(self.left_x, self.top_y)
    }
}
