// pub const BUILD_BUDGET: std::time::Duration = std::time::Duration::from_micros(1);

use crate::prelude::*;

pub struct MapBuilderState {
    pub generator: GeneraotrRunner,
    pub builder: MapBuilder,
}

impl MapBuilderState {
    pub fn build_world(&mut self, rng: &mut RandomNumberGenerator) {
        self.generator.next(&mut self.builder, rng);
    }

    pub fn is_finished(&self) -> bool {
        self.generator.is_finished()
    }
}

impl Default for MapBuilderState {
    fn default() -> Self {
        Self {
            builder: MapBuilder::new(SCREEN_WIDTH, SCREEN_HEIGHT, 10),
            generator: Default::default(),
        }
    }
}
