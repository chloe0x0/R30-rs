mod r30;
use r30::{DEB, R30};
use std::time::SystemTime;

fn main() {
    let mut seed: u64 = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Duraction since UNIX_EPOCH Failed!!!")
        .as_secs();
    seed *= seed; // square the time
    let mut rng: R30 = R30::new(seed);
    print!("1\t\t|");
    rng.print();
    for _n in 0..=50 {
        let bit: u64 = rng.rand_bit() as u64;
        print!("{}\t\t|", bit);
        rng.print();
    }

    for n in 0..50 {
        if (n + 1) % 5 == 0 {
            println!("{}", rng.rand_u64(5));
        } else {
            print!("{}, ", rng.rand_u64(5));
        }
    }
}
