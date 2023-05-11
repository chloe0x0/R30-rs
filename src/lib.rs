mod r30;
pub use r30::*;

#[cfg(test)]
mod tests {
    use crate::*;

    /// Assert that the Automata behaves as expected
    #[test]
    fn test_automata_behavior() {
        // Start with only the center cell active
        let mut gen = R30::center();
        // 35 precomputed values
        const EXPECTED: [u32; 35] = [
            65536, 229376, 311296, 1007616, 1126400, 3921920, 4738048, 16578048, 17240832,
            59358592, 81691840, 256036704, 291926320, 993762264, 1237903436, 4269616374, 59107090,
            80492991, 2406895233, 3507745474, 1539055206, 3364234171, 2110485640, 2222528476,
            3465523271, 1926273256, 2656693548, 3821938151, 879983672, 1453131340, 3538258934,
            1581045778, 3276229695, 1701410368, 3173979104,
        ];

        for i in 0..EXPECTED.len() {
            assert_eq!(gen.state, EXPECTED[i]);
            gen.evolve();
        }
    }

    /// Monobit frequency test for default implementation
    #[test]
    fn monobit_freq() {
        let mut gen = R30::default();
        let mut zero: u64 = 0;
        let mut one: u64 = 0;

        const BITS: u64 = 1e6 as u64;
        const EPSILON: f64 = 1e-3;

        for _ in 0..BITS {
            let bit = gen.next_bool();
            zero += !bit as u64;
            one += bit as u64;
        }

        // Compute P(0) and P(1)
        let p0 = zero as f64 / BITS as f64;
        let p1 = one as f64 / BITS as f64;
        // Get their distances from the Expected value of 0.5
        let d0 = (p0 - 0.5).abs();
        let d1 = (p1 - 0.5).abs();
        // If either is too large the test fails
        assert!(d0 <= EPSILON);
        assert!(d1 <= EPSILON);
    }

    /// Assert that values generated in [a, b] stay in [a, b]
    #[test]
    fn test_u64_range() {
        let mut gen = R30::default();

        const a: u64 = 1;
        const b: u64 = 100;

        const RUNS: u64 = 1e6 as u64;

        for _ in 0..=RUNS {
            let y = gen.next_u64_in(a, b);
            assert!(y >= a && y <= b);
        }
    }
    

}
