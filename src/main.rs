use r30_rs::*;

fn main() {
    let mut gen = R30::default();

    let v = vec!["owo", "uwu", "OwO", "UwU", "() W ()"];
    println!("{}", gen.rand_choice(&v));
}