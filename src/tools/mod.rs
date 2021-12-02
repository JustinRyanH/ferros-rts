use crate::prelude::Point;

pub struct PointLine {
    pub max: i32,
    pub current: i32,
    pub static_el: i32,
    pub static_first: bool,
}

impl Iterator for PointLine {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.max {
            return None;
        }
        self.current += 1;
        if self.static_first {
            Some(Point::new(self.static_el, self.current))
        } else {
            Some(Point::new(self.current, self.static_el))
        }
    }
}
