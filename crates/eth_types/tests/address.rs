#[cfg(test)]
mod tests {
    use eth_types::{Address, AddressError};
    use std::str::FromStr;

    // ZERO
    #[test]
    fn zero_is_all_zeros() {
        let addr = Address::zero();
        assert_eq!(addr.as_bytes(), &[0u8; 20]);
    }

    #[test]
    fn zero_const_zero_address() {
        assert_eq!(Address::zero(), Address::ZERO);
    }

    // from_slice
    #[test]
    fn from_slice_valid() {
        let addr = [1u8; 20];
        assert_eq!(
            Address::from_slice(&addr).unwrap(),
            Address::from(addr)
        );
    }

    #[test]
    fn from_slice_invalid_length() {
        let addr = [1u8; 19];
        assert!(matches!(
            Address::from_slice(&addr).unwrap_err(),
            AddressError::InvalidLength
        ));
    }

    // FromStr
    #[test]
    fn from_str_without_prefix() {
        let str = "abababababababababababababababababababab";
        let addr = Address::from_str(str).unwrap();
        assert_eq!(
            addr,
            Address::from([0xab; 20])
        );
    }

    #[test]
    fn from_str_with_prefix() {
        let str = "0xabababababababababababababababababababab";
        let addr = Address::from_str(str).unwrap();
        assert_eq!(
            addr,
            Address::from([0xab; 20])
        );
    }

    #[test]
    fn from_str_invalid_length() {
        let str = "0xabcabc";
        assert_eq!(
            Address::from_str(str).unwrap_err(),
            AddressError::InvalidLength
        )
    }

    #[test]
    fn from_str_invalid_hex() {
        let str = "0xabcabcwxyz";
        assert_eq!(
            Address::from_str(str).unwrap_err(),
            AddressError::InvalidHex
        )
    }

    // Display/Debug
    #[test]
    fn display_is_hex () {
        let addr = Address::from([0xab; 20]);
        assert_eq!(
            addr.to_string(),
            "abababababababababababababababababababab"
        );
    }
    
    #[test]
    fn debug_wraps_display () {
        let addr = Address::zero();
        let display = format!("Address({})", addr);
        assert_eq!(
            display,
            format!("{:?}", addr)
        );
    }

    // roundtrip
    #[test]
    fn display_parse_roundtrip() {
        let original = Address::from([9u8; 20]);
        let parsed = Address::from_str(&original.to_string()).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn display_parse_roundtrip_with_prefix() {
        let original = Address::from([7u8; 20]);
        let s = format!("0x{}", original);
        let parsed = Address::from_str(&s).unwrap();
        assert_eq!(parsed, original);
    }
}