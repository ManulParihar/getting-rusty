use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign}
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct U256([u64; 4]);

/**
 * U256([l0, l1, l2, l3])
 * ============================================================
 * (l0 * 2^0) + (l1 * 2^64) + (l2 * 2^128) + (l3 * 2^192)
 *      ^                                       ^
 *      |                                       |
 * lest significant                         most significant
 * ============================================================
 * |  limb3  |  limb2  |  limb1 | limb0 |
 * | 192 bit | 128 bit | 64 bit | 0 bit |
 * ============================================================
 */
impl U256 {
    pub const ZERO:Self = Self([0u64; 4]);

    pub fn zero() -> Self {
        Self::ZERO
    }

    pub fn as_limbs(&self) -> &[u64; 4] {
        &self.0
    }

    // Helper to construct U256 for multi limbs (num >= 2^64)
    pub fn from_limbs(limbs: [u64; 4]) -> Self {
        Self(limbs)
    }

    pub fn is_zero(&self) -> bool {
        self.0 == [0u64; 4]
    }
}

impl From<u64> for U256 {
    fn from(value: u64) -> Self {
        Self([value, 0, 0, 0])
    }
}

impl Display for U256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_zero() {
            return write!(f, "0");
        }
        
        let mut started = false;
        for limb in self.0.iter().rev() {
            if !started {
                if *limb == 0 { continue; }
                else {
                    started = true;
                    write!(f, "{:x}", limb)?;
                }
            }
            else {
                write!(f, "{:016x}", limb)?;
            }
        }
        Ok(())
    }
}

impl Debug for U256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "U256(0x{})", self)
    }
}

impl Add for U256 {
    type Output = U256;

    fn add(self, rhs: Self) -> Self::Output {
        
        let mut num = [0u64; 4];

        let (sum, carry) = self.0[0].overflowing_add(rhs.0[0]);
        num[0] = sum;
        let mut set_carry = carry;

        for i in 1..4 {
            let (sum, carry1) = self.0[i].overflowing_add(rhs.0[i]);
            num[i] = sum;
            
            if set_carry {
                let (sum, carry2) = num[i].overflowing_add(1u64);
                num[i] = sum;
                set_carry = carry1 || carry2;
            }
            else {
                set_carry = carry1;
            }
        }
        
        Self(num)
    }
}

impl AddAssign for U256 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
