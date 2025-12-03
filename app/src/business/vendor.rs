//! Safe wrapper for GncVendor.

use std::ffi::{CStr, CString};
use std::ptr::NonNull;

use gnucash_sys::ffi;
use gnucash_sys::{Book, Guid};

use super::{Address, Owner};

/// A vendor/supplier entity.
pub struct Vendor {
    ptr: NonNull<ffi::GncVendor>,
    #[allow(dead_code)]
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
    /// The pointer must be valid.
    pub unsafe fn from_raw(ptr: *mut ffi::GncVendor, owned: bool) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr, owned })
    }

    /// Returns the raw pointer.
    pub fn as_ptr(&self) -> *mut ffi::GncVendor {
        self.ptr.as_ptr()
    }

    /// Begins an edit session.
    pub fn begin_edit(&self) {
        unsafe { ffi::gncVendorBeginEdit(self.ptr.as_ptr()) }
    }

    /// Commits changes.
    pub fn commit_edit(&self) {
        unsafe { ffi::gncVendorCommitEdit(self.ptr.as_ptr()) }
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

    /// Creates an Owner from this vendor.
    pub fn to_owner(&self) -> Owner {
        let mut owner = Owner::new();
        unsafe { ffi::gncOwnerInitVendor(owner.as_mut_ptr(), self.ptr.as_ptr()) };
        owner
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

    /// Returns the notes.
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

    /// Returns the billing address.
    pub fn addr(&self) -> Option<Address> {
        unsafe {
            let ptr = ffi::gncVendorGetAddr(self.ptr.as_ptr());
            Address::from_raw(ptr, false)
        }
    }

    /// Returns true if the vendor is active.
    pub fn is_active(&self) -> bool {
        unsafe { ffi::gncVendorGetActive(self.ptr.as_ptr()) != 0 }
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

    /// Sets the notes.
    pub fn set_notes(&self, notes: &str) {
        let c_notes = CString::new(notes).unwrap();
        unsafe { ffi::gncVendorSetNotes(self.ptr.as_ptr(), c_notes.as_ptr()) }
    }

    /// Sets the active flag.
    pub fn set_active(&self, active: bool) {
        unsafe { ffi::gncVendorSetActive(self.ptr.as_ptr(), active as i32) }
    }

}

impl std::fmt::Debug for Vendor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vendor")
            .field("id", &self.id())
            .field("name", &self.name())
            .field("active", &self.is_active())
            .finish()
    }
}
