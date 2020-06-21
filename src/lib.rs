#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(use_asm, feature(llvm_asm))]
#![cfg_attr(not(use_asm), forbid(unsafe_code))]
#![cfg_attr(use_asm, deny(unsafe_code))]

//#![deny(unused_import_braces, unused_qualifications, trivial_casts)]
//#![deny(trivial_numeric_casts, variant_size_differences)]
//#![deny(non_shorthand_field_patterns, unused_attributes, unused_imports)]
//#![deny(unused_extern_crates, renamed_and_removed_lints, unused_allocation)]
//#![deny(unused_comparisons, bare_trait_objects, const_err, unused_must_use)]
//#![deny(unused_mut, unused_unsafe, private_in_public)]

#[cfg(all(test, not(feature = "std")))]
#[macro_use]
extern crate std;

/// This crate needs to be public, because we expose the `to_bytes!` macro.
/// See the similar issue in [`smallvec#198`]
///
/// [`smallvec#198`]: https://github.com/servo/rust-smallvec/pull/198
#[cfg(not(feature = "std"))]
#[macro_use]
#[doc(hidden)]
pub extern crate alloc;

#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
#[doc(hidden)]
pub use alloc::{
    borrow::Cow, boxed::Box, collections::BTreeMap, format, string::String, vec, vec::Vec,
};

#[cfg(feature = "std")]
#[allow(unused_imports)]
#[doc(hidden)]
pub use std::{
    borrow::Cow, boxed::Box, collections::BTreeMap, format, string::String, vec, vec::Vec,
};

#[macro_use]
extern crate derivative;

pub mod uint;
pub use self::uint::*;

#[macro_use]
pub mod ff;
pub use self::ff::*;

pub mod fft;
pub use self::fft::*;

pub mod pairing;
pub use self::pairing::*;

pub mod groups;
pub use self::groups::*;

mod rand;
pub use self::rand::*;

//mod to_field_vec;
//pub use to_field_vec::ToConstraintField;

pub mod msm;
pub use self::msm::*;

pub use num_traits::{One, Zero};

pub mod prelude {
    pub use crate::uint::Uint;

    pub use crate::ff::{Field, FpParameters, PrimeField, SquareRootField};

    pub use crate::groups::Group;

    pub use crate::pairing::{AffineCurve, PairingEngine, ProjectiveCurve};

    pub use crate::rand::UniformRand;

    pub use num_traits::{One, Zero};

    pub use crate::Error;
}

pub(crate) type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(pub &'static str);

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        writeln!(f, "Error: {}", self.0)
    }
}

/// Returns log2
pub fn log2(x: usize) -> u32 {
    if x <= 1 {
        return 0;
    }

    let n = x.leading_zeros();
    core::mem::size_of::<usize>() as u32 * 8 - n
}
