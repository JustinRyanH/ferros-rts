pub struct StaticPairIterator {
    pub max: i32,
    pub current: i32,
    pub static_el: i32,
    pub static_first: bool,
}

impl Iterator for StaticPairIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.max {
            return None;
        }
        self.current += 1;
        if self.static_first {
            Some((self.static_el, self.current))
        } else {
            Some((self.current, self.static_el))
        }
    }
}
