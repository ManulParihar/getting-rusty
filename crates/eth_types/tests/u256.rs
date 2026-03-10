#[cfg(test)]
mod tests {
    use eth_types::U256;

    #[test]
    fn zero_constant() {
        assert_eq!(U256::ZERO, U256::from(0u64));
    }

    #[test]
    fn zero_const_matches_zero_fn() {
        assert_eq!(
            U256::ZERO,
            U256::zero()
        );
    }

    #[test]
    fn from_u64() {
        let num = U256::from(42u64);
        let limbs = num.as_limbs();
        
        assert_eq!((*limbs)[0], 42);
        assert_eq!((*limbs)[1], 0);
        assert_eq!((*limbs)[2], 0);
        assert_eq!((*limbs)[3], 0);
    }

    #[test]
    fn is_equal() {
        assert_eq!(
            U256::from(42u64).to_string(),
            "2a"
        )
    }

    #[test]
    fn display_formatting() {
        let num = U256::from(42u64);
        let format = format!("{num}");
        assert_eq!(
            format,
            num.to_string()
        );
    }

    #[test]
    fn debug_formatting() {
        let num = U256::from(42u64);
        let format = format!("U256({num})");
        assert_eq!(
            format!("{:?}", num),
            format
        );
    }

    #[test]
    fn is_zero() {
        let zero = U256::zero();
        let non_zero = U256::from(42u64);

        assert_eq!(zero.is_zero(), true);
        assert_eq!(non_zero.is_zero(), false);
    }
}