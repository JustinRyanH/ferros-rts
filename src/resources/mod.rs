use bracket_lib::prelude::RandomNumberGenerator;

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
