// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.
use core::ops::Add;

#[derive(Copy, Clone, Debug)]
pub struct SaturatingU16 {
    value: u16,
}

// Conversions
impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16 {
            value: value,
        }
    }
}
impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        SaturatingU16 {
            value: *value,
        }        
    }
}
impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        SaturatingU16 {
            value: u16::from(value),
        }       
    }
}
impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        SaturatingU16 {
            value: u16::from(*value),
        }   
    }
}

// Additions
impl Add<SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: SaturatingU16) -> SaturatingU16 {
        SaturatingU16 {
            value: self.value.saturating_add(other.value),
        }
    }
}
impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: &SaturatingU16) -> SaturatingU16 {
        SaturatingU16 {
            value: self.value.saturating_add(other.value),
        }
    }
}
impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: u16) -> SaturatingU16 {
        SaturatingU16 {
            value: self.value.saturating_add(other),
        }
    }
}
impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: &u16) -> SaturatingU16 {
        SaturatingU16 {
            value: self.value.saturating_add(*other),
        }
    }
}

// Comparisons
impl PartialEq<SaturatingU16> for SaturatingU16 {
    fn eq(&self, other: &SaturatingU16) -> bool {
        self.value == other.value
    }
}
impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}