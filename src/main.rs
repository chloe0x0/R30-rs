mod R30;
use R30::r30;

fn main() {
    let mut rng: r30 = r30::new(0 as u64);
    rng.Print();
}
