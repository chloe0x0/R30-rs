mod r30;
use r30::{DEB, R30};
use std::time::SystemTime;

fn main() {
    let mut rng: R30 = R30::from_time();
    print!("1\t\t|");
    println!("{}", rng.to_string());
    for _n in 0..=50 {
        let bit: u64 = rng.rand_bit() as u64;
        print!("{}\t\t|", bit);
        println!("{}", rng.to_string());
    }

    for n in 0..50 {
        if (n + 1) % 5 == 0 {
            println!("{}", rng.rand_u64(5));
        } else {
            print!("{}, ", rng.rand_u64(5));
        }
    }
}
