//! Safe wrapper for GncTaxTable and GncTaxTableEntry.

use std::ffi::{CStr, CString};
use std::ptr::NonNull;

use gnucash_sys::ffi;
use gnucash_sys::{Account, Book, Guid, Numeric};

pub use ffi::GncAmountType as AmountType;

/// A tax rate table.
pub struct TaxTable {
    ptr: NonNull<ffi::GncTaxTable>,
    #[allow(dead_code)]
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
    /// The pointer must be valid.
    pub unsafe fn from_raw(ptr: *mut ffi::GncTaxTable, owned: bool) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr, owned })
    }

    /// Returns the raw pointer.
    pub fn as_ptr(&self) -> *mut ffi::GncTaxTable {
        self.ptr.as_ptr()
    }

    /// Begins an edit session.
    pub fn begin_edit(&self) {
        unsafe { ffi::gncTaxTableBeginEdit(self.ptr.as_ptr()) }
    }

    /// Commits changes.
    pub fn commit_edit(&self) {
        unsafe { ffi::gncTaxTableCommitEdit(self.ptr.as_ptr()) }
    }

    /// Returns the GUID.
    pub fn guid(&self) -> Guid {
        unsafe {
            let instance = self.ptr.as_ptr() as *const std::ffi::c_void;
            let guid_ptr = ffi::qof_instance_get_guid(instance);
            if guid_ptr.is_null() {
                Guid::from_bytes([0; 16])
            } else {
                Guid::from_bytes((*guid_ptr).reserved)
            }
        }
    }

    // ==================== Getters ====================

    /// Returns the name.
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

    /// Returns the parent tax table.
    pub fn parent(&self) -> Option<TaxTable> {
        unsafe {
            let ptr = ffi::gncTaxTableGetParent(self.ptr.as_ptr());
            Self::from_raw(ptr, false)
        }
    }

    /// Returns the reference count.
    pub fn refcount(&self) -> i64 {
        unsafe { ffi::gncTaxTableGetRefcount(self.ptr.as_ptr()) }
    }

    /// Returns the entries as a vector.
    pub fn entries(&self) -> Vec<TaxTableEntry> {
        let mut result = Vec::new();
        unsafe {
            let list = ffi::gncTaxTableGetEntries(self.ptr.as_ptr());
            let mut current = list;
            while !current.is_null() {
                let entry_ptr = (*current).data as *mut ffi::GncTaxTableEntry;
                if let Some(entry) = TaxTableEntry::from_raw(entry_ptr) {
                    result.push(entry);
                }
                current = (*current).next;
            }
        }
        result
    }

    // ==================== Setters ====================

    /// Sets the name.
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

    // ==================== Lookup ====================

    /// Looks up a tax table by name.
    pub fn lookup_by_name(book: &Book, name: &str) -> Option<Self> {
        let c_name = CString::new(name).ok()?;
        unsafe {
            let ptr = ffi::gncTaxTableLookupByName(book.as_ptr(), c_name.as_ptr());
            Self::from_raw(ptr, false)
        }
    }
}

impl std::fmt::Debug for TaxTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TaxTable")
            .field("name", &self.name())
            .field("refcount", &self.refcount())
            .finish()
    }
}

/// An entry in a tax table.
pub struct TaxTableEntry {
    ptr: NonNull<ffi::GncTaxTableEntry>,
}

unsafe impl Send for TaxTableEntry {}

impl TaxTableEntry {
    /// Creates a new TaxTableEntry.
    pub fn new() -> Self {
        let ptr = unsafe { ffi::gncTaxTableEntryCreate() };
        Self {
            ptr: NonNull::new(ptr).expect("gncTaxTableEntryCreate returned null"),
        }
    }

    /// Creates a TaxTableEntry wrapper from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be valid.
    pub unsafe fn from_raw(ptr: *mut ffi::GncTaxTableEntry) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr })
    }

    /// Returns the raw pointer.
    pub fn as_ptr(&self) -> *mut ffi::GncTaxTableEntry {
        self.ptr.as_ptr()
    }

    // ==================== Getters ====================

    /// Returns the account.
    pub fn account(&self) -> Option<Account> {
        unsafe {
            let ptr = ffi::gncTaxTableEntryGetAccount(self.ptr.as_ptr());
            Account::from_raw(ptr, false)
        }
    }

    /// Returns the amount type.
    pub fn amount_type(&self) -> AmountType {
        unsafe { ffi::gncTaxTableEntryGetType(self.ptr.as_ptr()) }
    }

    /// Returns the amount.
    pub fn amount(&self) -> Numeric {
        unsafe { ffi::gncTaxTableEntryGetAmount(self.ptr.as_ptr()).into() }
    }

    // ==================== Setters ====================

    /// Sets the account.
    pub fn set_account(&self, account: &Account) {
        unsafe { ffi::gncTaxTableEntrySetAccount(self.ptr.as_ptr(), account.as_ptr()) }
    }

    /// Sets the amount type.
    pub fn set_type(&self, amount_type: AmountType) {
        unsafe { ffi::gncTaxTableEntrySetType(self.ptr.as_ptr(), amount_type) }
    }

    /// Sets the amount.
    pub fn set_amount(&self, amount: Numeric) {
        unsafe { ffi::gncTaxTableEntrySetAmount(self.ptr.as_ptr(), amount.into()) }
    }
}

impl Default for TaxTableEntry {
    fn default() -> Self {
        Self::new()
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
