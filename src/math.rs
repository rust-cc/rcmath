use std::convert::From;
use std::ops::{Add, Div, Mul, Rem};

pub fn pow<T: Clone + Mul<Output = T> + From<u32>>(x: T, y: usize) -> T {
    (0..y).fold(1u32.into(), |acc, _| x.clone() * acc)
}

pub fn pow_mod(x: u32, mut y: u32, z: u32) -> u32 {
    let mut t = 1;
    let mut tmp = x % z;
    while y > 0 {
        if y & 1 > 0 {
            t = t * tmp % z;
        }

        tmp = tmp * tmp % z;
        y = y >> 1;
    }
    return t;
}

pub fn div_rem<T: Clone + Add + Div + Mul + Rem>(
    x: T,
    y: T,
) -> (<T as Div>::Output, <T as Rem>::Output) {
    (x.clone() / y.clone(), x % y)
}

pub fn div(_x: u32, _y: u32) {}

pub fn rem(_x: u32, _y: u32) {}

pub fn lcm(_x: u32, _y: u32) {}

pub fn gcd(_x: u32, _y: u32) {}

pub fn big_prime() {}

pub fn is_prime(_x: u32) {}

pub fn is_odd(_x: u32) {}

pub fn is_even(_x: u32) {}
