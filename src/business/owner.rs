//! Safe wrapper for GnuCash Owner.

use std::ffi::CStr;
use std::ptr::NonNull;

use crate::ffi;
use crate::Guid;

use super::{Address, Customer, Employee, Job, Vendor};

/// The type of owner.
pub use ffi::GncOwnerType as OwnerType;

/// A GnuCash Owner - a union type representing customer, vendor, employee, or job.
///
/// An Owner is used to represent the party associated with an invoice, bill, or
/// expense voucher. It can be a Customer, Vendor, Employee, or Job.
pub struct Owner {
    ptr: NonNull<ffi::GncOwner>,
    owned: bool,
}

unsafe impl Send for Owner {}

impl Owner {
    /// Creates a new Owner initialized with a customer.
    pub fn from_customer(customer: &Customer) -> Self {
        let owner = Box::new(ffi::_gncOwner {
            type_: ffi::GncOwnerType::GNC_OWNER_CUSTOMER,
            owner: ffi::_gncOwner__bindgen_ty_1 {
                undefined: std::ptr::null_mut(),
            },
            qof_temp: std::ptr::null_mut(),
        });
        let ptr = Box::into_raw(owner);
        unsafe {
            ffi::gncOwnerInitCustomer(ptr, customer.as_ptr());
        }
        Self {
            ptr: NonNull::new(ptr).unwrap(),
            owned: true,
        }
    }

    /// Creates a new Owner initialized with a vendor.
    pub fn from_vendor(vendor: &Vendor) -> Self {
        let owner = Box::new(ffi::_gncOwner {
            type_: ffi::GncOwnerType::GNC_OWNER_VENDOR,
            owner: ffi::_gncOwner__bindgen_ty_1 {
                undefined: std::ptr::null_mut(),
            },
            qof_temp: std::ptr::null_mut(),
        });
        let ptr = Box::into_raw(owner);
        unsafe {
            ffi::gncOwnerInitVendor(ptr, vendor.as_ptr());
        }
        Self {
            ptr: NonNull::new(ptr).unwrap(),
            owned: true,
        }
    }

    /// Creates a new Owner initialized with an employee.
    pub fn from_employee(employee: &Employee) -> Self {
        let owner = Box::new(ffi::_gncOwner {
            type_: ffi::GncOwnerType::GNC_OWNER_EMPLOYEE,
            owner: ffi::_gncOwner__bindgen_ty_1 {
                undefined: std::ptr::null_mut(),
            },
            qof_temp: std::ptr::null_mut(),
        });
        let ptr = Box::into_raw(owner);
        unsafe {
            ffi::gncOwnerInitEmployee(ptr, employee.as_ptr());
        }
        Self {
            ptr: NonNull::new(ptr).unwrap(),
            owned: true,
        }
    }

    /// Creates a new Owner initialized with a job.
    pub fn from_job(job: &Job) -> Self {
        let owner = Box::new(ffi::_gncOwner {
            type_: ffi::GncOwnerType::GNC_OWNER_JOB,
            owner: ffi::_gncOwner__bindgen_ty_1 {
                undefined: std::ptr::null_mut(),
            },
            qof_temp: std::ptr::null_mut(),
        });
        let ptr = Box::into_raw(owner);
        unsafe {
            ffi::gncOwnerInitJob(ptr, job.as_ptr());
        }
        Self {
            ptr: NonNull::new(ptr).unwrap(),
            owned: true,
        }
    }

