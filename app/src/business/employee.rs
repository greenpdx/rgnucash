//! Safe wrapper for GncEmployee.

use std::ffi::{CStr, CString};
use std::ptr::NonNull;

use gnucash_sys::ffi;
use gnucash_sys::{Book, Guid, Numeric};

use super::{Address, Owner};

/// An employee entity.
pub struct Employee {
    ptr: NonNull<ffi::GncEmployee>,
    #[allow(dead_code)]
    owned: bool,
}

unsafe impl Send for Employee {}

impl Employee {
    /// Creates a new Employee in the given book.
    pub fn new(book: &Book) -> Self {
        let ptr = unsafe { ffi::gncEmployeeCreate(book.as_ptr()) };
        Self {
            ptr: NonNull::new(ptr).expect("gncEmployeeCreate returned null"),
            owned: true,
        }
    }

    /// Creates an Employee wrapper from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be valid.
    pub unsafe fn from_raw(ptr: *mut ffi::GncEmployee, owned: bool) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr, owned })
    }

    /// Returns the raw pointer.
    pub fn as_ptr(&self) -> *mut ffi::GncEmployee {
        self.ptr.as_ptr()
    }

    /// Begins an edit session.
    pub fn begin_edit(&self) {
        unsafe { ffi::gncEmployeeBeginEdit(self.ptr.as_ptr()) }
    }

    /// Commits changes.
    pub fn commit_edit(&self) {
        unsafe { ffi::gncEmployeeCommitEdit(self.ptr.as_ptr()) }
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

    /// Creates an Owner from this employee.
    pub fn to_owner(&self) -> Owner {
        let mut owner = Owner::new();
        unsafe { ffi::gncOwnerInitEmployee(owner.as_mut_ptr(), self.ptr.as_ptr()) };
        owner
    }

    // ==================== Getters ====================

    /// Returns the employee ID.
    pub fn id(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncEmployeeGetID(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the employee username.
    pub fn username(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncEmployeeGetUsername(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the language.
    pub fn language(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncEmployeeGetLanguage(self.ptr.as_ptr());
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
            let ptr = ffi::gncEmployeeGetAddr(self.ptr.as_ptr());
            Address::from_raw(ptr, false)
        }
    }

    /// Returns the workday (hours per day).
    pub fn workday(&self) -> Numeric {
        unsafe { ffi::gncEmployeeGetWorkday(self.ptr.as_ptr()).into() }
    }

    /// Returns the rate (hourly rate).
    pub fn rate(&self) -> Numeric {
        unsafe { ffi::gncEmployeeGetRate(self.ptr.as_ptr()).into() }
    }

    /// Returns true if the employee is active.
    pub fn is_active(&self) -> bool {
        unsafe { ffi::gncEmployeeGetActive(self.ptr.as_ptr()) != 0 }
    }

    // ==================== Setters ====================

    /// Sets the employee ID.
    pub fn set_id(&self, id: &str) {
        let c_id = CString::new(id).unwrap();
        unsafe { ffi::gncEmployeeSetID(self.ptr.as_ptr(), c_id.as_ptr()) }
    }

    /// Sets the employee username.
    pub fn set_username(&self, username: &str) {
        let c_username = CString::new(username).unwrap();
        unsafe { ffi::gncEmployeeSetUsername(self.ptr.as_ptr(), c_username.as_ptr()) }
    }

    /// Sets the language.
    pub fn set_language(&self, language: &str) {
        let c_language = CString::new(language).unwrap();
        unsafe { ffi::gncEmployeeSetLanguage(self.ptr.as_ptr(), c_language.as_ptr()) }
    }

    /// Sets the workday (hours per day).
    pub fn set_workday(&self, workday: Numeric) {
        unsafe { ffi::gncEmployeeSetWorkday(self.ptr.as_ptr(), workday.into()) }
    }

    /// Sets the rate (hourly rate).
    pub fn set_rate(&self, rate: Numeric) {
        unsafe { ffi::gncEmployeeSetRate(self.ptr.as_ptr(), rate.into()) }
    }

    /// Sets the active flag.
    pub fn set_active(&self, active: bool) {
        unsafe { ffi::gncEmployeeSetActive(self.ptr.as_ptr(), active as i32) }
    }

}

impl std::fmt::Debug for Employee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Employee")
            .field("id", &self.id())
            .field("username", &self.username())
            .field("active", &self.is_active())
            .finish()
    }
}
