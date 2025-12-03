//! Safe Rust wrappers for GnuCash core types.

use std::ffi::CStr;
use std::fmt;

use crate::ffi;

/// GUID encoding length (32 hex characters).
pub const GUID_ENCODING_LENGTH: usize = 32;

/// A 128-bit globally unique identifier.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Guid(pub(crate) ffi::GncGUID);

impl Guid {
    /// Creates a new random GUID.
    pub fn new() -> Self {
        Self(unsafe { ffi::guid_new_return() })
    }

    /// Creates a GUID from raw bytes.
    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(ffi::GncGUID { reserved: bytes })
    }

    /// Returns the raw bytes of this GUID.
    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.0.reserved
    }

    /// Returns the null GUID (all zeros).
    pub fn null() -> &'static Self {
        unsafe {
            let ptr = ffi::guid_null();
            &*(ptr as *const ffi::GncGUID as *const Self)
        }
    }

    /// Checks if this is the null GUID.
    pub fn is_null(&self) -> bool {
        self.0.reserved == [0u8; 16]
    }

    /// Parses a GUID from a 32-character hex string.
    pub fn parse(s: &str) -> Option<Self> {
        if s.len() != GUID_ENCODING_LENGTH {
            return None;
        }
        let c_str = std::ffi::CString::new(s).ok()?;
        let mut guid = ffi::GncGUID { reserved: [0; 16] };
        let result = unsafe { ffi::string_to_guid(c_str.as_ptr(), &mut guid) };
        if result != 0 {
            Some(Self(guid))
        } else {
            None
        }
    }
}

impl Default for Guid {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Guid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Guid({})", self)
    }
}

impl fmt::Display for Guid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0.reserved {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl From<ffi::GncGUID> for Guid {
    fn from(guid: ffi::GncGUID) -> Self {
        Self(guid)
    }
}

impl From<Guid> for ffi::GncGUID {
    fn from(guid: Guid) -> Self {
        guid.0
    }
}

/// A rational number with 64-bit numerator and denominator.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Numeric(pub(crate) ffi::gnc_numeric);

impl Numeric {
    /// Creates a new Numeric from numerator and denominator.
    pub fn new(num: i64, denom: i64) -> Self {
        Self(ffi::gnc_numeric { num, denom })
    }

    /// Returns zero (0/1).
    pub fn zero() -> Self {
        Self::new(0, 1)
    }

    /// Returns the numerator.
    pub fn num(&self) -> i64 {
        self.0.num
    }

    /// Returns the denominator.
    pub fn denom(&self) -> i64 {
        self.0.denom
    }

    /// Checks if this value represents zero.
    pub fn is_zero(&self) -> bool {
        self.0.num == 0 && self.0.denom != 0
    }

    /// Checks if this value is negative.
    pub fn is_negative(&self) -> bool {
        (self.0.num < 0) != (self.0.denom < 0)
    }

    /// Checks if this value is positive.
    pub fn is_positive(&self) -> bool {
        !self.is_zero() && !self.is_negative()
    }

    /// Converts to f64.
    pub fn to_f64(&self) -> f64 {
        if self.0.denom == 0 {
            if self.0.num == 0 {
                f64::NAN
            } else if self.0.num > 0 {
                f64::INFINITY
            } else {
                f64::NEG_INFINITY
            }
        } else {
            self.0.num as f64 / self.0.denom as f64
        }
    }

    /// Returns the negation of this value.
    pub fn neg(&self) -> Self {
        Self::new(-self.0.num, self.0.denom)
    }

    /// Returns the absolute value.
    pub fn abs(&self) -> Self {
        Self::new(self.0.num.abs(), self.0.denom.abs())
    }
}

impl Default for Numeric {
    fn default() -> Self {
        Self::zero()
    }
}

impl fmt::Debug for Numeric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Numeric({}/{})", self.0.num, self.0.denom)
    }
}

impl fmt::Display for Numeric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.denom == 1 {
            write!(f, "{}", self.0.num)
        } else if self.0.denom == 0 {
            write!(f, "NaN")
        } else {
            write!(f, "{}/{}", self.0.num, self.0.denom)
        }
    }
}

impl From<i64> for Numeric {
    fn from(n: i64) -> Self {
        Self::new(n, 1)
    }
}

impl From<ffi::gnc_numeric> for Numeric {
    fn from(n: ffi::gnc_numeric) -> Self {
        Self(n)
    }
}

impl From<Numeric> for ffi::gnc_numeric {
    fn from(n: Numeric) -> Self {
        n.0
    }
}

impl std::ops::Neg for Numeric {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Numeric::neg(&self)
    }
}
