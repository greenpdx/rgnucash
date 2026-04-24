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

use std::ffi::{CStr, CString};
use std::ptr::NonNull;

use gnucash_sys::ffi;
use gnucash_sys::Book;

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

    /// Construct a new commodity. The most common use is creating a
    /// currency entry for a book that's being built from scratch —
    /// `Commodity::new(&book, "Costa Rican Colón", "CURRENCY", "CRC", None, 100)`.
    ///
    /// The resulting commodity is not registered with the book's
    /// commodity table; pair with [`CommodityTable::insert`] when you
    /// want it looked up by namespace/mnemonic later.
    pub fn new(
        book: &Book,
        fullname: &str,
        namespace: &str,
        mnemonic: &str,
        cusip: Option<&str>,
        fraction: i32,
    ) -> Option<Self> {
        let c_fullname = CString::new(fullname).ok()?;
        let c_namespace = CString::new(namespace).ok()?;
        let c_mnemonic = CString::new(mnemonic).ok()?;
        let c_cusip = cusip.and_then(|s| CString::new(s).ok());
        let cusip_ptr = c_cusip.as_ref().map_or(std::ptr::null(), |c| c.as_ptr());
        unsafe {
            let ptr = ffi::gnc_commodity_new(
                book.as_ptr(),
                c_fullname.as_ptr(),
                c_namespace.as_ptr(),
                c_mnemonic.as_ptr(),
                cusip_ptr,
                fraction,
            );
            Self::from_raw(ptr, true)
        }
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

/// The commodity table attached to a `Book`. Exactly one per book;
/// created lazily on first access via `gnc_commodity_table_get_table`.
/// Used to register currencies and securities so later lookups by
/// (namespace, mnemonic) succeed.
pub struct CommodityTable {
    ptr: NonNull<ffi::gnc_commodity_table>,
}

unsafe impl Send for CommodityTable {}

impl CommodityTable {
    /// # Safety
    /// Pointer must be valid.
    pub unsafe fn from_raw(ptr: *mut ffi::gnc_commodity_table) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr })
    }

    pub fn as_ptr(&self) -> *mut ffi::gnc_commodity_table {
        self.ptr.as_ptr()
    }

    /// Look up a commodity by namespace + mnemonic. Returns `None` if
    /// the table doesn't contain a matching entry.
    pub fn lookup(&self, namespace: &str, mnemonic: &str) -> Option<Commodity> {
        let c_ns = CString::new(namespace).ok()?;
        let c_mn = CString::new(mnemonic).ok()?;
        unsafe {
            let ptr = ffi::gnc_commodity_table_lookup(
                self.ptr.as_ptr(),
                c_ns.as_ptr(),
                c_mn.as_ptr(),
            );
            Commodity::from_raw(ptr, false)
        }
    }

    /// Insert a commodity into the table. If an equivalent entry is
    /// already present libgnucash returns the existing one and
    /// leaves the freshly-constructed commodity up to the GC; the
    /// returned `Commodity` is the authoritative table entry.
    pub fn insert(&self, commodity: &Commodity) -> Option<Commodity> {
        unsafe {
            let ptr = ffi::gnc_commodity_table_insert(self.ptr.as_ptr(), commodity.as_ptr());
            Commodity::from_raw(ptr, false)
        }
    }
}

impl std::fmt::Debug for CommodityTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommodityTable")
            .field("ptr", &self.ptr.as_ptr())
            .finish()
    }
}
