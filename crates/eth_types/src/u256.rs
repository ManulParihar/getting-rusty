use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, BitAnd, BitOr, BitXor, Shl, Shr}
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
    pub const ZERO: Self = Self([0u64; 4]);
    pub const MAX: Self = Self([u64::MAX; 4]);

    pub fn zero() -> Self {
        Self::ZERO
    }

    pub fn max() -> Self {
        Self::MAX
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

impl From<[u64; 4]> for U256 {
    fn from(value: [u64; 4]) -> Self {
        Self(value)
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

// Implementing << (bit shift left) for U256
impl Shl<u32> for U256 {
    type Output = U256;
    
    fn shl(self, rhs: u32) -> Self::Output {
        if rhs >= 256 {
            return Self::ZERO;
        }

        let limbs = self.0;
        // limbs to be shifted
        let limb_shift = (rhs / 64) as usize;
        // bits in a limb to be shifted
        let bit_shift = rhs % 64;

        let mut num = [0u64; 4];

        // limb shift
        for i in 0..4 {
            if i + limb_shift < 4 {
                num[i + limb_shift] = limbs[i];
            }
        }

        // bit shift
        if bit_shift > 0 {
            for i in (0..4).rev() {
                // do shl on current limb
                num[i] <<= bit_shift;

                // transfer carry over bits from the previous limb
                if i > 0 {
                    // during `num[i] <<= bit_shift`, the left most bits were updated to 0
                    // `|=` overwrites the left most bits with the carry over from previous bits
                    // since shl will discard carry over bits, it can be calculated
                    // by reverse shifting (i.e. shifting right) by (64-bit_shift) units
                    num[i] |= limbs[i - 1] >> (64 - bit_shift);
                }
            }
        }

        U256(num)
    }
}

// Implementing >> (bit shift right) for U256
impl Shr<u32> for U256 {
    type Output = U256;

    fn shr(self, rhs: u32) -> Self::Output {
        // handle overflow
        if rhs >= 256 {
            return Self::ZERO;
        }

        let mut num = [0u64; 4];
        let limbs = self.0;

        // limbs to be shifted
        let limb_shift = (rhs / 64) as usize;
        // bits to be shifted
        let bit_shift = rhs % 64;

        // limb shift
        for i in 0..4 {
            if i + limb_shift < 4 {
                num[i] = limbs[i + limb_shift];
            }
        }

        // bit shift
        if bit_shift > 0 {
            for i in 0..4 {
                // shift bits in the present limb
                num[i] >>= bit_shift;

                // handle carry over bits from the previous limb
                if i < 3 {
                    num[i] |= limbs[i + 1] << (64 - bit_shift);
                }
            }
        }

        U256(num)
    }
}

impl BitAnd for U256 {
    type Output = U256;

    fn bitand(self, rhs: Self) -> Self::Output {
        if self.is_zero() || rhs.is_zero() {
            return U256::ZERO;
        }

        let mut num = [0u64; 4];
        let lhs = self.0;
        let rhs = rhs.0;

        for i in 0..4 {
            num[i] = lhs[i] & rhs[i];
        }

        Self(num)
    }
}

impl BitOr for U256 {
    type Output = U256;

    fn bitor(self, rhs: Self) -> Self::Output {
        if self.is_zero() && rhs.is_zero() {
            return U256::ZERO;
        }

        if self.is_zero() {
            return rhs;
        }

        if rhs.is_zero() {
            return self;
        }

        let mut num = [0u64; 4];
        let lhs = self.0;
        let rhs = rhs.0;

        for i in 0..4 {
            num[i] = lhs[i] |  rhs[i];
        }

        Self(num)
    }
}

impl BitXor for U256 {
    type Output = U256;

    fn bitxor(self, rhs: Self) -> Self::Output {
        if self.is_zero() && rhs.is_zero() {
            return U256::ZERO;
        }

        if self.is_zero() {
            return rhs;
        }

        if rhs.is_zero() {
            return self;
        }

        let mut num = [0u64; 4];
        let lhs = self.0;
        let rhs = rhs.0;

        for i in 0..4 {
            num[i] = lhs[i] ^ rhs[i];
        }

        Self(num)
    }
}
