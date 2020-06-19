/// Flags to be encoded into the serialization.
#[derive(Default, Clone, Copy)]
pub struct EmptyFlags;

/// Flags to be encoded into the serialization.
/// The default flags (empty) should not change the binary representation.
#[derive(Clone, Copy)]
pub enum SWFlags {
    Infinity,
    PositiveY,
    NegativeY,
}

impl SWFlags {
    #[inline]
    pub fn u8_bitmask(&self) -> u8 {
        let mut mask = 0;
        match self {
            SWFlags::Infinity => mask |= 1 << 6,
            SWFlags::PositiveY => mask |= 1 << 7,
            _ => (),
        }
        mask
    }
}

impl Default for SWFlags {
    #[inline]
    fn default() -> Self {
        // NegativeY doesn't change the serialization
        SWFlags::NegativeY
    }
}

/// Flags to be encoded into the serialization.
/// The default flags (empty) should not change the binary representation.
#[derive(Clone, Copy)]
pub enum EdwardsFlags {
    PositiveY,
    NegativeY,
}

impl EdwardsFlags {
    #[inline]
    pub fn from_u8(value: u8) -> Self {
        let x_sign = (value >> 7) & 1 == 1;
        if x_sign {
            EdwardsFlags::PositiveY
        } else {
            EdwardsFlags::NegativeY
        }
    }

    #[inline]
    pub fn is_positive(&self) -> bool {
        match self {
            EdwardsFlags::PositiveY => true,
            EdwardsFlags::NegativeY => false,
        }
    }
}

impl Default for EdwardsFlags {
    #[inline]
    fn default() -> Self {
        // NegativeY doesn't change the serialization
        EdwardsFlags::NegativeY
    }
}
