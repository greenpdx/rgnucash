//! Safe wrapper for GnuCash TaxTable.

use std::ffi::{CStr, CString};
use std::ptr::NonNull;

use crate::ffi;
use crate::{Account, Book, Guid, Numeric};

/// A GnuCash TaxTable - a collection of tax rates.
pub struct TaxTable {
    ptr: NonNull<ffi::GncTaxTable>,
    owned: bool,
}

unsafe impl Send for TaxTable {}

impl TaxTable {
    /// Creates a new TaxTable in the given book.
    pub fn new(book: &Book) -> Self {
        let ptr = unsafe { ffi::gncTaxTableCreate(book.as_ptr()) };
        Self {
            ptr: NonNull::new(ptr).expect("gncTaxTableCreate returned null"),
            owned: true,
        }
    }

    /// Creates a TaxTable wrapper from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be valid and point to a properly initialized GncTaxTable.
    pub unsafe fn from_raw(ptr: *mut ffi::GncTaxTable, owned: bool) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr, owned })
    }

    /// Returns the raw pointer to the underlying GncTaxTable.
    pub fn as_ptr(&self) -> *mut ffi::GncTaxTable {
        self.ptr.as_ptr()
    }

    /// Returns the GUID of this tax table.
    pub fn guid(&self) -> Guid {
        unsafe {
            let guid_ptr =
                ffi::qof_instance_get_guid(self.ptr.as_ptr() as *const std::ffi::c_void);
            if guid_ptr.is_null() {
                Guid::from_bytes([0; 16])
            } else {
                Guid::from_bytes((*guid_ptr).reserved)
            }
        }
    }

    /// Begins an edit session on this tax table.
    pub fn begin_edit(&self) {
        unsafe { ffi::gncTaxTableBeginEdit(self.ptr.as_ptr()) }
    }

    /// Commits changes made during the edit session.
    pub fn commit_edit(&self) {
        unsafe { ffi::gncTaxTableCommitEdit(self.ptr.as_ptr()) }
    }

    // ==================== Getters ====================

    /// Returns the tax table name.
    pub fn name(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncTaxTableGetName(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the reference count.
    pub fn ref_count(&self) -> i64 {
        unsafe { ffi::gncTaxTableGetRefcount(self.ptr.as_ptr()) }
    }

    // ==================== Setters ====================

    /// Sets the tax table name.
    pub fn set_name(&self, name: &str) {
        let c_name = CString::new(name).unwrap();
        unsafe { ffi::gncTaxTableSetName(self.ptr.as_ptr(), c_name.as_ptr()) }
    }

    /// Adds an entry to this tax table.
    pub fn add_entry(&self, entry: &TaxTableEntry) {
        unsafe { ffi::gncTaxTableAddEntry(self.ptr.as_ptr(), entry.as_ptr()) }
    }

    /// Removes an entry from this tax table.
    pub fn remove_entry(&self, entry: &TaxTableEntry) {
        unsafe { ffi::gncTaxTableRemoveEntry(self.ptr.as_ptr(), entry.as_ptr()) }
    }

    /// Increments the reference count.
    pub fn inc_ref(&self) {
        unsafe { ffi::gncTaxTableIncRef(self.ptr.as_ptr()) }
    }

    /// Decrements the reference count.
    pub fn dec_ref(&self) {
        unsafe { ffi::gncTaxTableDecRef(self.ptr.as_ptr()) }
    }
}

impl Drop for TaxTable {
    fn drop(&mut self) {
        if self.owned {
            unsafe { ffi::gncTaxTableDestroy(self.ptr.as_ptr()) }
        }
    }
}

impl std::fmt::Debug for TaxTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TaxTable")
            .field("guid", &self.guid())
            .field("name", &self.name())
            .finish()
    }
}

/// A single entry in a TaxTable.
pub struct TaxTableEntry {
    ptr: NonNull<ffi::GncTaxTableEntry>,
    owned: bool,
}

unsafe impl Send for TaxTableEntry {}

impl TaxTableEntry {
    /// Creates a new TaxTableEntry.
    pub fn new() -> Self {
        let ptr = unsafe { ffi::gncTaxTableEntryCreate() };
        Self {
            ptr: NonNull::new(ptr).expect("gncTaxTableEntryCreate returned null"),
            owned: true,
        }
    }

    /// Creates a TaxTableEntry wrapper from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be valid and point to a properly initialized GncTaxTableEntry.
    pub unsafe fn from_raw(ptr: *mut ffi::GncTaxTableEntry, owned: bool) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr, owned })
    }

    /// Returns the raw pointer to the underlying GncTaxTableEntry.
    pub fn as_ptr(&self) -> *mut ffi::GncTaxTableEntry {
        self.ptr.as_ptr()
    }

    // ==================== Getters ====================

    /// Returns the account for this tax entry.
    pub fn account(&self) -> Option<Account> {
        unsafe {
            let ptr = ffi::gncTaxTableEntryGetAccount(self.ptr.as_ptr());
            Account::from_raw(ptr, false)
        }
    }

    /// Returns the tax amount type.
    pub fn amount_type(&self) -> ffi::GncAmountType {
        unsafe { ffi::gncTaxTableEntryGetType(self.ptr.as_ptr()) }
    }

    /// Returns the tax amount/percentage.
    pub fn amount(&self) -> Numeric {
        unsafe { ffi::gncTaxTableEntryGetAmount(self.ptr.as_ptr()).into() }
    }

    // ==================== Setters ====================

    /// Sets the account for this tax entry.
    pub fn set_account(&self, account: &Account) {
        unsafe { ffi::gncTaxTableEntrySetAccount(self.ptr.as_ptr(), account.as_ptr()) }
    }

    /// Sets the tax amount type.
    pub fn set_type(&self, amount_type: ffi::GncAmountType) {
        unsafe { ffi::gncTaxTableEntrySetType(self.ptr.as_ptr(), amount_type) }
    }

    /// Sets the tax amount/percentage.
    pub fn set_amount(&self, amount: Numeric) {
        unsafe { ffi::gncTaxTableEntrySetAmount(self.ptr.as_ptr(), amount.into()) }
    }
}

impl Default for TaxTableEntry {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for TaxTableEntry {
    fn drop(&mut self) {
        if self.owned {
            unsafe { ffi::gncTaxTableEntryDestroy(self.ptr.as_ptr()) }
        }
    }
}

impl std::fmt::Debug for TaxTableEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TaxTableEntry")
            .field("amount_type", &self.amount_type())
            .field("amount", &self.amount())
            .finish()
    }
}
