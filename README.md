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

## Usage

Instantiation

With an explicit u64 seed
```rust
fn main() {
    let mut rng: r30 = R30::new(69);
}
```

seeing that R30 is a deterministic system (it will produce the same sequence given the same seed) it may be desirable to use the system time as a seed

The API makes this trivial
```rust
fn main() {
    let mut rng = R30::from_time();
}
```
the above will square the system time and use it as the seed

The power of the R30 generator lies in the fact that the distribution of 0 and 1 cells in its center column is uniform

to sample a random bit from the current state
```rust
fn main() {
    let mut rng = R30::from_time();

    let bit: bool = rng.rand_bit(); // 1 or 0
}
```

to sample N bits as the basis for a 64 bit word
```rust
fn main() {
    let mut rng = R30::from_time();
    
    let x: u64 = rng.rand_u64(5); // sample 5 bits and fill a u64 with them
}
```

to generate a 64 bit word in the interval [a, b]
```rust
fn main() {
    let mut rng = R30::from_time();

    let a: u64 = 0;
    let b: u64 = 10;
    let x: u64 = rng.rand_u64_in(a, b);
}
```

to uniformly sample an element from a Vec\<T>
```rust
fn main() {
    let mut rng = R30::from_time();

    let names: Vec<&str> = vec!["Chloe", "Gatsby", "Kafka", "Tori"];
    println!("I love {}!", rng.rand_choice(&names));

    let nums: Vec<i32> = vec![0, 1, 2, 3];
    let x: &i32 = rng.rand_choice(&nums);
}
```