    /// Creates an Owner wrapper from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be valid and point to a properly initialized GncOwner.
    pub unsafe fn from_raw(ptr: *mut ffi::GncOwner, owned: bool) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr, owned })
    }

    /// Returns the raw pointer to the underlying GncOwner.
    pub fn as_ptr(&self) -> *mut ffi::GncOwner {
        self.ptr.as_ptr()
    }

    /// Returns the type of this owner.
    pub fn owner_type(&self) -> OwnerType {
        unsafe { ffi::gncOwnerGetType(self.ptr.as_ptr()) }
    }

    /// Returns true if this is a valid owner.
    pub fn is_valid(&self) -> bool {
        unsafe { ffi::gncOwnerIsValid(self.ptr.as_ptr()) != 0 }
    }

    /// Returns the GUID of this owner.
    pub fn guid(&self) -> Guid {
        unsafe {
            let guid_ptr = ffi::gncOwnerGetGUID(self.ptr.as_ptr());
            if guid_ptr.is_null() {
                Guid::from_bytes([0; 16])
            } else {
                Guid::from_bytes((*guid_ptr).reserved)
            }
        }
    }

    /// Returns the owner ID.
    pub fn id(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncOwnerGetID(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the owner name.
    pub fn name(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncOwnerGetName(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the owner address.
    pub fn address(&self) -> Option<Address> {
        unsafe {
            let ptr = ffi::gncOwnerGetAddr(self.ptr.as_ptr());
            Address::from_raw(ptr, false)
        }
    }

    /// Returns true if the owner is active.
    pub fn is_active(&self) -> bool {
        unsafe { ffi::gncOwnerGetActive(self.ptr.as_ptr()) != 0 }
    }

    /// Sets whether the owner is active.
    pub fn set_active(&self, active: bool) {
        unsafe { ffi::gncOwnerSetActive(self.ptr.as_ptr(), active as i32) }
    }

    /// Returns the customer if this owner is a customer.
    pub fn as_customer(&self) -> Option<Customer> {
        if self.owner_type() != OwnerType::GNC_OWNER_CUSTOMER {
            return None;
        }
        unsafe {
            let ptr = ffi::gncOwnerGetCustomer(self.ptr.as_ptr());
            Customer::from_raw(ptr, false)
        }
    }

    /// Returns the vendor if this owner is a vendor.
    pub fn as_vendor(&self) -> Option<Vendor> {
        if self.owner_type() != OwnerType::GNC_OWNER_VENDOR {
            return None;
        }
        unsafe {
            let ptr = ffi::gncOwnerGetVendor(self.ptr.as_ptr());
            Vendor::from_raw(ptr, false)
        }
    }

    /// Returns the employee if this owner is an employee.
    pub fn as_employee(&self) -> Option<Employee> {
        if self.owner_type() != OwnerType::GNC_OWNER_EMPLOYEE {
            return None;
        }
        unsafe {
            let ptr = ffi::gncOwnerGetEmployee(self.ptr.as_ptr());
            Employee::from_raw(ptr, false)
        }
    }

    /// Returns the job if this owner is a job.
    pub fn as_job(&self) -> Option<Job> {
        if self.owner_type() != OwnerType::GNC_OWNER_JOB {
            return None;
        }
        unsafe {
            let ptr = ffi::gncOwnerGetJob(self.ptr.as_ptr());
            Job::from_raw(ptr, false)
        }
    }

    /// Returns the end owner (resolves jobs to their customer/vendor).
    pub fn end_owner(&self) -> Option<Owner> {
        unsafe {
            let ptr = ffi::gncOwnerGetEndOwner(self.ptr.as_ptr());
            // end_owner returns const pointer, cast to mutable for our wrapper
            Self::from_raw(ptr as *mut _, false)
        }
    }

    /// Copies this owner to another owner.
    pub fn copy_to(&self, dest: &mut Owner) {
        unsafe { ffi::gncOwnerCopy(self.ptr.as_ptr(), dest.ptr.as_ptr()) }
    }
}

impl Drop for Owner {
    fn drop(&mut self) {
        if self.owned {
            // Owner is a simple struct, we just need to drop the Box
            unsafe {
                let _ = Box::from_raw(self.ptr.as_ptr());
            }
        }
    }
}

impl std::fmt::Debug for Owner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Owner")
            .field("type", &self.owner_type())
            .field("id", &self.id())
            .field("name", &self.name())
            .field("active", &self.is_active())
            .finish()
    }
}

impl PartialEq for Owner {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ffi::gncOwnerEqual(self.ptr.as_ptr(), other.ptr.as_ptr()) != 0 }
    }
}

impl Eq for Owner {}
