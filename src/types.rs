//! Safe Rust wrappers for GnuCash core types.

use std::fmt;

use crate::ffi;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// GUID encoding length (32 hex characters).
pub const GUID_ENCODING_LENGTH: usize = 32;

/// A 128-bit globally unique identifier.
///
/// # Examples
///
/// ```ignore
/// use gnucash_sys::Guid;
///
/// // Create a new random GUID
/// let guid = Guid::new();
/// println!("GUID: {}", guid);
///
/// // Parse from hex string
/// let parsed = Guid::parse("12345678901234567890123456789012").unwrap();
///
/// // Check for null GUID
/// assert!(!guid.is_null());
/// ```
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

    /// Returns a reference to the inner GncGUID.
    pub fn as_ffi(&self) -> &ffi::GncGUID {
        &self.0
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
///
/// Used for precise financial calculations without floating-point rounding errors.
///
/// # Examples
///
/// ```
/// use gnucash_sys::Numeric;
///
/// // Create $100.00 (10000 cents / 100)
/// let amount = Numeric::new(10000, 100);
/// assert_eq!(amount.to_f64(), 100.0);
///
/// // Create from integer
/// let whole: Numeric = 42i64.into();
/// assert_eq!(whole.num(), 42);
/// assert_eq!(whole.denom(), 1);
///
/// // Negate a value
/// let neg = -amount;
/// assert!(neg.is_negative());
/// ```
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

// ==================== Serde Support ====================

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;
    use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

    impl Serialize for Guid {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(&self.to_string())
        }
    }

    impl<'de> Deserialize<'de> for Guid {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            Guid::parse(&s).ok_or_else(|| de::Error::custom("invalid GUID format"))
        }
    }

    impl Serialize for Numeric {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            use serde::ser::SerializeStruct;
            let mut state = serializer.serialize_struct("Numeric", 2)?;
            state.serialize_field("num", &self.num())?;
            state.serialize_field("denom", &self.denom())?;
            state.end()
        }
    }

    impl<'de> Deserialize<'de> for Numeric {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            #[derive(Deserialize)]
            struct NumericData {
                num: i64,
                denom: i64,
            }
            let data = NumericData::deserialize(deserializer)?;
            Ok(Numeric::new(data.num, data.denom))
        }
    }
}
