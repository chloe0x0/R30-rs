const DEB: u64 = (1 as u64) << 31;


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
        let n: u64 = (self.state & ((1 as u64)) << 63) ^ (self.state & ((1 as u64) << 63)) | ((self.state & ((1 as u64) << 62)) << 63);
        self.state = (self.state & !((1 as u64) << 63)) | ((n as u64) << 63);
        self.state = (self.state >> 1) ^ (self.state | (self.state << 1));
    }
    pub fn RandBit(&mut self) -> u64
    {
        let bit: u64 = (self.state & ((1 as u64) << 31) != 0) as u64;
        self.Iterate();
        return bit;
    }
    pub fn Print(&self)
    {
        let mut i = 63;
        while i >= 0
        {
            let n = (self.state & ((1 as u64) << i) != 0) as u64;
            if n != 0
            {
                print!("1");
            }
            else
            {
                print!(" ");
            }
            i = i - 1;
        }
        print!("\n");
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