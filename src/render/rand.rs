pub struct Rnd {
    Seed: u32,
}

impl Rnd {
    const OFFSET: u32 = 0x5414d5f4;

    #[inline]
    pub fn new(seed: u32) -> Self {
        Self {
            Seed: seed ^ Rnd::OFFSET,
        }
    }

    #[inline]
    pub fn new_randomized() -> Self {
        Self {
            Seed: Rnd::OFFSET.wrapping_mul(Rnd::generate_time_seed()),
        }
    }

    fn generate_time_seed() -> u32 {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now();
        let since_epoch = now.duration_since(UNIX_EPOCH);
        match since_epoch {
            Ok(d) => {
                let ms: u128 = d.as_micros();
                let a = (ms >> 96) as u32;
                let b = ((ms >> 64) & 0xffffffff) as u32;
                let c = ((ms >> 32) & 0xffffffff) as u32;
                let d = ms as u32;
                a ^ b ^ c ^ d
            }
            _ => Rnd::OFFSET,
        }
    }

    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        let mut random: u32 = self.Seed;
        random ^= random << 13;
        random ^= random >> 17;
        random ^= random << 5;
        self.Seed ^= random;
        random
    }

    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        let a: u64 = self.next_u32() as u64;
        let b: u64 = self.next_u32() as u64;
        a | (b << 32)
    }

    #[inline]
    pub fn next_i64(&mut self) -> i64 {
        self.next_u64() as i64
    }

    #[inline]
    pub fn next_i32(&mut self) -> i32 {
        self.next_u32() as i32
    }

    #[inline]
    pub fn next_u64_range(&mut self, min: u64, max: u64) -> u64 {
        if max > min {
            let range = max - min;
            min + (self.next_u64() % range)
        } else {
            min
        }
    }

    #[inline]
    pub fn next_u32_range(&mut self, min: u32, max: u32) -> u32 {
        if max > min {
            let range = max - min;
            min + (self.next_u32() % range)
        } else {
            min
        }
    }

    #[inline]
    pub fn next_i64_range(&mut self, min: i64, max: i64) -> i64 {
        if max > min {
            let range = (max - min) as u64;
            let r = self.next_u64() % range;
            (r as i64) + min
        } else {
            min
        }
    }

    #[inline]
    pub fn next_i32_range(&mut self, min: i32, max: i32) -> i32 {
        if max > min {
            let range = (max - min) as u32;
            let r = self.next_u32() % range;
            (r as i32) + min
        } else {
            min
        }
    }
}
