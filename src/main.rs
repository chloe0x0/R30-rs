mod R30;
use R30::{r30, DEB};

fn main()
{
    let mut rng: r30 = r30::new(DEB);
    print!("1\t\t|");
    rng.Print();
    for n in 0..=50
    {
        let bit: u64 = rng.RandBit() as u64;
        print!("{}\t\t|", bit);
        rng.Print();
    }
}
