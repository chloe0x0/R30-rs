mod R30;
use R30::{r30, DEB};
use std::time::SystemTime;

fn main()
{
    let seed: u64 = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Duraction since UNIX_EPOCH Failed!!!")
        .as_secs();
    let mut rng: r30 = r30::new(seed);
    print!("1\t\t|");
    rng.Print();
    for n in 0..=50
    {
        let bit: u64 = rng.RandBit() as u64;
        print!("{}\t\t|", bit);
        rng.Print();
    }

    for n in 0..=5
    {
        print!("{}, ", rng.Rand64(10));
    }
}
