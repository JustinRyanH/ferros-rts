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

    // pub fn update(&mut self, ctx: &mut BTerm) {
    //     if let Some(key) = ctx.key {
    //         let delta = match key {
    //             VirtualKeyCode::Left => Point::new(-1, 0),
    //             VirtualKeyCode::Right => Point::new(1, 0),
    //             VirtualKeyCode::Up => Point::new(0, -1),
    //             VirtualKeyCode::Down => Point::new(0, 1),
    //             _ => Point::new(0, 0)
    //         };
    //         self.position += delta;
    //     }
    // }

    pub fn render(&self, draw: &mut DrawBatch) {
        draw.set(self.position, self.color, to_cp437('@'));
    }
}
