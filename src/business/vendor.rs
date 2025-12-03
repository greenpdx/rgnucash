//! Safe wrapper for GnuCash Vendor.

use std::ffi::{CStr, CString};
use std::ptr::NonNull;

use crate::ffi;
use crate::{Book, Guid};

use super::{Address, BillTerm, TaxTable};

/// A GnuCash Vendor - someone who sends bills.
pub struct Vendor {
    ptr: NonNull<ffi::GncVendor>,
    owned: bool,
}

unsafe impl Send for Vendor {}

impl Vendor {
    /// Creates a new Vendor in the given book.
    pub fn new(book: &Book) -> Self {
        let ptr = unsafe { ffi::gncVendorCreate(book.as_ptr()) };
        Self {
            ptr: NonNull::new(ptr).expect("gncVendorCreate returned null"),
            owned: true,
        }
    }

    /// Creates a Vendor wrapper from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be valid and point to a properly initialized GncVendor.
    pub unsafe fn from_raw(ptr: *mut ffi::GncVendor, owned: bool) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr, owned })
    }

    /// Returns the raw pointer to the underlying GncVendor.
    pub fn as_ptr(&self) -> *mut ffi::GncVendor {
        self.ptr.as_ptr()
    }

    /// Returns the GUID of this vendor.
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

    /// Begins an edit session on this vendor.
    pub fn begin_edit(&self) {
        unsafe { ffi::gncVendorBeginEdit(self.ptr.as_ptr()) }
    }

    /// Commits changes made during the edit session.
    pub fn commit_edit(&self) {
        unsafe { ffi::gncVendorCommitEdit(self.ptr.as_ptr()) }
    }

    // ==================== Getters ====================

    /// Returns the vendor ID.
    pub fn id(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncVendorGetID(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the vendor name.
    pub fn name(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncVendorGetName(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the vendor notes.
    pub fn notes(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncVendorGetNotes(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the vendor address.
    pub fn address(&self) -> Option<Address> {
        unsafe {
            let ptr = ffi::gncVendorGetAddr(self.ptr.as_ptr());
            Address::from_raw(ptr, false)
        }
    }

    /// Returns the payment terms.
    pub fn terms(&self) -> Option<BillTerm> {
        unsafe {
            let ptr = ffi::gncVendorGetTerms(self.ptr.as_ptr());
            BillTerm::from_raw(ptr, false)
        }
    }

    /// Returns the tax included setting.
    pub fn tax_included(&self) -> ffi::GncTaxIncluded {
        unsafe { ffi::gncVendorGetTaxIncluded(self.ptr.as_ptr()) }
    }

    /// Returns true if the vendor is active.
    pub fn is_active(&self) -> bool {
        unsafe { ffi::gncVendorGetActive(self.ptr.as_ptr()) != 0 }
    }

    /// Returns true if tax table override is enabled.
    pub fn tax_table_override(&self) -> bool {
        unsafe { ffi::gncVendorGetTaxTableOverride(self.ptr.as_ptr()) != 0 }
    }

    /// Returns the tax table.
    pub fn tax_table(&self) -> Option<TaxTable> {
        unsafe {
            let ptr = ffi::gncVendorGetTaxTable(self.ptr.as_ptr());
            TaxTable::from_raw(ptr, false)
        }
    }

    /// Returns true if the vendor has been modified.
    pub fn is_dirty(&self) -> bool {
        unsafe { ffi::gncVendorIsDirty(self.ptr.as_ptr()) != 0 }
    }

    // ==================== Setters ====================

    /// Sets the vendor ID.
    pub fn set_id(&self, id: &str) {
        let c_id = CString::new(id).unwrap();
        unsafe { ffi::gncVendorSetID(self.ptr.as_ptr(), c_id.as_ptr()) }
    }

    /// Sets the vendor name.
    pub fn set_name(&self, name: &str) {
        let c_name = CString::new(name).unwrap();
        unsafe { ffi::gncVendorSetName(self.ptr.as_ptr(), c_name.as_ptr()) }
    }

    /// Sets the vendor notes.
    pub fn set_notes(&self, notes: &str) {
        let c_notes = CString::new(notes).unwrap();
        unsafe { ffi::gncVendorSetNotes(self.ptr.as_ptr(), c_notes.as_ptr()) }
    }

    /// Sets the payment terms.
    pub fn set_terms(&self, terms: &BillTerm) {
        unsafe { ffi::gncVendorSetTerms(self.ptr.as_ptr(), terms.as_ptr()) }
    }

    /// Sets the tax included setting.
    pub fn set_tax_included(&self, tax_included: ffi::GncTaxIncluded) {
        unsafe { ffi::gncVendorSetTaxIncluded(self.ptr.as_ptr(), tax_included) }
    }

    /// Sets whether the vendor is active.
    pub fn set_active(&self, active: bool) {
        unsafe { ffi::gncVendorSetActive(self.ptr.as_ptr(), active as i32) }
    }

    /// Sets whether to override the tax table.
    pub fn set_tax_table_override(&self, override_: bool) {
        unsafe { ffi::gncVendorSetTaxTableOverride(self.ptr.as_ptr(), override_ as i32) }
    }

    /// Sets the tax table.
    pub fn set_tax_table(&self, table: &TaxTable) {
        unsafe { ffi::gncVendorSetTaxTable(self.ptr.as_ptr(), table.as_ptr()) }
    }
}

impl Drop for Vendor {
    fn drop(&mut self) {
        if self.owned {
            unsafe { ffi::gncVendorDestroy(self.ptr.as_ptr()) }
        }
    }
}

impl std::fmt::Debug for Vendor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vendor")
            .field("guid", &self.guid())
            .field("id", &self.id())
            .field("name", &self.name())
            .field("active", &self.is_active())
            .finish()
    }
}
