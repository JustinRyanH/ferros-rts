use crate::prelude::*;

pub fn clear_batch(draw: &mut DrawBatch) {
    draw.target(0);
    draw.cls();
    draw.target(1);
    draw.cls();
}

pub fn submit_batch(ctx: &mut BTerm, draw: &mut DrawBatch) -> BError {
    draw.submit(0)?;
    render_draw_buffer(ctx)
}

pub struct PointLine {
    pub max: i32,
    pub current: i32,
    pub static_el: i32,
    pub static_first: bool,
}

impl PointLine {
    fn get_new_point(&self) -> Point {
        match self.static_first {
            true => Point::new(self.static_el, self.current),
            false => Point::new(self.current, self.static_el),
        }
    }
}

impl Iterator for PointLine {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.max {
            return None;
        }
        let new_point = self.get_new_point();
        self.current += 1;
        Some(new_point)
    }
}
