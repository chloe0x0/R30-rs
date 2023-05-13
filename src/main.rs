use std::time;
use r30_rs::*;

fn main() {
    let mut gen = R30::default();

    let start = time::Instant::now();
    {
        let mut v: Vec<u64> = Vec::with_capacity(1e6 as usize);

        for _ in 0..1e6 as u64 {
            v.push(gen.next_u64());
        }
    }
    let end = start.elapsed();

    println!("{:?}", end);
}