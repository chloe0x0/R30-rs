mod r30;
use r30::{DEB, R30};
use std::time::{SystemTime, Instant, Duration};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bit_uniformity() {
        // test that the generated distribution of bits in the center cell is uniformly distributed along the unit interval [0, 1]
        let mut distribution: [usize; 2] = [0, 0];
        let mut rng: R30 = R30::from_time();
        const K: usize = 50000;

        for _n in 0..=K {
            distribution[rng.rand_bit() as usize] += 1;
        }

        // assert the uniformity
        assert_eq!((distribution[0] / K), (1 / 2));
        assert_eq!((distribution[1] / K), (1 / 2));
    }
    #[test]
    fn u64_uniformity() {
        // test the uniformity of N generated 64 bit words in the interval [0, K]
        const K: u64 = 10000;
        let mut rng: R30 = R30::from_time();
        let mut distribution: [u8; (K + 1) as usize] = [0; (K + 1) as usize];

        const N: usize = 50000;
        for _n in 0..N {
            distribution[rng.rand_u64_in(0, K) as usize] += 1;
        }

        // can assert that P(x) ~ 1 / N for all x in the distribution
        for n in distribution {
            assert!((((n as f64) / (N as f64)) as f64 - (1.0 / (N as f64)) as f64) <= 0.05);
        }
    }
    #[test]
    fn speed_test() {
        let mut rng = R30::from_time();

        let start = Instant::now();
        for n in 0..10000000 {
            let bit = rng.rand_bit();
        }
        let end = start.elapsed().as_secs();

        assert!(end < 1);
    }
}

fn main() {
    let mut rng: R30 = R30::from_time();

    // print matrix of pseudorandom integers in the interval [0, 100]
    for i in 0..50 {
        let n = rng.rand_u64_in(0, 100);
        if (i + 1) % 5 == 0 {
            println!("{}", n);
        } else {
            print!("{}, ", n);
        }
    }

    // Coin flip
    if rng.rand_bit() {
        println!("Tails!");
    } else {
        println!("Heads!");
    }

    // fair 6 sided dice roll
    println!("Rolled a {}!", rng.rand_u64_in(1, 6));

    for _n in 0..=10000000 {
        let bit: bool = rng.rand_bit();
    }

    let v: Vec<i32> = vec![0; 10];
    let choice: &i32 = rng.rand_choice(&v);
}
