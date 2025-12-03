//! Safe wrapper for GncJob.

use std::ffi::{CStr, CString};
use std::ptr::NonNull;

use gnucash_sys::ffi;
use gnucash_sys::{Book, Guid};

use super::Owner;

/// A job entity linked to a customer.
pub struct Job {
    ptr: NonNull<ffi::GncJob>,
    #[allow(dead_code)]
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
    /// The pointer must be valid.
    pub unsafe fn from_raw(ptr: *mut ffi::GncJob, owned: bool) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr, owned })
    }

    /// Returns the raw pointer.
    pub fn as_ptr(&self) -> *mut ffi::GncJob {
        self.ptr.as_ptr()
    }

    /// Begins an edit session.
    pub fn begin_edit(&self) {
        unsafe { ffi::gncJobBeginEdit(self.ptr.as_ptr()) }
    }

    /// Commits changes.
    pub fn commit_edit(&self) {
        unsafe { ffi::gncJobCommitEdit(self.ptr.as_ptr()) }
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

    /// Creates an Owner from this job.
    pub fn to_owner(&self) -> Owner {
        let mut owner = Owner::new();
        unsafe { ffi::gncOwnerInitJob(owner.as_mut_ptr(), self.ptr.as_ptr()) };
        owner
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

    /// Returns the owner of this job.
    pub fn owner(&self) -> Owner {
        unsafe {
            let owner_ptr = ffi::gncJobGetOwner(self.ptr.as_ptr());
            if owner_ptr.is_null() {
                Owner::new()
            } else {
                Owner::from_raw(*owner_ptr)
            }
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
        let c_ref = CString::new(reference).unwrap();
        unsafe { ffi::gncJobSetReference(self.ptr.as_ptr(), c_ref.as_ptr()) }
    }

    /// Sets the owner of this job.
    pub fn set_owner(&self, owner: &Owner) {
        unsafe { ffi::gncJobSetOwner(self.ptr.as_ptr(), owner.as_ptr()) }
    }

    /// Sets the active flag.
    pub fn set_active(&self, active: bool) {
        unsafe { ffi::gncJobSetActive(self.ptr.as_ptr(), active as i32) }
    }

}

impl std::fmt::Debug for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Job")
            .field("id", &self.id())
            .field("name", &self.name())
            .field("active", &self.is_active())
            .finish()
    }
}
