use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter, Result};
use std::iter::Step;
use std::ops::{Add, Div, Mul, Range, Rem, Sub};

use super::math;

type Digest = usize;
const STEP: Digest = Digest::max_value() / 2;
const SINGLE_MAX_VALUE: Digest = STEP - 1;
const EMPTY_VALUE: Digest = 0 as Digest;
const ONE_VALUE: Digest = 1 as Digest;

#[derive(Clone, Eq, PartialEq)]
pub struct BigNum {
    sign: bool,
    nums: Vec<Digest>,
}

impl BigNum {
    /// parse string to big number
    pub fn parse(_i: String) -> Self {
        Self::u_new(Vec::new()) //TODO
    }

    pub fn u_new(nums: Vec<Digest>) -> Self {
        Self {
            sign: true,
            nums: nums,
        }
    }

    pub fn i_new(nums: Vec<Digest>, sign: bool) -> Self {
        Self { sign, nums }
    }

    pub fn new_empty() -> Self {
        Self::u_new(vec![EMPTY_VALUE])
    }

    pub fn new_one() -> Self {
        Self::u_new(vec![ONE_VALUE])
    }

    pub fn new_max() -> Self {
        Self::u_new(vec![SINGLE_MAX_VALUE; 1024])
    }

    pub fn new_single_max() -> Self {
        Self::u_new(vec![SINGLE_MAX_VALUE])
    }

    pub fn new_min() -> Self {
        Self::u_new(vec![EMPTY_VALUE]) // TODO if litter than 0
    }

    pub fn range(&self) -> Range<BigNum> {
        (BigNum::new_empty()..self.clone())
    }

    fn empty() -> Self {
        BigNum {
            sign: true,
            nums: vec![],
        }
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

    fn taggle_sign(&mut self) {
        self.sign = !self.sign;
    }

    fn to_usize(&self) -> Option<usize> {
        if self.sign == false {
            None
        } else {
            if self.nums.len() > 2 {
                Some(usize::max_value())
            } else if self.nums.len() == 0 {
                Some(0usize)
            } else if self.nums.len() == 1 {
                Some(*self.nums.get(0).clone().unwrap())
            } else {
                let one = self.nums.get(0).clone().unwrap();
                let two = self.nums.get(1).clone().unwrap();

                if two * STEP + one > usize::max_value() {
                    Some(usize::max_value())
                } else {
                    Some(two * STEP + one)
                }
            }
        }
    }

    fn add_usize(&self, n: usize) -> BigNum {
        self + &BigNum::u_new(vec![n as Digest])
    }

    fn div_rem(&self, rhs: &BigNum) -> (BigNum, BigNum) {
        if rhs == &BigNum::new_empty() {
            panic!("Cannot divide by zero-valued !");
        }
        if self <= rhs {
            return (BigNum::new_empty(), self.clone());
        }

        let mut total = BigNum::new_empty();
        let mut step = BigNum::new_one();

        while &total <= self {
            step = &step + &BigNum::new_one();
            total = &(&step + &BigNum::new_one()) * rhs;
        }
        let rem = self - &(&step * rhs);

        (step, rem)
    }
}

impl From<u32> for BigNum {
    fn from(u: u32) -> Self {
        BigNum::u_new(BigNum::from_denary(u as Digest))
    }
}

impl From<u64> for BigNum {
    fn from(u: u64) -> Self {
        BigNum::u_new(BigNum::from_denary(u as Digest))
    }
}

impl From<u8> for BigNum {
    fn from(u: u8) -> Self {
        BigNum::u_new(BigNum::from_denary(u as Digest))
    }
}

impl From<bool> for BigNum {
    fn from(u: bool) -> Self {
        BigNum::u_new(BigNum::from_denary(u as Digest))
    }
}

impl Default for BigNum {
    fn default() -> Self {
        Self::u_new(Vec::new())
    }
}

impl Display for BigNum {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let sign = if self.sign { "+" } else { "-" };
        let mut hex = String::new();
        hex.extend(self.nums.iter().map(|i| format!("{}", i)));
        write!(f, "{}{}", sign, hex)
    }
}

impl Debug for BigNum {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let sign = if self.sign { "+" } else { "-" };
        write!(f, "BigNum: {} {:?}", sign, self.nums)
    }
}

fn recursion_add(mut x: BigNum, mut y: BigNum, z: Digest) -> BigNum {
    if x.len() == 0 || y.len() == 0 {
        if x.len() == 0 && y.len() == 0 {
            if z == EMPTY_VALUE {
                return BigNum::empty();
            }
            return BigNum::u_new(vec![ONE_VALUE]);
        }

        let new_s = if x.len() != 0 { x.pop() } else { y.pop() };

        let mut new_big_num = recursion_add(x, y, EMPTY_VALUE);
        new_big_num.append(BigNum::u_new(vec![new_s]));
        return new_big_num;
    }

    let xx = x.pop();
    let yy = y.pop();

    let new_add = xx + yy + z;
    let (new_s, new_z) = if new_add > SINGLE_MAX_VALUE {
        (new_add - STEP, ONE_VALUE)
    } else {
        (new_add, EMPTY_VALUE)
    };

    let mut new_big_num = recursion_add(x, y, new_z);
    new_big_num.append(BigNum::u_new(vec![new_s]));
    new_big_num
}

