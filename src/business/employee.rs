//! Safe wrapper for GnuCash Employee.

use std::ffi::{CStr, CString};
use std::ptr::NonNull;

use crate::ffi;
use crate::{Account, Book, Guid, Numeric};

use super::Address;

/// A GnuCash Employee - someone who can submit expense vouchers.
pub struct Employee {
    ptr: NonNull<ffi::GncEmployee>,
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
    /// The pointer must be valid and point to a properly initialized GncEmployee.
    pub unsafe fn from_raw(ptr: *mut ffi::GncEmployee, owned: bool) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr, owned })
    }

    /// Returns the raw pointer to the underlying GncEmployee.
    pub fn as_ptr(&self) -> *mut ffi::GncEmployee {
        self.ptr.as_ptr()
    }

    /// Returns the GUID of this employee.
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

    /// Begins an edit session on this employee.
    pub fn begin_edit(&self) {
        unsafe { ffi::gncEmployeeBeginEdit(self.ptr.as_ptr()) }
    }

    /// Commits changes made during the edit session.
    pub fn commit_edit(&self) {
        unsafe { ffi::gncEmployeeCommitEdit(self.ptr.as_ptr()) }
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

    /// Returns the employee name.
    pub fn name(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncEmployeeGetName(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the employee address.
    pub fn address(&self) -> Option<Address> {
        unsafe {
            let ptr = ffi::gncEmployeeGetAddr(self.ptr.as_ptr());
            Address::from_raw(ptr, false)
        }
    }

    /// Returns the employee language.
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

    /// Returns the employee ACL.
    pub fn acl(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncEmployeeGetAcl(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the workday hours.
    pub fn workday(&self) -> Numeric {
        unsafe { ffi::gncEmployeeGetWorkday(self.ptr.as_ptr()).into() }
    }

    /// Returns the hourly rate.
    pub fn rate(&self) -> Numeric {
        unsafe { ffi::gncEmployeeGetRate(self.ptr.as_ptr()).into() }
    }

    /// Returns true if the employee is active.
    pub fn is_active(&self) -> bool {
        unsafe { ffi::gncEmployeeGetActive(self.ptr.as_ptr()) != 0 }
    }

    /// Returns the credit card account for expense reimbursement.
    pub fn credit_card_account(&self) -> Option<Account> {
        unsafe {
            let ptr = ffi::gncEmployeeGetCCard(self.ptr.as_ptr());
            Account::from_raw(ptr, false)
        }
    }

    /// Returns true if the employee has been modified.
    pub fn is_dirty(&self) -> bool {
        unsafe { ffi::gncEmployeeIsDirty(self.ptr.as_ptr()) != 0 }
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

    /// Sets the employee name.
    pub fn set_name(&self, name: &str) {
        let c_name = CString::new(name).unwrap();
        unsafe { ffi::gncEmployeeSetName(self.ptr.as_ptr(), c_name.as_ptr()) }
    }

    /// Sets the employee language.
    pub fn set_language(&self, language: &str) {
        let c_language = CString::new(language).unwrap();
        unsafe { ffi::gncEmployeeSetLanguage(self.ptr.as_ptr(), c_language.as_ptr()) }
    }

    /// Sets the employee ACL.
    pub fn set_acl(&self, acl: &str) {
        let c_acl = CString::new(acl).unwrap();
        unsafe { ffi::gncEmployeeSetAcl(self.ptr.as_ptr(), c_acl.as_ptr()) }
    }

    /// Sets the workday hours.
    pub fn set_workday(&self, workday: Numeric) {
        unsafe { ffi::gncEmployeeSetWorkday(self.ptr.as_ptr(), workday.into()) }
    }

    /// Sets the hourly rate.
    pub fn set_rate(&self, rate: Numeric) {
        unsafe { ffi::gncEmployeeSetRate(self.ptr.as_ptr(), rate.into()) }
    }

    /// Sets whether the employee is active.
    pub fn set_active(&self, active: bool) {
        unsafe { ffi::gncEmployeeSetActive(self.ptr.as_ptr(), active as i32) }
    }

    /// Sets the credit card account for expense reimbursement.
    pub fn set_credit_card_account(&self, account: &Account) {
        unsafe { ffi::gncEmployeeSetCCard(self.ptr.as_ptr(), account.as_ptr()) }
    }
}

impl Drop for Employee {
    fn drop(&mut self) {
        if self.owned {
            unsafe { ffi::gncEmployeeDestroy(self.ptr.as_ptr()) }
        }
    }
}

impl std::fmt::Debug for Employee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Employee")
            .field("guid", &self.guid())
            .field("id", &self.id())
            .field("name", &self.name())
            .field("username", &self.username())
            .field("active", &self.is_active())
            .finish()
    }
}
