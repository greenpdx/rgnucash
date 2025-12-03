//! Safe wrapper for GncAddress.

use std::ffi::{CStr, CString};
use std::ptr::NonNull;

use gnucash_sys::ffi;
use gnucash_sys::Book;

/// A mailing address.
pub struct Address {
    ptr: NonNull<ffi::GncAddress>,
    #[allow(dead_code)]
    owned: bool,
}

unsafe impl Send for Address {}

impl Address {
    /// Creates a new Address in the given book.
    pub fn new(book: &Book) -> Self {
        let ptr = unsafe { ffi::gncAddressCreate(book.as_ptr(), std::ptr::null_mut()) };
        Self {
            ptr: NonNull::new(ptr).expect("gncAddressCreate returned null"),
            owned: true,
        }
    }

    /// Creates an Address wrapper from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be valid.
    pub unsafe fn from_raw(ptr: *mut ffi::GncAddress, owned: bool) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr, owned })
    }

    /// Returns the raw pointer.
    pub fn as_ptr(&self) -> *mut ffi::GncAddress {
        self.ptr.as_ptr()
    }

    // ==================== Getters ====================

    /// Returns the name.
    pub fn name(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncAddressGetName(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns address line 1.
    pub fn addr1(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncAddressGetAddr1(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns address line 2.
    pub fn addr2(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncAddressGetAddr2(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns address line 3.
    pub fn addr3(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncAddressGetAddr3(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns address line 4.
    pub fn addr4(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncAddressGetAddr4(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the phone number.
    pub fn phone(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncAddressGetPhone(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the fax number.
    pub fn fax(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncAddressGetFax(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the email address.
    pub fn email(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncAddressGetEmail(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    // ==================== Setters ====================

    /// Sets the name.
    pub fn set_name(&self, name: &str) {
        let c_name = CString::new(name).unwrap();
        unsafe { ffi::gncAddressSetName(self.ptr.as_ptr(), c_name.as_ptr()) }
    }

    /// Sets address line 1.
    pub fn set_addr1(&self, addr: &str) {
        let c_addr = CString::new(addr).unwrap();
        unsafe { ffi::gncAddressSetAddr1(self.ptr.as_ptr(), c_addr.as_ptr()) }
    }

    /// Sets address line 2.
    pub fn set_addr2(&self, addr: &str) {
        let c_addr = CString::new(addr).unwrap();
        unsafe { ffi::gncAddressSetAddr2(self.ptr.as_ptr(), c_addr.as_ptr()) }
    }

    /// Sets address line 3.
    pub fn set_addr3(&self, addr: &str) {
        let c_addr = CString::new(addr).unwrap();
        unsafe { ffi::gncAddressSetAddr3(self.ptr.as_ptr(), c_addr.as_ptr()) }
    }

    /// Sets address line 4.
    pub fn set_addr4(&self, addr: &str) {
        let c_addr = CString::new(addr).unwrap();
        unsafe { ffi::gncAddressSetAddr4(self.ptr.as_ptr(), c_addr.as_ptr()) }
    }

    /// Sets the phone number.
    pub fn set_phone(&self, phone: &str) {
        let c_phone = CString::new(phone).unwrap();
        unsafe { ffi::gncAddressSetPhone(self.ptr.as_ptr(), c_phone.as_ptr()) }
    }

    /// Sets the fax number.
    pub fn set_fax(&self, fax: &str) {
        let c_fax = CString::new(fax).unwrap();
        unsafe { ffi::gncAddressSetFax(self.ptr.as_ptr(), c_fax.as_ptr()) }
    }

    /// Sets the email address.
    pub fn set_email(&self, email: &str) {
        let c_email = CString::new(email).unwrap();
        unsafe { ffi::gncAddressSetEmail(self.ptr.as_ptr(), c_email.as_ptr()) }
    }

    /// Clears the address (sets all fields to empty).
    pub fn clear(&self) {
        unsafe { ffi::gncAddressClearDirty(self.ptr.as_ptr()) }
    }
}

impl std::fmt::Debug for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Address")
            .field("name", &self.name())
            .field("addr1", &self.addr1())
            .field("addr2", &self.addr2())
            .field("phone", &self.phone())
            .field("email", &self.email())
            .finish()
    }
}
