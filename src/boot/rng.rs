/// Simple xorshift32 RNG — no external crate needed.
pub struct Rng {
    state: u32,
}

impl Rng {
    pub fn new(seed: u32) -> Self {
        Self {
            state: if seed == 0 { 1 } else { seed },
        }
    }

    pub fn next_u32(&mut self) -> u32 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 17;
        self.state ^= self.state << 5;
        self.state
    }

    pub fn range(&mut self, max: u32) -> u32 {
        self.next_u32() % max
    }

    pub fn float(&mut self) -> f32 {
        (self.next_u32() & 0xFFFF) as f32 / 65536.0
    }
}
