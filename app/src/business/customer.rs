//! Safe wrapper for GncCustomer.

use std::ffi::{CStr, CString};
use std::ptr::NonNull;

use gnucash_sys::ffi;
use gnucash_sys::{Book, Guid, Numeric};

use super::{Address, Owner};

/// A customer entity.
pub struct Customer {
    ptr: NonNull<ffi::GncCustomer>,
    #[allow(dead_code)]
    owned: bool,
}

unsafe impl Send for Customer {}

impl Customer {
    /// Creates a new Customer in the given book.
    pub fn new(book: &Book) -> Self {
        let ptr = unsafe { ffi::gncCustomerCreate(book.as_ptr()) };
        Self {
            ptr: NonNull::new(ptr).expect("gncCustomerCreate returned null"),
            owned: true,
        }
    }

    /// Creates a Customer wrapper from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be valid.
    pub unsafe fn from_raw(ptr: *mut ffi::GncCustomer, owned: bool) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr, owned })
    }

    /// Returns the raw pointer.
    pub fn as_ptr(&self) -> *mut ffi::GncCustomer {
        self.ptr.as_ptr()
    }

    /// Begins an edit session.
    pub fn begin_edit(&self) {
        unsafe { ffi::gncCustomerBeginEdit(self.ptr.as_ptr()) }
    }

    /// Commits changes.
    pub fn commit_edit(&self) {
        unsafe { ffi::gncCustomerCommitEdit(self.ptr.as_ptr()) }
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

    /// Creates an Owner from this customer.
    pub fn to_owner(&self) -> Owner {
        let mut owner = Owner::new();
        unsafe { ffi::gncOwnerInitCustomer(owner.as_mut_ptr(), self.ptr.as_ptr()) };
        owner
    }

    // ==================== Getters ====================

    /// Returns the customer ID.
    pub fn id(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncCustomerGetID(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the customer name.
    pub fn name(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncCustomerGetName(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the notes.
    pub fn notes(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncCustomerGetNotes(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the billing address.
    pub fn addr(&self) -> Option<Address> {
        unsafe {
            let ptr = ffi::gncCustomerGetAddr(self.ptr.as_ptr());
            Address::from_raw(ptr, false)
        }
    }

    /// Returns the shipping address.
    pub fn ship_addr(&self) -> Option<Address> {
        unsafe {
            let ptr = ffi::gncCustomerGetShipAddr(self.ptr.as_ptr());
            Address::from_raw(ptr, false)
        }
    }

    /// Returns the discount.
    pub fn discount(&self) -> Numeric {
        unsafe { ffi::gncCustomerGetDiscount(self.ptr.as_ptr()).into() }
    }

    /// Returns the credit limit.
    pub fn credit(&self) -> Numeric {
        unsafe { ffi::gncCustomerGetCredit(self.ptr.as_ptr()).into() }
    }

    /// Returns true if the customer is active.
    pub fn is_active(&self) -> bool {
        unsafe { ffi::gncCustomerGetActive(self.ptr.as_ptr()) != 0 }
    }

    // ==================== Setters ====================

    /// Sets the customer ID.
    pub fn set_id(&self, id: &str) {
        let c_id = CString::new(id).unwrap();
        unsafe { ffi::gncCustomerSetID(self.ptr.as_ptr(), c_id.as_ptr()) }
    }

    /// Sets the customer name.
    pub fn set_name(&self, name: &str) {
        let c_name = CString::new(name).unwrap();
        unsafe { ffi::gncCustomerSetName(self.ptr.as_ptr(), c_name.as_ptr()) }
    }

    /// Sets the notes.
    pub fn set_notes(&self, notes: &str) {
        let c_notes = CString::new(notes).unwrap();
        unsafe { ffi::gncCustomerSetNotes(self.ptr.as_ptr(), c_notes.as_ptr()) }
    }

    /// Sets the discount.
    pub fn set_discount(&self, discount: Numeric) {
        unsafe { ffi::gncCustomerSetDiscount(self.ptr.as_ptr(), discount.into()) }
    }

    /// Sets the credit limit.
    pub fn set_credit(&self, credit: Numeric) {
        unsafe { ffi::gncCustomerSetCredit(self.ptr.as_ptr(), credit.into()) }
    }

    /// Sets the active flag.
    pub fn set_active(&self, active: bool) {
        unsafe { ffi::gncCustomerSetActive(self.ptr.as_ptr(), active as i32) }
    }

}

impl std::fmt::Debug for Customer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Customer")
            .field("id", &self.id())
            .field("name", &self.name())
            .field("active", &self.is_active())
            .finish()
    }
}
