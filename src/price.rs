//! Safe wrappers for GnuCash Price and PriceDB.

use std::ffi::{CStr, CString};
use std::ptr::NonNull;

use crate::ffi;
use crate::{Book, Guid, Numeric};

/// Re-export PriceSource enum.
pub use ffi::PriceSource;

/// A GnuCash Price - a price quote for a commodity.
pub struct Price {
    ptr: NonNull<ffi::GNCPrice>,
    owned: bool,
}

unsafe impl Send for Price {}

impl Price {
    /// Creates a new Price in the given book.
    pub fn new(book: &Book) -> Self {
        let ptr = unsafe { ffi::gnc_price_create(book.as_ptr()) };
        Self {
            ptr: NonNull::new(ptr).expect("gnc_price_create returned null"),
            owned: true,
        }
    }

    /// Creates a Price wrapper from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be valid and point to a properly initialized GNCPrice.
    pub unsafe fn from_raw(ptr: *mut ffi::GNCPrice, owned: bool) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr, owned })
    }

    /// Returns the raw pointer to the underlying GNCPrice.
    pub fn as_ptr(&self) -> *mut ffi::GNCPrice {
        self.ptr.as_ptr()
    }

    /// Increments the reference count.
    pub fn ref_(&self) {
        unsafe { ffi::gnc_price_ref(self.ptr.as_ptr()) }
    }

    /// Decrements the reference count.
    pub fn unref(&self) {
        unsafe { ffi::gnc_price_unref(self.ptr.as_ptr()) }
    }

    /// Begins an edit session on this price.
    pub fn begin_edit(&self) {
        unsafe { ffi::gnc_price_begin_edit(self.ptr.as_ptr()) }
    }

    /// Commits changes made during the edit session.
    pub fn commit_edit(&self) {
        unsafe { ffi::gnc_price_commit_edit(self.ptr.as_ptr()) }
    }

    /// Creates a clone of this price in the given book.
    pub fn clone_in_book(&self, book: &Book) -> Option<Price> {
        unsafe {
            let ptr = ffi::gnc_price_clone(self.ptr.as_ptr(), book.as_ptr());
            Self::from_raw(ptr, true)
        }
    }

    /// Creates an inverted price (1/price).
    pub fn invert(&self) -> Option<Price> {
        unsafe {
            let ptr = ffi::gnc_price_invert(self.ptr.as_ptr());
            Self::from_raw(ptr, true)
        }
    }

    // ==================== Getters ====================

    /// Returns the time of this price quote.
    pub fn time(&self) -> i64 {
        unsafe { ffi::gnc_price_get_time64(self.ptr.as_ptr()) }
    }

    /// Returns the price source.
    pub fn source(&self) -> PriceSource {
        unsafe { ffi::gnc_price_get_source(self.ptr.as_ptr()) }
    }

    /// Returns the price source as a string.
    pub fn source_string(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gnc_price_get_source_string(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the price type string.
    pub fn type_string(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gnc_price_get_typestr(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the price value.
    pub fn value(&self) -> Numeric {
        unsafe { ffi::gnc_price_get_value(self.ptr.as_ptr()).into() }
    }

    // ==================== Setters ====================

    /// Sets the time of this price quote.
    pub fn set_time(&self, time: i64) {
        unsafe { ffi::gnc_price_set_time64(self.ptr.as_ptr(), time) }
    }

    /// Sets the price source.
    pub fn set_source(&self, source: PriceSource) {
        unsafe { ffi::gnc_price_set_source(self.ptr.as_ptr(), source) }
    }

    /// Sets the price source from a string.
    ///
    /// # Panics
    ///
    /// Panics if `source` contains a null byte.
    pub fn set_source_string(&self, source: &str) {
        let c_source = CString::new(source).unwrap();
        unsafe { ffi::gnc_price_set_source_string(self.ptr.as_ptr(), c_source.as_ptr()) }
    }

    /// Sets the price type string.
    ///
    /// # Panics
    ///
    /// Panics if `type_str` contains a null byte.
    pub fn set_type_string(&self, type_str: &str) {
        let c_type = CString::new(type_str).unwrap();
        unsafe { ffi::gnc_price_set_typestr(self.ptr.as_ptr(), c_type.as_ptr()) }
    }

    /// Sets the price value.
    pub fn set_value(&self, value: Numeric) {
        unsafe { ffi::gnc_price_set_value(self.ptr.as_ptr(), value.into()) }
    }
}

impl Drop for Price {
    fn drop(&mut self) {
        if self.owned {
            unsafe { ffi::gnc_price_unref(self.ptr.as_ptr()) }
        }
    }
}

impl std::fmt::Debug for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Price")
            .field("time", &self.time())
            .field("source", &self.source())
            .field("value", &self.value())
            .finish()
    }
}

impl PartialEq for Price {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ffi::gnc_price_equal(self.ptr.as_ptr(), other.ptr.as_ptr()) != 0 }
    }
}

