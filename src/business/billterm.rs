//! Safe wrapper for GnuCash BillTerm.

use std::ffi::{CStr, CString};
use std::ptr::NonNull;

use crate::ffi;
use crate::{Book, Guid, Numeric};

/// A GnuCash BillTerm - payment terms for invoices.
pub struct BillTerm {
    ptr: NonNull<ffi::GncBillTerm>,
    owned: bool,
}

unsafe impl Send for BillTerm {}

impl BillTerm {
    /// Creates a new BillTerm in the given book.
    pub fn new(book: &Book) -> Self {
        let ptr = unsafe { ffi::gncBillTermCreate(book.as_ptr()) };
        Self {
            ptr: NonNull::new(ptr).expect("gncBillTermCreate returned null"),
            owned: true,
        }
    }

    /// Creates a BillTerm wrapper from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be valid and point to a properly initialized GncBillTerm.
    pub unsafe fn from_raw(ptr: *mut ffi::GncBillTerm, owned: bool) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr, owned })
    }

    /// Returns the raw pointer to the underlying GncBillTerm.
    pub fn as_ptr(&self) -> *mut ffi::GncBillTerm {
        self.ptr.as_ptr()
    }

    /// Returns the GUID of this bill term.
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

    /// Begins an edit session on this bill term.
    pub fn begin_edit(&self) {
        unsafe { ffi::gncBillTermBeginEdit(self.ptr.as_ptr()) }
    }

    /// Commits changes made during the edit session.
    pub fn commit_edit(&self) {
        unsafe { ffi::gncBillTermCommitEdit(self.ptr.as_ptr()) }
    }

    // ==================== Getters ====================

    /// Returns the bill term name.
    pub fn name(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncBillTermGetName(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the bill term description.
    pub fn description(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncBillTermGetDescription(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the bill term type.
    pub fn term_type(&self) -> ffi::GncBillTermType {
        unsafe { ffi::gncBillTermGetType(self.ptr.as_ptr()) }
    }

    /// Returns the number of days until due.
    pub fn due_days(&self) -> i32 {
        unsafe { ffi::gncBillTermGetDueDays(self.ptr.as_ptr()) }
    }

    /// Returns the discount days.
    pub fn discount_days(&self) -> i32 {
        unsafe { ffi::gncBillTermGetDiscountDays(self.ptr.as_ptr()) }
    }

    /// Returns the discount percentage.
    pub fn discount(&self) -> Numeric {
        unsafe { ffi::gncBillTermGetDiscount(self.ptr.as_ptr()).into() }
    }

    /// Returns the cutoff day (for proximo terms).
    pub fn cutoff(&self) -> i32 {
        unsafe { ffi::gncBillTermGetCutoff(self.ptr.as_ptr()) }
    }

    /// Returns the reference count.
    pub fn ref_count(&self) -> i64 {
        unsafe { ffi::gncBillTermGetRefcount(self.ptr.as_ptr()) }
    }

    // ==================== Setters ====================

    /// Sets the bill term name.
    pub fn set_name(&self, name: &str) {
        let c_name = CString::new(name).unwrap();
        unsafe { ffi::gncBillTermSetName(self.ptr.as_ptr(), c_name.as_ptr()) }
    }

    /// Sets the bill term description.
    pub fn set_description(&self, desc: &str) {
        let c_desc = CString::new(desc).unwrap();
        unsafe { ffi::gncBillTermSetDescription(self.ptr.as_ptr(), c_desc.as_ptr()) }
    }

    /// Sets the bill term type.
    pub fn set_type(&self, term_type: ffi::GncBillTermType) {
        unsafe { ffi::gncBillTermSetType(self.ptr.as_ptr(), term_type) }
    }

    /// Sets the number of days until due.
    pub fn set_due_days(&self, days: i32) {
        unsafe { ffi::gncBillTermSetDueDays(self.ptr.as_ptr(), days) }
    }

    /// Sets the discount days.
    pub fn set_discount_days(&self, days: i32) {
        unsafe { ffi::gncBillTermSetDiscountDays(self.ptr.as_ptr(), days) }
    }

    /// Sets the discount percentage.
    pub fn set_discount(&self, discount: Numeric) {
        unsafe { ffi::gncBillTermSetDiscount(self.ptr.as_ptr(), discount.into()) }
    }

    /// Sets the cutoff day (for proximo terms).
    pub fn set_cutoff(&self, cutoff: i32) {
        unsafe { ffi::gncBillTermSetCutoff(self.ptr.as_ptr(), cutoff) }
    }

    /// Increments the reference count.
    pub fn inc_ref(&self) {
        unsafe { ffi::gncBillTermIncRef(self.ptr.as_ptr()) }
    }

    /// Decrements the reference count.
    pub fn dec_ref(&self) {
        unsafe { ffi::gncBillTermDecRef(self.ptr.as_ptr()) }
    }
}

impl Drop for BillTerm {
    fn drop(&mut self) {
        if self.owned {
            unsafe { ffi::gncBillTermDestroy(self.ptr.as_ptr()) }
        }
    }
}

impl std::fmt::Debug for BillTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BillTerm")
            .field("guid", &self.guid())
            .field("name", &self.name())
            .field("type", &self.term_type())
            .field("due_days", &self.due_days())
            .finish()
    }
}
