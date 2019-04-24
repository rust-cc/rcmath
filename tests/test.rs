#[cfg(test)]
mod tests {
    use sunzi::big_num::BigNum;
    use sunzi::math;

    #[test]
    fn test_pow() {
        assert_eq!(9u8, math::pow(3u8, 2));
        assert_eq!(9u16, math::pow(3u16, 2));
        assert_eq!(9u32, math::pow(3u32, 2));
        assert_eq!(9u64, math::pow(3u64, 2));
        assert_eq!(9i8, math::pow(3i8, 2));
        assert_eq!(9i16, math::pow(3i16, 2));
        assert_eq!(9i32, math::pow(3i32, 2));
        assert_eq!(9i64, math::pow(3i64, 2));
        assert_eq!(9u32, math::pow(3u32, BigNum::u_new(vec![2])));
        assert_eq!(27i32, math::pow(3i32, BigNum::u_new(vec![3])));
        assert_eq!(
            BigNum::u_new(vec![9]),
            math::pow(BigNum::u_new(vec![3]), 2_usize)
        );
    }

    #[test]
    fn test_pow_mod() {
        assert_eq!(56, math::pow_mod(2, 2_u32.pow(31), 100));
    }

    #[test]
    fn test_big_add() {
        assert_eq!(
            BigNum::u_new(vec![1, 2, 3]) + BigNum::u_new(vec![2, 3, 4]),
            BigNum::u_new(vec![3, 5, 7])
        );
        assert_eq!(
            BigNum::u_new(vec![1, 2, 3]) + BigNum::u_new(vec![1, 2, 3, 4]),
            BigNum::u_new(vec![1, 3, 5, 7])
        );
        assert_eq!(
            BigNum::u_new(vec![1, 2, 3, 4, 5]) + BigNum::u_new(vec![2, 3, 4]),
            BigNum::u_new(vec![1, 2, 5, 7, 9])
        );
        assert_eq!(
            BigNum::new_single_max() + BigNum::new_one(),
            BigNum::u_new(vec![1, 0])
        );
        assert_eq!(
            BigNum::new_single_max() + BigNum::new_one() + BigNum::new_one(),
            BigNum::u_new(vec![1, 1])
        )
    }

    #[test]
    fn test_big_num_operate() {
        assert_eq!(56, math::pow_mod(2, 2_u32.pow(31), 100))
    }
}
