//! Safe wrapper for GnuCash Job.

use std::ffi::{CStr, CString};
use std::ptr::NonNull;

use crate::ffi;
use crate::{Book, Guid, Numeric};

use super::Owner;

/// A GnuCash Job - a project associated with a customer or vendor.
pub struct Job {
    ptr: NonNull<ffi::GncJob>,
    owned: bool,
}

unsafe impl Send for Job {}

impl Job {
    /// Creates a new Job in the given book.
    pub fn new(book: &Book) -> Self {
        let ptr = unsafe { ffi::gncJobCreate(book.as_ptr()) };
        Self {
            ptr: NonNull::new(ptr).expect("gncJobCreate returned null"),
            owned: true,
        }
    }

    /// Creates a Job wrapper from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be valid and point to a properly initialized GncJob.
    pub unsafe fn from_raw(ptr: *mut ffi::GncJob, owned: bool) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr, owned })
    }

    /// Returns the raw pointer to the underlying GncJob.
    pub fn as_ptr(&self) -> *mut ffi::GncJob {
        self.ptr.as_ptr()
    }

    /// Returns the GUID of this job.
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

    /// Begins an edit session on this job.
    pub fn begin_edit(&self) {
        unsafe { ffi::gncJobBeginEdit(self.ptr.as_ptr()) }
    }

    /// Commits changes made during the edit session.
    pub fn commit_edit(&self) {
        unsafe { ffi::gncJobCommitEdit(self.ptr.as_ptr()) }
    }

    // ==================== Getters ====================

    /// Returns the job ID.
    pub fn id(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncJobGetID(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the job name.
    pub fn name(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncJobGetName(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the job reference.
    pub fn reference(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncJobGetReference(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the job rate.
    pub fn rate(&self) -> Numeric {
        unsafe { ffi::gncJobGetRate(self.ptr.as_ptr()).into() }
    }

    /// Returns the owner (customer or vendor) of this job.
    pub fn owner(&self) -> Option<Owner> {
        unsafe {
            let ptr = ffi::gncJobGetOwner(self.ptr.as_ptr());
            Owner::from_raw(ptr, false)
        }
    }

    /// Returns true if the job is active.
    pub fn is_active(&self) -> bool {
        unsafe { ffi::gncJobGetActive(self.ptr.as_ptr()) != 0 }
    }

    // ==================== Setters ====================

    /// Sets the job ID.
    pub fn set_id(&self, id: &str) {
        let c_id = CString::new(id).unwrap();
        unsafe { ffi::gncJobSetID(self.ptr.as_ptr(), c_id.as_ptr()) }
    }

    /// Sets the job name.
    pub fn set_name(&self, name: &str) {
        let c_name = CString::new(name).unwrap();
        unsafe { ffi::gncJobSetName(self.ptr.as_ptr(), c_name.as_ptr()) }
    }

    /// Sets the job reference.
    pub fn set_reference(&self, reference: &str) {
        let c_reference = CString::new(reference).unwrap();
        unsafe { ffi::gncJobSetReference(self.ptr.as_ptr(), c_reference.as_ptr()) }
    }

    /// Sets the job rate.
    pub fn set_rate(&self, rate: Numeric) {
        unsafe { ffi::gncJobSetRate(self.ptr.as_ptr(), rate.into()) }
    }

    /// Sets the owner (customer or vendor) of this job.
    pub fn set_owner(&self, owner: &Owner) {
        unsafe { ffi::gncJobSetOwner(self.ptr.as_ptr(), owner.as_ptr()) }
    }

    /// Sets whether the job is active.
    pub fn set_active(&self, active: bool) {
        unsafe { ffi::gncJobSetActive(self.ptr.as_ptr(), active as i32) }
    }
}

impl Drop for Job {
    fn drop(&mut self) {
        if self.owned {
            unsafe { ffi::gncJobDestroy(self.ptr.as_ptr()) }
        }
    }
}

impl std::fmt::Debug for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Job")
            .field("guid", &self.guid())
            .field("id", &self.id())
            .field("name", &self.name())
            .field("active", &self.is_active())
            .finish()
    }
}
