mod r30;
use r30::{DEB, R30};
use std::time::SystemTime;

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
