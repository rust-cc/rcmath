use core::{
    cmp::{Ord, Ordering, PartialOrd},
    fmt::{Display, Formatter, Result as FmtResult},
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
};
use num_traits::{One, Zero};
use unroll::unroll_for_loops;

use crate::{
    bytes::{FromBytes, ToBytes},
    io::{Read, Result as IoResult, Write},
    uint::{arithmetic as fa, Uint as _U, U256, U320, U384, U768, U832},
};

use super::{FftField, Field, FpParameters, LegendreSymbol, PrimeField, SquareRootField};

#[cfg(use_asm)]
use std::mem::MaybeUninit;

#[cfg(use_asm)]
include!(concat!(env!("OUT_DIR"), "/field_assembly.rs"));

impl_Fp!(Fp256, Fp256Parameters, U256, U256, 4);
impl_Fp!(Fp320, Fp320Parameters, U320, U320, 5);
impl_Fp!(Fp384, Fp384Parameters, U384, U384, 6);
impl_Fp!(Fp768, Fp768Parameters, U768, U768, 12);
impl_Fp!(Fp832, Fp832Parameters, U832, U832, 13);

pub mod fp2;
pub use self::fp2::*;

pub mod fp3;
pub use self::fp3::*;

pub mod fp4;
pub use self::fp4::*;

pub mod fp6_2over3;

pub mod fp6_3over2;
pub use self::fp6_3over2::*;

pub mod fp12_2over3over2;
pub use self::fp12_2over3over2::*;
