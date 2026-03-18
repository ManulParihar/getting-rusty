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
        let sum = U256::MAX + U256::from_limbs([1, 0, 0, 0]);

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
    fn all_bit_shl() {
        let num1 = U256::from([
            0x8000000000000001,
            0x8000000000000001,
            0x8000000000000001,
            0x8000000000000001,
        ]);

        let num2 = U256::from([
            0x0000000000000002,
            0x0000000000000003,
            0x0000000000000003,
            0x0000000000000003,
        ]);

        assert_eq!(num1 << 1, num2);
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

    #[test]
    fn basic_shr() {
        // ...0001 >> 1 === ...0000
        let num = U256::from(1u64);
        assert_eq!(
            num >> 1,
            U256::ZERO
        );
    }

    #[test]
    fn all_bit_shr() {
        let num1 = U256::from([
            0x8000000000000001,
            0x8000000000000001,
            0x8000000000000001,
            0x8000000000000001,
        ]);

        let num2 = U256::from([
            0xC000000000000000,
            0xC000000000000000,
            0xC000000000000000,
            0x4000000000000000,
        ]);

        assert_eq!(num1 >> 1, num2);
    }

    #[test]
    fn boundary_shift_shr() {
        // 1000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 >> 63
        // 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0001
        let num = U256::from_limbs([
            0x8000000000000000,
            0, 0, 0
        ]);
        assert_eq!(
            num >> 63,
            U256::from(1u64)
        );
    }

    #[test]
    fn limb_shift_shr() {
        // U256([0, 1, 0, 0]) >> 64 === U256([1, 0, 0, 0])
        let num = U256::from_limbs([0, 1, 0, 0]);
        assert_eq!(
            num >> 64,
            U256::from(1u64)
        );
    }

    #[test]
    fn multi_limb_shift_shr() {
        // U256([0, 0, 1, 0]) >> 128 === U256([1, 0, 0, 0])
        let num = U256::from_limbs([0, 0, 1, 0]);
        assert_eq!(
            num >> 128,
            U256::from_limbs([1, 0, 0, 0])
        );
    }

    #[test]
    fn overflow_limb_shift_shr() {
        // U256([0, 0, 0, 1]) >> 256 === U256([0, 0, 0, 0])
        let num = U256::from_limbs([0, 0, 0, 1]);
        assert_eq!(
            num >> 256,
            U256::ZERO
        );
    }

    #[test]
    fn basic_and() {
        let lhs = U256::from(1);
        let rhs = U256::from(1);
        assert_eq!(
            lhs & rhs,
            U256::from(1)
        );
    }

    #[test]
    fn zero_and() {
        assert_eq!(
            U256::ZERO & U256::from_limbs([123, 456, 789, 999]),
            U256::ZERO
        );
    }

    #[test]
    fn all_ones_and() {
        let num = U256::from_limbs([1, 2, 3, 4]);
        assert_eq!(
            num & U256::MAX,
            num
        );
    }

    #[test]
    fn complement_and() {
        let n1 = U256::from_limbs([u64::MAX, 0, 0, 0]);
        let n2 = U256::from_limbs([0, u64::MAX, 0, 0]);
        assert_eq!(
            n1 & n2,
            U256::ZERO
        );
    }

    #[test]
    fn partial_overlap_and() {
        let a = U256::from_limbs([
            0xF0F0F0F0F0F0F0F0,
            0x0F0F0F0F0F0F0F0F,
            0xFFFF0000FFFF0000,
            0x0000FFFF0000FFFF,
        ]);

        let b = U256::from_limbs([
            0xFF00FF00FF00FF00,
            0x00FF00FF00FF00FF,
            0xFFFF00000000FFFF,
            0x00000000FFFFFFFF,
        ]);

        let expected = U256::from_limbs([
            0xF000F000F000F000,
            0x000F000F000F000F,
            0xFFFF000000000000,
            0x000000000000FFFF,
        ]);

        assert_eq!(
            a & b,
            expected
        );
    }

    #[test]
    fn basic_or() {
        let n1 = U256::from(1);
        let n2 = U256::from(2);
        assert_eq!(
            n1 | n2,
            U256::from(3)
        );
    }

    #[test]
    fn zero_or() {
        let num = U256::from(1);
        assert_eq!(
            num | U256::ZERO,
            num
        );
        assert_eq!(
            U256::ZERO | num,
            num
        );
    }

    #[test]
    fn both_zero_or() {
        assert_eq!(
            U256::ZERO | U256::ZERO,
            U256::ZERO
        );
    }

    #[test]
    fn all_ones_or() {
        assert_eq!(
            U256::MAX | U256::ZERO,
            U256::MAX
        );
    }

    #[test]
    fn identity_or() {
        let n1 = U256::from_limbs([123, 456, 789, 999]);
        assert_eq!(
            n1 | U256::ZERO,
            n1
        );
    }

    #[test]
    fn disjoin_bits_or() {
        let n1 = U256::from_limbs([0xAAAAAAAAAAAAAAAA; 4]);
        let n2 = U256::from_limbs([0x5555555555555555; 4]);
        assert_eq!(
            n1 | n2,
            U256::MAX
        );
    }

    #[test]
    fn random_or() {
        let a = U256::from_limbs([
            0x8000000000000001,
            0x8000000000000001,
            0x8000000000000001,
            0x8000000000000001,
        ]);

        let b = U256::from_limbs([
            0x0,
            0xFFFFFFFFFFFFFFFF,
            0x0,
            0xFFFFFFFFFFFFFFFF,
        ]);

        let expected = U256::from_limbs([
            0x8000000000000001,
            0xFFFFFFFFFFFFFFFF,
            0x8000000000000001,
            0xFFFFFFFFFFFFFFFF,
        ]);
        assert_eq!(a | b, expected);
    }

    #[test]
    fn basic_xor() {
        let a = U256::from(1);
        let b = U256::from(1);
        assert_eq!(
            a ^ b,
            U256::ZERO
        );
    }

    #[test]
    fn self_xor() {
        let a = U256::from_limbs([123, 456, 789, 999]);
        assert_eq!(
            a ^ a,
            U256::ZERO
        );
    }

    #[test]
    fn zero_identity_xor() {
        let a = U256::from_limbs([123, 456, 789, 999]);
        assert_eq!(
            a ^ U256::ZERO,
            a
        );
    }

    #[test]
    fn with_zero_xor() {
        let all_ones = U256::MAX;
        assert_eq!(
            all_ones ^ U256::ZERO,
            all_ones
        );
    }

    #[test]
    fn complement_xor() {
        let n1 = U256::from_limbs([0xAAAAAAAAAAAAAAAA; 4]);
        let n2 = U256::from_limbs([0x5555555555555555; 4]);
        assert_eq!(
            n1 ^ n2,
            U256::MAX
        );
    }

    #[test]
    fn random_xor() {
        let a = U256::from_limbs([
            0x8000000000000001,
            0x8000000000000001,
            0x8000000000000001,
            0x8000000000000001,
        ]);

        let b = U256::from_limbs([
            0xFFFFFFFFFFFFFFFF,
            0,
            0xFFFFFFFFFFFFFFFF,
            0,
        ]);

        let expected = U256::from_limbs([
            0x7FFFFFFFFFFFFFFE,
            0x8000000000000001,
            0x7FFFFFFFFFFFFFFE,
            0x8000000000000001,
        ]);
        assert_eq!(a ^ b, expected);
    }
}