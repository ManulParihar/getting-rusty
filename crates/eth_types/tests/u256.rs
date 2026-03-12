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
        
        // (*limbs)[0] <=> limbs[0] because of auto deref
        assert_eq!(limbs[0], 42);
        assert_eq!(limbs[1], 0);
        assert_eq!(limbs[2], 0);
        assert_eq!(limbs[3], 0);
    }

    #[test]
    fn display_hex() {
        assert_eq!(
            U256::from(42u64).to_string(),
            "2a"
        )
    }

    #[test]
    fn display_formatting() {
        let num = U256::from(42u64);
        assert_eq!(
            num.to_string(),
            "2a"
        );
    }

    #[test]
    fn debug_formatting() {
        let num = U256::from(42u64);
        let format = format!("U256(0x{num})");
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

    #[test]
    fn display_multi_limb() {
        // 2^64
        let num = U256::from_limbs([0, 1, 0, 0]);
        assert_eq!(
            num.to_string(),
            // 4 bits per hex digit => 16 bits for 2^64 => 16 zeros
            "10000000000000000"
        );
    }

    #[test]
    fn basic_addition() {
        let sum = U256::from(1u64) + U256::from(10u64);
        assert_eq!(
            sum,
            U256::from(11u64)
        );
    }

    #[test]
    fn single_limb_overflow() {
        let sum = U256::from_limbs([u64::MAX, 0, 0, 0]) + U256::from(1u64);
        assert_eq!(
            sum,
            U256::from_limbs([0, 1, 0, 0])
        );
    }

    #[test]
    fn multi_limb_overflow() {
        let sum = U256::from_limbs([u64::MAX, u64::MAX, u64::MAX, 1u64]) + 
                        U256::from_limbs([u64::MAX, u64::MAX, u64::MAX, 10u64]);

        assert_eq!(
            sum,
            U256::from_limbs([(u64::MAX - 1u64), u64::MAX, u64::MAX, 12u64])
        );
    }

    #[test]
    fn overflow_wrapping() {
        let sum = U256::from_limbs([u64::MAX, u64::MAX, u64::MAX, u64::MAX]) + 
                        U256::from_limbs([1, 0, 0, 0]);

        assert_eq!(
            sum,
            U256::ZERO
        );
    }

    #[test]
    fn add_assign() {
        let mut num = U256::from_limbs([u64::MAX, u64::MAX, u64::MAX, 0u64]);
        num += U256::from(1u64);

        assert_eq!(
            num,
            U256::from_limbs([0, 0, 0, 1])
        );
    }

    #[test]
    fn basic_shl() {
        // ...0001 << 1 === ...0010
        let num = U256::from(1u64);
        assert_eq!(
            num << 1,
            U256::from(2u64)
        );
    }

    #[test]
    fn boundary_shift_shl() {
        // (binary) 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0001 << 63
        // (binary) 1000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000
        // (hex)    8    0    0    0    0    0    0    0    0    0    0    0    0    0    0    0
        let num = U256::from(1u64);
        assert_eq!(
            num << 63,
            U256::from_limbs([0x8000000000000000, 0, 0, 0])
        );
    }

    #[test]
    fn limb_shift_shl() {
        // U256([1, 0, 0, 0]) << 64 === U256([0, 1, 0, 0])
        let num = U256::from(1u64);
        assert_eq!(
            num << 64,
            U256::from_limbs([0, 1, 0, 0])
        );
    }

    #[test]
    fn multi_limb_shift_shl() {
        // U256([1, 0, 0, 0]) << 128 === U256([0, 0, 1, 0])
        let num = U256::from(1u64);
        assert_eq!(
            num << 128,
            U256::from_limbs([0, 0, 1, 0])
        );
    }

    #[test]
    fn overflow_limb_shift_shl() {
        // U256([1, 0, 0, 0]) << 256 === U256([0, 0, 0, 0])
        let num = U256::from(1u64);
        assert_eq!(
            num << 256,
            U256::ZERO
        );
    }
}