fn recursion_sub(mut x: BigNum, mut y: BigNum, z: Digest) -> BigNum {
    if x.len() == 0 || y.len() == 0 {
        if x.len() == 0 && y.len() == 0 {
            if z == EMPTY_VALUE {
                return BigNum::empty();
            }
            return BigNum::u_new(vec![SINGLE_MAX_VALUE]);
        }

        let (new_s, new_z) = if x.len() != 0 {
            (x.pop(), EMPTY_VALUE)
        } else {
            (y.pop(), ONE_VALUE)
        };

        let mut new_big_num = recursion_add(x, y, new_z);
        new_big_num.append(BigNum::u_new(vec![new_s]));
        return new_big_num;
    }

    let xx = x.pop();
    let yy = y.pop();

    let (new_s, new_z) = if yy > xx || yy + z > xx {
        (xx + STEP - z - yy, ONE_VALUE)
    } else {
        (xx - yy, EMPTY_VALUE)
    };

    let mut new_big_num = recursion_sub(x, y, new_z);
    new_big_num.append(BigNum::u_new(vec![new_s]));
    new_big_num
}

fn first_cmp(x: &BigNum, y: &BigNum, index: usize) -> Ordering {
    if x.nums.get(index).is_none() {
        return Ordering::Equal;
    }

    let order = x.nums.get(index).as_ref().cmp(&y.nums.get(index).as_ref());
    if order == Ordering::Equal {
        first_cmp(x, y, index + 1)
    } else {
        order
    }
}

impl Add for &BigNum {
    type Output = BigNum;

    fn add(self, other: &BigNum) -> BigNum {
        recursion_add(self.clone(), other.clone(), EMPTY_VALUE)
    }
}

impl Add for BigNum {
    type Output = BigNum;

    fn add(self, other: BigNum) -> BigNum {
        recursion_add(self.clone(), other.clone(), EMPTY_VALUE)
    }
}

impl Sub for BigNum {
    type Output = BigNum;

    fn sub(self, other: BigNum) -> BigNum {
        let mut num = recursion_sub(self.clone(), other.clone(), EMPTY_VALUE);
        if self < other {
            num.taggle_sign();
        }
        num
    }
}

impl Sub for &BigNum {
    type Output = BigNum;

    fn sub(self, other: &BigNum) -> BigNum {
        let mut num = recursion_sub(self.clone(), other.clone(), EMPTY_VALUE);
        if self < other {
            num.taggle_sign();
        }
        num
    }
}

impl Mul for BigNum {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        rhs.range()
            .fold(Self::new_empty(), |acc, _| self.clone() + acc)
    }
}

impl Mul for &BigNum {
    type Output = BigNum;

    fn mul(self, rhs: &BigNum) -> BigNum {
        rhs.range()
            .fold(BigNum::new_empty(), |acc, _| self.clone() + acc)
    }
}

// Div
impl Div for BigNum {
    // The division of rational numbers is a closed operation.
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        self.div_rem(&rhs).0
    }
}

impl Div for &BigNum {
    // The division of rational numbers is a closed operation.
    type Output = BigNum;

    fn div(self, rhs: Self) -> BigNum {
        self.div_rem(rhs).0
    }
}

// Rem (mod)
impl Rem for BigNum {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        self.div_rem(&rhs).1
    }
}

// Rem (mod)
impl Rem for &BigNum {
    type Output = BigNum;

    fn rem(self, rhs: Self) -> BigNum {
        self.div_rem(&rhs).1
    }
}

// TODO &

impl PartialOrd for BigNum {
    fn partial_cmp(&self, other: &BigNum) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigNum {
    fn cmp(&self, other: &BigNum) -> Ordering {
        if self.len() == 0 && other.len() == 0 {
            return Ordering::Equal;
        }

        if self.sign != other.sign {
            self.sign.cmp(&other.sign)
        } else {
            if self.sign == true {
                if self.len() != other.len() {
                    self.len().cmp(&other.len())
                } else {
                    first_cmp(self, other, 0)
                }
            } else {
                if self.len() != other.len() {
                    other.len().cmp(&self.len())
                } else {
                    first_cmp(other, self, 0)
                }
            }
        }
    }
}

impl Step for BigNum {
    fn steps_between(start: &BigNum, end: &BigNum) -> Option<usize> {
        let new_num = end - start;
        new_num.to_usize()
    }
    fn add_usize(&self, n: usize) -> Option<BigNum> {
        Some(self.add_usize(n))
    }

    fn replace_one(&mut self) -> BigNum {
        self.clone()
    }

    fn replace_zero(&mut self) -> BigNum {
        self.clone()
    }

    fn add_one(&self) -> BigNum {
        self.clone() + BigNum::new_one()
    }

    fn sub_one(&self) -> BigNum {
        self.clone() - BigNum::new_one()
    }
}
