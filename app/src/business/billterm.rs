//! Safe wrapper for GncBillTerm.

use std::ffi::{CStr, CString};
use std::ptr::NonNull;

use gnucash_sys::ffi;
use gnucash_sys::{Book, Guid, Numeric};

pub use ffi::GncBillTermType as BillTermType;

/// Payment terms for invoices.
pub struct BillTerm {
    ptr: NonNull<ffi::GncBillTerm>,
    #[allow(dead_code)]
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
    /// The pointer must be valid.
    pub unsafe fn from_raw(ptr: *mut ffi::GncBillTerm, owned: bool) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr, owned })
    }

    /// Returns the raw pointer.
    pub fn as_ptr(&self) -> *mut ffi::GncBillTerm {
        self.ptr.as_ptr()
    }

    /// Begins an edit session.
    pub fn begin_edit(&self) {
        unsafe { ffi::gncBillTermBeginEdit(self.ptr.as_ptr()) }
    }

    /// Commits changes.
    pub fn commit_edit(&self) {
        unsafe { ffi::gncBillTermCommitEdit(self.ptr.as_ptr()) }
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
            let ptr = ffi::gncBillTermGetName(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the description.
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
    pub fn term_type(&self) -> BillTermType {
        unsafe { ffi::gncBillTermGetType(self.ptr.as_ptr()) }
    }

    /// Returns the due days.
    pub fn due_days(&self) -> i32 {
        unsafe { ffi::gncBillTermGetDueDays(self.ptr.as_ptr()) }
    }

    /// Returns the discount days.
    pub fn discount_days(&self) -> i32 {
        unsafe { ffi::gncBillTermGetDiscountDays(self.ptr.as_ptr()) }
    }

    /// Returns the discount.
    pub fn discount(&self) -> Numeric {
        unsafe { ffi::gncBillTermGetDiscount(self.ptr.as_ptr()).into() }
    }

    /// Returns the cutoff day.
    pub fn cutoff(&self) -> i32 {
        unsafe { ffi::gncBillTermGetCutoff(self.ptr.as_ptr()) }
    }

    /// Returns the reference count.
    pub fn refcount(&self) -> i64 {
        unsafe { ffi::gncBillTermGetRefcount(self.ptr.as_ptr()) }
    }

    // ==================== Setters ====================

    /// Sets the name.
    pub fn set_name(&self, name: &str) {
        let c_name = CString::new(name).unwrap();
        unsafe { ffi::gncBillTermSetName(self.ptr.as_ptr(), c_name.as_ptr()) }
    }

    /// Sets the description.
    pub fn set_description(&self, desc: &str) {
        let c_desc = CString::new(desc).unwrap();
        unsafe { ffi::gncBillTermSetDescription(self.ptr.as_ptr(), c_desc.as_ptr()) }
    }

    /// Sets the bill term type.
    pub fn set_type(&self, term_type: BillTermType) {
        unsafe { ffi::gncBillTermSetType(self.ptr.as_ptr(), term_type) }
    }

    /// Sets the due days.
    pub fn set_due_days(&self, days: i32) {
        unsafe { ffi::gncBillTermSetDueDays(self.ptr.as_ptr(), days) }
    }

    /// Sets the discount days.
    pub fn set_discount_days(&self, days: i32) {
        unsafe { ffi::gncBillTermSetDiscountDays(self.ptr.as_ptr(), days) }
    }

    /// Sets the discount.
    pub fn set_discount(&self, discount: Numeric) {
        unsafe { ffi::gncBillTermSetDiscount(self.ptr.as_ptr(), discount.into()) }
    }

    /// Sets the cutoff day.
    pub fn set_cutoff(&self, cutoff: i32) {
        unsafe { ffi::gncBillTermSetCutoff(self.ptr.as_ptr(), cutoff) }
    }

    // ==================== Lookup ====================

    /// Looks up a bill term by name.
    pub fn lookup_by_name(book: &Book, name: &str) -> Option<Self> {
        let c_name = CString::new(name).ok()?;
        unsafe {
            let ptr = ffi::gncBillTermLookupByName(book.as_ptr(), c_name.as_ptr());
            Self::from_raw(ptr, false)
        }
    }
}

impl std::fmt::Debug for BillTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BillTerm")
            .field("name", &self.name())
            .field("type", &self.term_type())
            .field("due_days", &self.due_days())
            .finish()
    }
}
