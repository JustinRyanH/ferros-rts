use crate::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Player {
    pub position: Point,
    pub color: ColorPair,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            position: Point::new(x, y),
            color: ColorPair::new(GREEN, BLACK),
        }
    }

    pub fn new_position(&mut self, delta: Point) -> Point {
        self.position + delta
    }

    pub fn move_position(&mut self, delta: Point) {
        self.position += delta;
    }

    pub fn render(&self, draw: &mut DrawBatch) {
        draw.set(self.position, self.color, to_cp437('@'));
    }
}
