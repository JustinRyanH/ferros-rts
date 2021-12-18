use crate::prelude::*;

pub struct ProgressBar {
    pub total: i32,
    pub current: i32,
    pub y: i32,
}

impl ProgressBar {
    pub const HEIGHT: i32 = 7;
    pub const WIDTH: i32 = 70;
    pub fn new(y: i32) -> Self {
        Self {
            total: 1,
            current: 0,
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
