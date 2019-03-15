#[cfg(test)]
mod tests {
    use sunzi::big_num::BigNum;
    use sunzi::math;

    #[test]
    fn test_pow_mod() {
        assert_eq!(56, math::pow_mod(2, 2_u32.pow(31), 100))
    }

    #[test]
    fn test_big_num() {
        println!("{}", BigNum::max_value());
        println!("{:?}", BigNum::max_value());
        println!("{}", BigNum::min_value());
        println!("{:?}", BigNum::min_value());
        assert_eq!(56, math::pow_mod(2, 2_u32.pow(31), 100))
    }
}