impl Eq for Price {}

impl std::hash::Hash for Price {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash by time and value since those define price identity
        self.time().hash(state);
        self.value().num().hash(state);
        self.value().denom().hash(state);
    }
}

/// A GnuCash PriceDB - a database of price quotes.
pub struct PriceDB {
    ptr: NonNull<ffi::GNCPriceDB>,
    owned: bool,
}

unsafe impl Send for PriceDB {}

impl PriceDB {
    /// Gets the price database for a book.
    pub fn get_db(book: &Book) -> Option<Self> {
        unsafe {
            let ptr = ffi::gnc_pricedb_get_db(book.as_ptr());
            Self::from_raw(ptr, false)
        }
    }

    /// Creates a PriceDB wrapper from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be valid and point to a properly initialized GNCPriceDB.
    pub unsafe fn from_raw(ptr: *mut ffi::GNCPriceDB, owned: bool) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr, owned })
    }

    /// Returns the raw pointer to the underlying GNCPriceDB.
    pub fn as_ptr(&self) -> *mut ffi::GNCPriceDB {
        self.ptr.as_ptr()
    }

    /// Begins an edit session on this price database.
    pub fn begin_edit(&self) {
        unsafe { ffi::gnc_pricedb_begin_edit(self.ptr.as_ptr()) }
    }

    /// Commits changes made during the edit session.
    pub fn commit_edit(&self) {
        unsafe { ffi::gnc_pricedb_commit_edit(self.ptr.as_ptr()) }
    }

    /// Sets bulk update mode.
    pub fn set_bulk_update(&self, bulk_update: bool) {
        unsafe { ffi::gnc_pricedb_set_bulk_update(self.ptr.as_ptr(), bulk_update as i32) }
    }

    /// Adds a price to the database.
    pub fn add_price(&self, price: &Price) -> bool {
        unsafe { ffi::gnc_pricedb_add_price(self.ptr.as_ptr(), price.as_ptr()) != 0 }
    }

    /// Removes a price from the database.
    pub fn remove_price(&self, price: &Price) -> bool {
        unsafe { ffi::gnc_pricedb_remove_price(self.ptr.as_ptr(), price.as_ptr()) != 0 }
    }

    /// Looks up a price by GUID.
    pub fn lookup_by_guid(guid: &Guid, book: &Book) -> Option<Price> {
        unsafe {
            let ptr = ffi::gnc_price_lookup(guid.as_ffi(), book.as_ptr());
            Price::from_raw(ptr, false)
        }
    }
}

impl Drop for PriceDB {
    fn drop(&mut self) {
        if self.owned {
            unsafe { ffi::gnc_pricedb_destroy(self.ptr.as_ptr()) }
        }
    }
}

impl std::fmt::Debug for PriceDB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PriceDB").finish()
    }
}
