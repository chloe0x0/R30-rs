use rand::RngCore;
use std::time::SystemTime;
use std::{fmt::Display, time::UNIX_EPOCH};

///
/// Implementation of a PRNG based off of the Rule-30 Elementary Cellular Automata
/// Implemented to be FAST and small on memory
///
/// Bits are sampled from the center cell.
/// A bit is sampled from the center, and the automata is iterated
/// This process is repeated N times to get a bit-string of N bits. This can be used to create 32, 64, 128, ... , unsigned ints, signed ints, floats, etc etc
///
/// The generated bit strings should be distributed uniformly, as the distribution of bits in the center cell is uniform (from experimentation)
///

/// Mask used to extract middle bit
const MASK: u32 = 1 << 16;

/// A Rule30 CA based PRNG
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct R30 {
    /// The current state of the Automata (a bit-string of len 32)
    pub state: u32,
}

impl R30 {
    #[inline(always)]
    pub fn new(seed: u32) -> Self {
        R30 { state: seed }
    }

    #[inline(always)]
    pub fn center() -> Self {
        R30 { state: MASK }
    }

    /// Update the current state of the automata according to Rule-30
    #[inline(always)]
    pub fn evolve(&mut self) {
        let left = self.state.rotate_left(1);
        let right = self.state.rotate_right(1);
        self.state = left ^ (self.state | right);
    }

    /// Sample a bit from the center cell as a boolean
    #[inline(always)]
    pub fn next_bool(&mut self) -> bool {
        let bit: bool = (self.state & MASK) != 0;
        self.evolve();
        bit
    }

    /// Sample 32 bits from the center of the automata (done by iterating the automata 32 times) and interpret the resultant bitstring as u32
    #[inline(always)]
    pub fn next_u32(&mut self) -> u32 {
        let mut bits = 0u32;

        for i in 0..32 {
            let bit = self.next_bool() as u32;
            bits |= bit << i;
        }

        bits
    }

    /// Sample a u64 using the same method as next_u32
    #[inline(always)]
    pub fn next_u64(&mut self) -> u64 {
        let mut bits = 0u64;

        for i in 0..64 {
            let bit = self.next_bool() as u64;
            bits |= bit << i;
        }

        bits
    }

    /// Sample a u8
    #[inline(always)]
    pub fn next_u8(&mut self) -> u8 {
        let mut bits = 0u8;

        for i in 0..8 {
            let bit = self.next_bool() as u8;
            bits |= bit << i;
        }

        bits
    }

    /// Sample an i32
    #[inline(always)]
    pub fn next_i32(&mut self) -> i32 {
        self.next_u32() as i32
    }

    /// Sample an i64
    #[inline(always)]
    pub fn next_i64(&mut self) -> i64 {
        self.next_u64() as i64
    }

    /// Sample an f32 in the interval [0, 1)
    #[inline(always)]
    pub fn next_f32(&mut self) -> f32 {
        self.next_u32() as f32 / u32::MAX as f32
    }

    /// Sample an f64 in the interval [0, 1)
    #[inline(always)]
    pub fn next_f64(&mut self) -> f64 {
        self.next_u64() as f64 / u64::MAX as f64
    }

    /// Generate a u64 in [a, b]
    #[inline(always)]
    pub fn next_u64_in(&mut self, a: u64, b: u64) -> u64 {
        let y: u64 = self.next_u64();
        return y % (b + 1 - a) + a;
    }

    /// Generate a u32 in [a, b]
    #[inline(always)]
    pub fn next_u32_in(&mut self, a: u32, b: u32) -> u32 {
        let y: u32 = self.next_u32();
        return y % (b + 1 - a) + a;
    }

    /// Uniformly sample from a Vec<T>
    pub fn rand_choice<'a, T>(&'a mut self, xs: &'a Vec<T>) -> &T {
        let ix: usize = self.next_u64_in(0, xs.len() as u64 - 1) as usize;
        return &xs[ix];
    }

}

impl Default for R30 {
    /// Use the square of the system time as the seed
    fn default() -> Self {
        let mut time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Fucky wucky in computing distance since UNIX_EPOCH UWU")
            .as_secs();
        // Square the time to distribute the bits more uniformly throughout
        time *= time;

        R30 { state: time as u32 }
    }
}

impl RngCore for R30 {
    fn next_u32(&mut self) -> u32 {
        self.next_u32()
    }
    fn next_u64(&mut self) -> u64 {
        self.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for byte in dest {
            *byte = self.next_u8();
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        Ok(self.fill_bytes(dest))
    }
}

impl Display for R30 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.state)
    }
}
