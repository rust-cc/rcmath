use rand_core::SeedableRng;
use rand_xorshift::XorShiftRng;
use serde::{Deserialize, Serialize};

use crate::uint::{arithmetic, Uint};
use crate::utils::BitIterator;

fn uint_arithmetic_test<U: Uint>(a: U, b: U, zero: U) {
    // zero == zero
    assert_eq!(zero, zero);

    // zero.is_zero() == true
    assert_eq!(zero.is_zero(), true);

    // a == a
    assert_eq!(a, a);

    // a + 0 = a
    let mut a0_add = a.clone();
    a0_add.add_nocarry(&zero);
    assert_eq!(a0_add, a);

    // a - 0 = a
    let mut a0_sub = a.clone();
    a0_sub.sub_noborrow(&zero);
    assert_eq!(a0_sub, a);

    // a - a = 0
    let mut aa_sub = a.clone();
    aa_sub.sub_noborrow(&a);
    assert_eq!(aa_sub, zero);

    // a + b = b + a
    let mut ab_add = a.clone();
    ab_add.add_nocarry(&b);
    let mut ba_add = b.clone();
    ba_add.add_nocarry(&a);
    assert_eq!(ab_add, ba_add);
}

fn uint_bits_test<U: Uint>() {
    let mut one = U::from(1u64);
    assert!(one.get_bit(0));
    assert!(!one.get_bit(1));
    one.mul(5);
    let thirty_two = one;
    assert!(!thirty_two.get_bit(0));
    assert!(!thirty_two.get_bit(1));
    assert!(!thirty_two.get_bit(2));
    assert!(!thirty_two.get_bit(3));
    assert!(!thirty_two.get_bit(4));
    assert!(thirty_two.get_bit(5), "{:?}", thirty_two);
}

fn uint_bytes_test<U: Uint>() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);
    let x: U = U::random(&mut rng);
    let bytes = x.to_bytes();
    let y = U::from_bytes(bytes.as_ref()).unwrap();
    assert_eq!(x, y);
}

fn test_uint<U: Uint>(zero: U) {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);
    let a: U = U::random(&mut rng);
    let b: U = U::random(&mut rng);
    uint_arithmetic_test(a, b, zero);
    uint_bytes_test::<U>();
    uint_bits_test::<U>();
}

#[test]
fn test_uint64() {
    crate::uint_impl!(U64, 1);
    test_uint(U64::new([0u64; 1]));
}

#[test]
fn test_uint128() {
    crate::uint_impl!(U128, 2);
    test_uint(U128::new([0u64; 2]));
}

#[test]
fn test_uint256() {
    crate::uint_impl!(U256, 4);
    test_uint(U256::new([0u64; 4]));
}
