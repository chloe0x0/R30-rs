pub const DEB: u64 = (1 as u64) << 31;
const CELL_STR: &str = "\u{2588}";

pub struct r30
{
    state: u64
}

impl r30
{
    pub fn new(seed: u64) -> Self
    {
        r30 { state: seed }
    }
    fn Iterate(&mut self)
    {
        // pre-evolve the edge bit to avoid overflow on the most significant bit (leads to the state becoming u64 max)
        let n: u64 = (self.state & ((1 as u64)))<<63 ^ (self.state & ((1 as u64) << 63)) | (self.state >> 1);
        self.state = (self.state & !((1 as u64) << 63)) | (n << 63);
        self.state = (self.state >> 1) ^ (self.state | (self.state << 1));
    }
    pub fn RandBit(&mut self) -> bool
    {
        let bit: bool = self.state & ((1 as u64) << 31) != 0;
        self.Iterate();
        return bit;
    }
    pub fn Rand64(&mut self, bits: usize) -> u64
    {
        let mut y: u64 = 0 as u64;

        for n in 0..bits
        {
            let bit: bool = self.RandBit();
            y = (y & !((1 as u64) << n)) | ((bit as u64) << n);
        }
        self.Iterate();
        return y;
    }
    pub fn Print(&self)
    {
        let mut string = String::new();

        let mut i = 63;
        while i >= 0
        {
            let n = (self.state & ((1 as u64) << i) != 0) as u64;
            if n != 0
            {
                string = string + CELL_STR;
            }
            else
            {
                string = string + " ";
            }
            i = i - 1;
        }
        println!("{}", string);
    }
}

fn main()
{
    let mut prng: r30 = r30::new(DEB);
    prng.Print();
    for i in 0..=100
    {
       prng.Iterate();
       prng.Print();
    }
}