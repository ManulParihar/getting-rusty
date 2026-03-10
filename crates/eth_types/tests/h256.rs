#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use eth_types::{H256, H256Error};

    // zero
    #[test]
    fn zero_is_all_zeros() {
        let zero = H256::zero();
        assert_eq!(zero, H256::from([0u8; 32]));
    }

    #[test]
    fn zero_const_matches_zero_fn() {
        assert_eq!(H256::ZERO, H256::zero());
    }

    // as_bytes
    #[test]
    fn check_as_bytes() {
        let hash = H256::from([1u8; 32]);
        assert_eq!(hash.as_bytes(), &[1u8; 32]);
    }

    // from_slice
    #[test]
    fn from_valid_slice() {
        let hash = [1u8; 32];
        assert_eq!(
            H256::from_slice(&hash).unwrap(),
            H256::from(hash)
        );
    }

    #[test]
    fn from_invalid_slice() {
        let hash = [1u8; 33];
        let h256_from_slice_err = H256::from_slice(&hash).unwrap_err();
        assert!(matches!(
            h256_from_slice_err,
            H256Error::InvalidLength
        ));
    }

    // FromStr
    #[test]
    fn from_str_with_prefix() {
        let str = "0xabababababababababababababababababababababababababababababababab";
        assert_eq!(
            H256::from_str(str).unwrap(),
            H256::from([0xab; 32])
        );
    }

    #[test]
    fn from_str_without_prefix() {
        let str = "abababababababababababababababababababababababababababababababab";
        assert_eq!(
            H256::from_str(str).unwrap(),
            H256::from([0xab; 32])
        );
    }

    #[test]
    fn from_str_invalid_hex() {
        let str = "0xqwerty";
        let e = H256::from_str(str).unwrap_err();
        assert_eq!(e, H256Error::InvalidHex);
    }

    #[test]
    fn from_str_invalid_length() {
        let str = "0xababababababababababababababababababababababababababababababababab";
        let e = H256::from_str(str).unwrap_err();
        assert_eq!(e, H256Error::InvalidLength);
    }

    // Display/Debug
    #[test]
    fn h256_display() {
        let hash = H256::from([0xab; 32]);
        assert_eq!(
            hash.to_string(),
            "abababababababababababababababababababababababababababababababab"
        );
    }

    #[test]
    fn h256_debug_display() {
        let hash = H256::from([0xab; 32]);
        let format = format!("H256({hash})");
        assert_eq!(
            format,
            format!("{:?}", hash)
        );
    }

}