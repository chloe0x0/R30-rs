use std::time::SystemTime;

// Establish some constants
pub const DEB: u64 = (1 as u64) << 31; // use for debugging
const CELL_STR: &str = "\u{2588}"; // what to print for active cells when converting the state ot a String`

// An entire PRNG in a single 64 bit word....
pub struct R30 {
    state: u64,
}

impl R30 {
    pub fn new(seed: u64) -> Self {
        R30 { state: seed }
    }
    pub fn from_time() -> Self {
        // get current system time and square it
        let mut seed: u64 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Duraction since UNIX_EPOCH Failed!!!")
            .as_secs();
        seed *= seed; // square the time

        R30 { state: seed }
    }
    fn iterate(&mut self) {
        // pre-evolve the edge bit to avoid overflow on the most significant bit (leads to the state becoming u64 max)
        let n: u64 =
            (self.state & (1 as u64)) << 63 ^ (self.state & ((1 as u64) << 63)) | (self.state >> 1);
        self.state = (self.state & !((1 as u64) << 63)) | (n << 63);
        self.state = (self.state >> 1) ^ (self.state | (self.state << 1));
    }
    pub fn rand_bit(&mut self) -> bool {
        let bit: bool = self.state & ((1 as u64) << 31) != 0;
        self.iterate();
        return bit;
    }
    pub fn rand_u64_in(&mut self, a: u64, b: u64) -> u64 {
        let y: u64 = self.rand_u64(64);
        return y % (b + 1 - a) + a;
    }
    pub fn rand_u64(&mut self, bits: usize) -> u64 {
        let mut y: u64 = 0 as u64;

        for n in 0..bits {
            let bit: bool = self.rand_bit();
            y = (y & !((1 as u64) << n)) | ((bit as u64) << n);
        }
        self.iterate();
        return y;
    }
    pub fn to_string(&self) -> String {
        let mut s = String::new();

        let mut i = 63;
        while i >= 0 {
            let n = (self.state & ((1 as u64) << i) != 0) as u64;
            s += if n != 0 { CELL_STR } else { " " };
            i -= 1;
        }

        return s;
    }
}
