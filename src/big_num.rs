use std::fmt::{Debug, Display, Formatter, Result};

type Digest = usize;

pub struct BigNum {
    nums: Vec<Digest>,
}

impl BigNum {
    fn new(nums: Vec<Digest>) -> Self {
        Self { nums }
    }

    pub fn min_value() -> Self {
        Self::new(vec![0_usize])
    }

    pub fn max_value() -> Self {
        Self::new(vec![usize::max_value(); 1024]) // maybe this need change 1024 -> usize::max_value()
    }
}

impl Display for BigNum {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut hex = String::new();
        hex.extend(self.nums.iter().map(|i| format!("{}", i)));
        write!(f, "{}", hex)
    }
}

impl Debug for BigNum {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut hex = String::new();
        hex.extend(self.nums.iter().map(|i| format!("{}", i)));
        write!(f, "BigNum: {}", hex)
    }
}
