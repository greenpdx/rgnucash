//! Safe wrapper for gnc_commodity.
//!
//! A commodity in GnuCash is a currency (USD, CRC, EUR ...), a security
//! (stock, bond), or any other tradable unit. Every `Account`,
//! `Transaction`, and `Invoice` is denominated in exactly one commodity.
//!
//! The underlying `gnc_commodity_s` structure is opaque — all access
//! goes through the `gnc_commodity_*` getter family, which this wrapper
//! covers for the fields most callers need (mnemonic, namespace,
//! fullname, fraction).

use std::ffi::CStr;
use std::ptr::NonNull;

use gnucash_sys::ffi;

/// A GnuCash commodity — currency, security, or other tradable unit.
pub struct Commodity {
    ptr: NonNull<ffi::gnc_commodity>,
    #[allow(dead_code)]
    owned: bool,
}

unsafe impl Send for Commodity {}

impl Commodity {
    /// Creates a Commodity wrapper from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be valid and point to a live `gnc_commodity`.
    pub unsafe fn from_raw(ptr: *mut ffi::gnc_commodity, owned: bool) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr, owned })
    }

    /// Returns the raw pointer.
    pub fn as_ptr(&self) -> *mut ffi::gnc_commodity {
        self.ptr.as_ptr()
    }

    /// Returns the commodity mnemonic — "USD", "CRC", ticker symbols, etc.
    pub fn mnemonic(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gnc_commodity_get_mnemonic(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the commodity namespace — "CURRENCY", "NYSE", "NASDAQ", etc.
    pub fn namespace(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gnc_commodity_get_namespace(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the human-readable full name ("United States Dollar").
    pub fn fullname(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gnc_commodity_get_fullname(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the combined "NAMESPACE::MNEMONIC" unique name — the form
    /// GnuCash uses to round-trip commodities in its XML backend.
    pub fn unique_name(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gnc_commodity_get_unique_name(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the print name for display ("USD (US Dollar)").
    pub fn printname(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gnc_commodity_get_printname(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the fraction — smallest divisible unit, e.g. 100 for USD
    /// (cents), 1000 for most precious metals, 1 for share-denominated
    /// securities.
    pub fn fraction(&self) -> i32 {
        unsafe { ffi::gnc_commodity_get_fraction(self.ptr.as_ptr()) as i32 }
    }
}

impl std::fmt::Debug for Commodity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Commodity")
            .field("namespace", &self.namespace())
            .field("mnemonic", &self.mnemonic())
            .field("fraction", &self.fraction())
            .finish()
    }
}
