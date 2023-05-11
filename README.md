# R30-rs
A psuedo-random bit generator in Rust implemented with the Rule-30 elementary cellular automata

                                █
                               ███
                              ██  █
                             ██ ████
                            ██  █   █
                           ██ ████ ███
                          ██  █    █  █
                         ██ ████  ██████
                        ██  █   ███     █
                       ██ ████ ██  █   ███
                      ██  █    █ ████ ██  █
                     ██ ████  ██ █    █ ████
                    ██  █   ███  ██  ██ █   █
                   ██ ████ ██  ███ ███  ██ ███
                  ██  █    █ ███   █  ███  █  █
                 ██ ████  ██ █  █ █████  ███████

It should be noted that R30 is NOT a cryptographically secure PRNG, though it is very suitable for simulations.

Compared to other generators R30 has a small state of only 32 bits. MT19937 requires about 2.5 KiB. R30 even beats the TinyMT variant which has a state size of 128 bits. For this reason, R30-32 is particularly well suited for systems where memory is limited. 

## Seeding the generator

Using an explicit u32 seed
```Rust
let mut gen: R30 = R30::new(69u32);
```

Using the Default trait
(the time since the UNIX_EPOCH will be computed in seconds and squared)
```Rust
let mut gen: R30 = R30::default();
```

If you only want the middle cell to be 1, simply use the center() trait
```Rust
let mut gen: R30 = R30::center();
// Equivalent to R30::new(1 << 16)
```

## Using the generator
The R30 struct implements traits for generating u32, u64, i32, i64, f32, f64, and bool types, as well as support for generating u32 and u64 types within an interval [a, b], and uniformly sampling from a vector.

for example, to generate a random boolean
```Rust
use r30_rs::*;

fn main() {    
    let mut gen = R30::default();

    if gen.next_bool() {
        println!("uwu");
    } else {
        println!("owo");
    }
}
```

to generate a u32
```Rust
let num: u32 = gen.next_u32();
```
u64, i32, i64, f32, and f64 types are generated similarly 
(next_<type_name>)

for generating a u32 or u64 in the closed interval [a, b]
```Rust
// Roll a 6 sided die
let num: u32 = gen.next_u32_in(1, 6);
// Roll a D20
let roll: u64 = gen.next_u64_in(1, 20);
```

to uniformly sample an element from a Vec\<T>
```Rust
fn main() {
    let v = vec!["owo", "uwu", "OwO", "UwU", "() W ()"];
    println!("{}", gen.rand_choice(&v));
}
```

## Testing the generator
To test that everything works, simply run
```console
cargo test --release
```