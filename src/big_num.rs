use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::Add;

use super::math;

type Digest = usize;
const SINGLE_MAX_VALUE: Digest = Digest::max_value() / 2 - 2;
const EMPTY_VALUE: Digest = 0 as Digest;
const ONE_VALUE: Digest = 1 as Digest;

#[derive(Clone, Eq, PartialEq)]
pub struct BigNum {
    nums: Vec<Digest>,
}

impl BigNum {
    /// parse string to big number
    pub fn parse(_i: String) -> Self {
        Self::new(Vec::new()) //TODO
    }

    pub fn new(nums: Vec<Digest>) -> Self {
        Self { nums }
    }

    pub fn new_empty() -> Self {
        Self::new(vec![EMPTY_VALUE])
    }

    pub fn new_one() -> Self {
        Self::new(vec![ONE_VALUE])
    }

    pub fn new_max() -> Self {
        Self::new(vec![SINGLE_MAX_VALUE; 1024])
    }

    pub fn new_single_max() -> Self {
        Self::new(vec![SINGLE_MAX_VALUE])
    }

    pub fn new_min() -> Self {
        Self::new(vec![EMPTY_VALUE]) // TODO if litter than 0
    }

    fn empty() -> Self {
        BigNum { nums: vec![] }
    }

    fn len(&self) -> usize {
        self.nums.len()
    }

    fn append(&mut self, mut big_num: BigNum) {
        self.nums.append(&mut big_num.nums);
    }

    fn pop(&mut self) -> Digest {
        if self.len() > 0 {
            self.nums.pop().unwrap()
        } else {
            EMPTY_VALUE
        }
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

    fn _fill_align(&mut self, len: usize) {
        let self_len = self.nums.len();
        if self_len < len {
            let _ = (0..(len - self_len)).map(|_| self.nums.insert(0, EMPTY_VALUE));
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
        write!(f, "BigNum: {:?}", self.nums)
    }
}

fn recursion_add(mut x: BigNum, mut y: BigNum, z: Digest) -> BigNum {
    if x.len() == 0 || y.len() == 0 {
        if x.len() == 0 && y.len() == 0 {
            if z == EMPTY_VALUE {
                return BigNum::empty();
            }
            return BigNum::new(vec![ONE_VALUE]);
        }

        let new_s = if x.len() != 0 { x.pop() } else { y.pop() };

        let mut new_big_num = recursion_add(x, y, EMPTY_VALUE);
        new_big_num.append(BigNum::new(vec![new_s]));
        return new_big_num;
    }

    let xx = x.pop();
    let yy = y.pop();

    let new_add = xx + yy + z;
    let (new_s, new_z) = if new_add > SINGLE_MAX_VALUE {
        (new_add - SINGLE_MAX_VALUE - 1, ONE_VALUE)
    } else {
        (new_add, EMPTY_VALUE)
    };

    let mut new_big_num = recursion_add(x, y, new_z);
    new_big_num.append(BigNum::new(vec![new_s]));
    new_big_num
}

// TODO +
impl Add for BigNum {
    type Output = BigNum;

    fn add(self, other: BigNum) -> BigNum {
        recursion_add(self.clone(), other.clone(), EMPTY_VALUE)
    }
}

// TODO -
// TODO *
// TODO /
// TODO %
// TODO &

impl PartialOrd for BigNum {
    fn partial_cmp(&self, other: &BigNum) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigNum {
    fn cmp(&self, other: &BigNum) -> Ordering {
        self.nums.cmp(&other.nums)
    }
}
