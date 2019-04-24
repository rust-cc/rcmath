#[cfg(test)]
mod tests {
    use sunzi::big_num::BigNum;
    use sunzi::math;

    #[test]
    fn test_pow_mod() {
        assert_eq!(56, math::pow_mod(2, 2_u32.pow(31), 100))
    }

    #[test]
    fn test_big_add() {
        assert_eq!(
            BigNum::new(vec![1, 2, 3]) + BigNum::new(vec![2, 3, 4]),
            BigNum::new(vec![3, 5, 7])
        );
        assert_eq!(
            BigNum::new(vec![1, 2, 3]) + BigNum::new(vec![1, 2, 3, 4]),
            BigNum::new(vec![1, 3, 5, 7])
        );
        assert_eq!(
            BigNum::new(vec![1, 2, 3, 4, 5]) + BigNum::new(vec![2, 3, 4]),
            BigNum::new(vec![1, 2, 5, 7, 9])
        );
        assert_eq!(
            BigNum::new_single_max() + BigNum::new_one(),
            BigNum::new(vec![1, 0])
        );
        assert_eq!(
            BigNum::new_single_max() + BigNum::new_one() + BigNum::new_one(),
            BigNum::new(vec![1, 1])
        )
    }

    #[test]
    fn test_big_num_operate() {
        assert_eq!(56, math::pow_mod(2, 2_u32.pow(31), 100))
    }
}
