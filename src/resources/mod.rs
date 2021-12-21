use crate::prelude::*;

pub struct ProgressBar {
    pub total: i32,
    pub current: i32,
    pub label: Option<&'static str>,
    pub y: i32,
}

impl ProgressBar {
    pub const HEIGHT: i32 = 7;
    pub const WIDTH: i32 = 70;
    pub fn new(y: i32) -> Self {
        Self {
            total: 1,
            current: 0,
            label: None,
            y,
        }
    }
}

pub struct WorldGenRng(RandomNumberGenerator);

impl WorldGenRng {
    pub fn new() -> Self {
        Self(RandomNumberGenerator::new())
    }
}

impl Default for WorldGenRng {
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::Deref for WorldGenRng {
    type Target = RandomNumberGenerator;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for WorldGenRng {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct InWorldCamera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl InWorldCamera {
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
        Point::new(self.left_x.max(0), self.top_y)
    }
}

impl<'a> IntoIterator for &'a InWorldCamera {
    type Item = Point;

    type IntoIter = CameraIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let point = Point::new(self.left_x.max(0), self.top_y);
        CameraIterator {
            camera: self,
            point,
        }
    }
}

pub struct CameraIterator<'a> {
    pub camera: &'a InWorldCamera,
    pub point: Point,
}

impl<'a> CameraIterator<'a> {
    fn step(&mut self) {
        self.point.x += 1;
        if self.point.x > self.x_limit() {
            self.point.x = self.camera.left_x.max(0);
            self.point.y += 1;
        }
    }

    fn x_limit(&mut self) -> i32 {
        if self.camera.left_x < 0 {
            return self.camera.right_x + self.camera.left_x.abs();
        }
        self.camera.right_x
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
