use std::{fmt::Debug, str::FromStr};

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum AddressError {
    #[error("Invalid length: expected 20 bytes")]
    InvalidLength,
    #[error("Invalid hex")]
    InvalidHex,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Address(pub [u8; 20]);

impl Address {
    pub const ZERO: Self = Self([0u8; 20]);

    pub fn zero() -> Self {
        Self::ZERO
    }

    pub fn as_bytes(&self) -> &[u8; 20] {
        &self.0
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self, AddressError> {
        if slice.len() != 20 {
            return Err(AddressError::InvalidLength);
        }
        else {
            let mut bytes = [0u8; 20];
            bytes.copy_from_slice(slice);
            Ok(Address(bytes))
        }
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl Debug for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Address({self})")
    }
}

impl AsRef<[u8]> for Address {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl From<[u8; 20]> for Address {
    fn from(value: [u8; 20]) -> Self {
        Address(value)
    }
}

impl FromStr for Address {
    type Err = AddressError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("0x").unwrap_or(s);
        let s_vec = hex::decode(s).map_err(|_| AddressError::InvalidHex)?;
        Address::from_slice(&s_vec)
    }
}
