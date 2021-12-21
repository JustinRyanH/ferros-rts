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

    pub fn update(&mut self, Point { x, y }: Point) {
        self.left_x = x - DIMENSION_WIDTH;
        self.right_x = x + DIMENSION_WIDTH;
        self.top_y = y - DIMENSION_HEIGHT;
        self.bottom_y = y + DIMENSION_HEIGHT;
    }

    pub fn top_left_corner(&self) -> Point {
        Point::new(self.left_x, self.top_y)
    }
}

impl<'a> IntoIterator for &'a Camera {
    type Item = Point;

    type IntoIter = CameraIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let point = self.top_left_corner();
        CameraIterator {
            camera: self,
            point,
        }
    }
}

pub struct CameraIterator<'a> {
    pub camera: &'a Camera,
    pub point: Point,
}

impl<'a> CameraIterator<'a> {
    fn step(&mut self) {
        self.point.x += 1;
        if self.point.x > self.camera.right_x {
            self.point.x = self.camera.left_x;
            self.point.y += 1;
        }
    }

    fn is_out_of_bounds(&mut self) -> bool {
        self.point.y > self.camera.bottom_y
    }
}

impl<'a> Iterator for CameraIterator<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let out = Some(self.point);
        self.step();
        if self.is_out_of_bounds() {
            return None;
        }
        out
    }
}
