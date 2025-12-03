//! Safe wrapper for GnuCash Customer.

use std::ffi::{CStr, CString};
use std::ptr::NonNull;

use crate::ffi;
use crate::{Book, Guid, Numeric};

use super::{Address, BillTerm, TaxTable};

/// A GnuCash Customer - someone who receives invoices.
pub struct Customer {
    ptr: NonNull<ffi::GncCustomer>,
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
    /// The pointer must be valid and point to a properly initialized GncCustomer.
    pub unsafe fn from_raw(ptr: *mut ffi::GncCustomer, owned: bool) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr, owned })
    }

    /// Returns the raw pointer to the underlying GncCustomer.
    pub fn as_ptr(&self) -> *mut ffi::GncCustomer {
        self.ptr.as_ptr()
    }

    /// Returns the GUID of this customer.
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

    /// Begins an edit session on this customer.
    pub fn begin_edit(&self) {
        unsafe { ffi::gncCustomerBeginEdit(self.ptr.as_ptr()) }
    }

    /// Commits changes made during the edit session.
    pub fn commit_edit(&self) {
        unsafe { ffi::gncCustomerCommitEdit(self.ptr.as_ptr()) }
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

    /// Returns the customer notes.
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
    pub fn address(&self) -> Option<Address> {
        unsafe {
            let ptr = ffi::gncCustomerGetAddr(self.ptr.as_ptr());
            Address::from_raw(ptr, false)
        }
    }

    /// Returns the shipping address.
    pub fn ship_address(&self) -> Option<Address> {
        unsafe {
            let ptr = ffi::gncCustomerGetShipAddr(self.ptr.as_ptr());
            Address::from_raw(ptr, false)
        }
    }

    /// Returns the payment terms.
    pub fn terms(&self) -> Option<BillTerm> {
        unsafe {
            let ptr = ffi::gncCustomerGetTerms(self.ptr.as_ptr());
            BillTerm::from_raw(ptr, false)
        }
    }

    /// Returns the tax included setting.
    pub fn tax_included(&self) -> ffi::GncTaxIncluded {
        unsafe { ffi::gncCustomerGetTaxIncluded(self.ptr.as_ptr()) }
    }

    /// Returns true if the customer is active.
    pub fn is_active(&self) -> bool {
        unsafe { ffi::gncCustomerGetActive(self.ptr.as_ptr()) != 0 }
    }

    /// Returns the discount amount.
    pub fn discount(&self) -> Numeric {
        unsafe { ffi::gncCustomerGetDiscount(self.ptr.as_ptr()).into() }
    }

    /// Returns the credit limit.
    pub fn credit(&self) -> Numeric {
        unsafe { ffi::gncCustomerGetCredit(self.ptr.as_ptr()).into() }
    }

    /// Returns true if tax table override is enabled.
    pub fn tax_table_override(&self) -> bool {
        unsafe { ffi::gncCustomerGetTaxTableOverride(self.ptr.as_ptr()) != 0 }
    }

    /// Returns the tax table.
    pub fn tax_table(&self) -> Option<TaxTable> {
        unsafe {
            let ptr = ffi::gncCustomerGetTaxTable(self.ptr.as_ptr());
            TaxTable::from_raw(ptr, false)
        }
    }

    /// Returns true if the customer has been modified.
    pub fn is_dirty(&self) -> bool {
        unsafe { ffi::gncCustomerIsDirty(self.ptr.as_ptr()) != 0 }
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

    /// Sets the customer notes.
    pub fn set_notes(&self, notes: &str) {
        let c_notes = CString::new(notes).unwrap();
        unsafe { ffi::gncCustomerSetNotes(self.ptr.as_ptr(), c_notes.as_ptr()) }
    }

    /// Sets the payment terms.
    pub fn set_terms(&self, terms: &BillTerm) {
        unsafe { ffi::gncCustomerSetTerms(self.ptr.as_ptr(), terms.as_ptr()) }
    }

    /// Sets the tax included setting.
    pub fn set_tax_included(&self, tax_included: ffi::GncTaxIncluded) {
        unsafe { ffi::gncCustomerSetTaxIncluded(self.ptr.as_ptr(), tax_included) }
    }

    /// Sets whether the customer is active.
    pub fn set_active(&self, active: bool) {
        unsafe { ffi::gncCustomerSetActive(self.ptr.as_ptr(), active as i32) }
    }

    /// Sets the discount amount.
    pub fn set_discount(&self, discount: Numeric) {
        unsafe { ffi::gncCustomerSetDiscount(self.ptr.as_ptr(), discount.into()) }
    }

    /// Sets the credit limit.
    pub fn set_credit(&self, credit: Numeric) {
        unsafe { ffi::gncCustomerSetCredit(self.ptr.as_ptr(), credit.into()) }
    }

    /// Sets whether to override the tax table.
    pub fn set_tax_table_override(&self, override_: bool) {
        unsafe { ffi::gncCustomerSetTaxTableOverride(self.ptr.as_ptr(), override_ as i32) }
    }

    /// Sets the tax table.
    pub fn set_tax_table(&self, table: &TaxTable) {
        unsafe { ffi::gncCustomerSetTaxTable(self.ptr.as_ptr(), table.as_ptr()) }
    }
}

impl Drop for Customer {
    fn drop(&mut self) {
        if self.owned {
            unsafe { ffi::gncCustomerDestroy(self.ptr.as_ptr()) }
        }
    }
}

impl std::fmt::Debug for Customer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Customer")
            .field("guid", &self.guid())
            .field("id", &self.id())
            .field("name", &self.name())
            .field("active", &self.is_active())
            .finish()
    }
}
