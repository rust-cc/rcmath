use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::Add;

use super::math;

type Digest = usize;
const SINGLE_MAX_VALUE: usize = usize::max_value();

pub struct BigNum {
    nums: Vec<Digest>,
}

impl BigNum {
    /// parse string to big number
    pub fn parse(_i: String) -> Self {
        Self::new(Vec::new()) //TODO
    }

    pub fn min_value() -> Self {
        Self::new(vec![0_usize])
    }

    pub fn max_value() -> Self {
        Self::new(vec![SINGLE_MAX_VALUE; 1024]) // maybe this need change 1024 -> usize::max_value()
    }

    fn new(nums: Vec<Digest>) -> Self {
        Self { nums }
    }

    /// change denary number to number list
    fn from_denary(u: Digest) -> Vec<Digest> {
        let (d, r) = math::div_rem(u, SINGLE_MAX_VALUE);
        if d == 0 {
            return vec![r];
        } else {
            let mut new_vec = BigNum::from_denary(d);
            new_vec.push(r);
            return new_vec;
        }
    }
}

impl From<u32> for BigNum {
    fn from(u: u32) -> Self {
        BigNum::new(BigNum::from_denary(u as Digest))
    }
}

impl From<u64> for BigNum {
    fn from(u: u64) -> Self {
        BigNum::new(BigNum::from_denary(u as Digest))
    }
}

// TODO +
impl Add for BigNum {
    type Output = BigNum;

    fn add(self, _other: BigNum) -> BigNum {
        BigNum::default()
    }
}

// TODO -
// TODO *
// TODO /
// TODO %
// TODO &

impl Default for BigNum {
    fn default() -> Self {
        Self::new(Vec::new())
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
