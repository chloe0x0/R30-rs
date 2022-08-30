mod r30;
use r30::{R30, DEB};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bit_uniformity() {
        // test that the generated distribution of bits in the center cell is uniformly distributed along the unit interval [0, 1]
        let mut distribution: [usize; 2] = [0, 0]; 
        let mut rng: R30 = R30::from_time();
        let K: usize = 50000;

        for _n in 0..=K {
            distribution[rng.rand_bit() as usize] += 1;
        }

        // assert the uniformity 
        assert_eq!((distribution[0] / K), (1 / 2));
        assert_eq!((distribution[1] / K), (1 / 2));
    }
    #[test]
    fn test_u64_properties() {
        // test the statistical properties of 64 bit words in the range [0, K] 
        const K: u64 = 100;
        let mut rng: R30 = R30::from_time();
        let mut distribution: [u64; (K+1) as usize] = [0; (K+1) as usize];

        const N: usize = 50000;
        for _n in 0..N {
            distribution[rng.rand_u64_in(0, K) as usize] += 1;
        }

        // can assert that P(x) = 1 / N for all x in the distribution
        // validate some statistical properties of the distribution
        // Mean of a uniform distribution is (a + b) / 2
        // In our case: b = 0, so the mean should be about K / 2
        let mut mean: f64 = 0.0;
        for n in distribution {
            mean += (n as f64);
            assert!(( ( (n as f64) / (N as f64) ) as f64 - (1.0 / (N as f64) ) as f64 ) <= 0.05 );
        }
        mean /= (N as f64);
        assert!(mean - (K as f64 / 2.0) <= 0.00005);

        // Compute the variance
        let mut var_: f64 = 0.0;
        for n in distribution {
            var_ += ((n as f64 - mean) * (n as f64 - mean)) as f64;
        }
        let var: f64 = (K * K) as f64 / 12.0;
        assert!(var - var_ <= 0.005);
    }
}

fn main() {
    let mut rng: R30 = R30::from_time();

    for i in 0..50 {
        let n = rng.rand_u64_in(0, 100);
        if (i + 1) % 5 == 0 {
            println!("{}", n);
        } else {
            print!("{}, ", n);
        }
    }
}
