//! Safe wrapper for GnuCash Entry.

use std::ffi::{CStr, CString};
use std::ptr::NonNull;

use crate::ffi;
use crate::{Account, Book, Guid, Numeric};

use super::{Owner, TaxTable};

/// A GnuCash Entry - a line item in an invoice.
pub struct Entry {
    ptr: NonNull<ffi::GncEntry>,
    owned: bool,
}

unsafe impl Send for Entry {}

impl Entry {
    /// Creates a new Entry in the given book.
    pub fn new(book: &Book) -> Self {
        let ptr = unsafe { ffi::gncEntryCreate(book.as_ptr()) };
        Self {
            ptr: NonNull::new(ptr).expect("gncEntryCreate returned null"),
            owned: true,
        }
    }

    /// Creates an Entry wrapper from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be valid and point to a properly initialized GncEntry.
    pub unsafe fn from_raw(ptr: *mut ffi::GncEntry, owned: bool) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr, owned })
    }

    /// Returns the raw pointer to the underlying GncEntry.
    pub fn as_ptr(&self) -> *mut ffi::GncEntry {
        self.ptr.as_ptr()
    }

    /// Returns the GUID of this entry.
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

    /// Begins an edit session on this entry.
    pub fn begin_edit(&self) {
        unsafe { ffi::gncEntryBeginEdit(self.ptr.as_ptr()) }
    }

    /// Commits changes made during the edit session.
    pub fn commit_edit(&self) {
        unsafe { ffi::gncEntryCommitEdit(self.ptr.as_ptr()) }
    }

    // ==================== Common Getters ====================

    /// Returns the entry date.
    pub fn date(&self) -> i64 {
        unsafe { ffi::gncEntryGetDate(self.ptr.as_ptr()) }
    }

    /// Returns the date entered.
    pub fn date_entered(&self) -> i64 {
        unsafe { ffi::gncEntryGetDateEntered(self.ptr.as_ptr()) }
    }

    /// Returns the entry description.
    pub fn description(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncEntryGetDescription(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the entry action.
    pub fn action(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncEntryGetAction(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the entry notes.
    pub fn notes(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncEntryGetNotes(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the quantity.
    pub fn quantity(&self) -> Numeric {
        unsafe { ffi::gncEntryGetQuantity(self.ptr.as_ptr()).into() }
    }

    // ==================== Invoice-specific Getters ====================

    /// Returns the invoice account.
    pub fn inv_account(&self) -> Option<Account> {
        unsafe {
            let ptr = ffi::gncEntryGetInvAccount(self.ptr.as_ptr());
            Account::from_raw(ptr, false)
        }
    }

    /// Returns the invoice price.
    pub fn inv_price(&self) -> Numeric {
        unsafe { ffi::gncEntryGetInvPrice(self.ptr.as_ptr()).into() }
    }

    /// Returns the invoice discount.
    pub fn inv_discount(&self) -> Numeric {
        unsafe { ffi::gncEntryGetInvDiscount(self.ptr.as_ptr()).into() }
    }

    /// Returns the invoice discount type.
    pub fn inv_discount_type(&self) -> ffi::GncAmountType {
        unsafe { ffi::gncEntryGetInvDiscountType(self.ptr.as_ptr()) }
    }

    /// Returns how the invoice discount is applied.
    pub fn inv_discount_how(&self) -> ffi::GncDiscountHow {
        unsafe { ffi::gncEntryGetInvDiscountHow(self.ptr.as_ptr()) }
    }

    /// Returns true if the invoice entry is taxable.
    pub fn inv_taxable(&self) -> bool {
        unsafe { ffi::gncEntryGetInvTaxable(self.ptr.as_ptr()) != 0 }
    }

    /// Returns true if tax is included in the invoice price.
    pub fn inv_tax_included(&self) -> bool {
        unsafe { ffi::gncEntryGetInvTaxIncluded(self.ptr.as_ptr()) != 0 }
    }

    /// Returns the invoice tax table.
    pub fn inv_tax_table(&self) -> Option<TaxTable> {
        unsafe {
            let ptr = ffi::gncEntryGetInvTaxTable(self.ptr.as_ptr());
            TaxTable::from_raw(ptr, false)
        }
    }

    // ==================== Bill-specific Getters ====================

    /// Returns the bill account.
    pub fn bill_account(&self) -> Option<Account> {
        unsafe {
            let ptr = ffi::gncEntryGetBillAccount(self.ptr.as_ptr());
            Account::from_raw(ptr, false)
        }
    }

    /// Returns the bill price.
    pub fn bill_price(&self) -> Numeric {
        unsafe { ffi::gncEntryGetBillPrice(self.ptr.as_ptr()).into() }
    }

    /// Returns true if the bill entry is taxable.
    pub fn bill_taxable(&self) -> bool {
        unsafe { ffi::gncEntryGetBillTaxable(self.ptr.as_ptr()) != 0 }
    }

    /// Returns true if tax is included in the bill price.
    pub fn bill_tax_included(&self) -> bool {
        unsafe { ffi::gncEntryGetBillTaxIncluded(self.ptr.as_ptr()) != 0 }
    }

    /// Returns the bill tax table.
    pub fn bill_tax_table(&self) -> Option<TaxTable> {
        unsafe {
            let ptr = ffi::gncEntryGetBillTaxTable(self.ptr.as_ptr());
            TaxTable::from_raw(ptr, false)
        }
    }

    /// Returns true if this entry is billable.
    pub fn billable(&self) -> bool {
        unsafe { ffi::gncEntryGetBillable(self.ptr.as_ptr()) != 0 }
    }

    /// Returns the bill payment type.
    pub fn bill_payment(&self) -> ffi::GncEntryPaymentType {
        unsafe { ffi::gncEntryGetBillPayment(self.ptr.as_ptr()) }
    }

    // ==================== Common Setters ====================

    /// Sets the entry date.
    pub fn set_date(&self, date: i64) {
        unsafe { ffi::gncEntrySetDate(self.ptr.as_ptr(), date) }
    }

    /// Sets the date entered.
    pub fn set_date_entered(&self, date: i64) {
        unsafe { ffi::gncEntrySetDateEntered(self.ptr.as_ptr(), date) }
    }

    /// Sets the entry description.
    pub fn set_description(&self, desc: &str) {
        let c_desc = CString::new(desc).unwrap();
        unsafe { ffi::gncEntrySetDescription(self.ptr.as_ptr(), c_desc.as_ptr()) }
    }

    /// Sets the entry action.
    pub fn set_action(&self, action: &str) {
        let c_action = CString::new(action).unwrap();
        unsafe { ffi::gncEntrySetAction(self.ptr.as_ptr(), c_action.as_ptr()) }
    }

    /// Sets the entry notes.
    pub fn set_notes(&self, notes: &str) {
        let c_notes = CString::new(notes).unwrap();
        unsafe { ffi::gncEntrySetNotes(self.ptr.as_ptr(), c_notes.as_ptr()) }
    }

    /// Sets the quantity.
    pub fn set_quantity(&self, quantity: Numeric) {
        unsafe { ffi::gncEntrySetQuantity(self.ptr.as_ptr(), quantity.into()) }
    }

    // ==================== Invoice-specific Setters ====================

    /// Sets the invoice account.
    pub fn set_inv_account(&self, account: &Account) {
        unsafe { ffi::gncEntrySetInvAccount(self.ptr.as_ptr(), account.as_ptr()) }
    }

    /// Sets the invoice price.
    pub fn set_inv_price(&self, price: Numeric) {
        unsafe { ffi::gncEntrySetInvPrice(self.ptr.as_ptr(), price.into()) }
    }

    /// Sets the invoice discount.
    pub fn set_inv_discount(&self, discount: Numeric) {
        unsafe { ffi::gncEntrySetInvDiscount(self.ptr.as_ptr(), discount.into()) }
    }

    /// Sets the invoice discount type.
    pub fn set_inv_discount_type(&self, discount_type: ffi::GncAmountType) {
        unsafe { ffi::gncEntrySetInvDiscountType(self.ptr.as_ptr(), discount_type) }
    }

    /// Sets how the invoice discount is applied.
    pub fn set_inv_discount_how(&self, discount_how: ffi::GncDiscountHow) {
        unsafe { ffi::gncEntrySetInvDiscountHow(self.ptr.as_ptr(), discount_how) }
    }

    /// Sets whether the invoice entry is taxable.
    pub fn set_inv_taxable(&self, taxable: bool) {
        unsafe { ffi::gncEntrySetInvTaxable(self.ptr.as_ptr(), taxable as i32) }
    }

    /// Sets whether tax is included in the invoice price.
    pub fn set_inv_tax_included(&self, included: bool) {
        unsafe { ffi::gncEntrySetInvTaxIncluded(self.ptr.as_ptr(), included as i32) }
    }

    /// Sets the invoice tax table.
    pub fn set_inv_tax_table(&self, table: &TaxTable) {
        unsafe { ffi::gncEntrySetInvTaxTable(self.ptr.as_ptr(), table.as_ptr()) }
    }

    // ==================== Bill-specific Setters ====================

    /// Sets the bill account.
    pub fn set_bill_account(&self, account: &Account) {
        unsafe { ffi::gncEntrySetBillAccount(self.ptr.as_ptr(), account.as_ptr()) }
    }

    /// Sets the bill price.
    pub fn set_bill_price(&self, price: Numeric) {
        unsafe { ffi::gncEntrySetBillPrice(self.ptr.as_ptr(), price.into()) }
    }

    /// Sets whether the bill entry is taxable.
    pub fn set_bill_taxable(&self, taxable: bool) {
        unsafe { ffi::gncEntrySetBillTaxable(self.ptr.as_ptr(), taxable as i32) }
    }

    /// Sets whether tax is included in the bill price.
    pub fn set_bill_tax_included(&self, included: bool) {
        unsafe { ffi::gncEntrySetBillTaxIncluded(self.ptr.as_ptr(), included as i32) }
    }

    /// Sets the bill tax table.
    pub fn set_bill_tax_table(&self, table: &TaxTable) {
        unsafe { ffi::gncEntrySetBillTaxTable(self.ptr.as_ptr(), table.as_ptr()) }
    }

    /// Sets whether this entry is billable.
    pub fn set_billable(&self, billable: bool) {
        unsafe { ffi::gncEntrySetBillable(self.ptr.as_ptr(), billable as i32) }
    }

    /// Sets the bill-to owner.
    pub fn set_bill_to(&self, owner: &Owner) {
        unsafe { ffi::gncEntrySetBillTo(self.ptr.as_ptr(), owner.as_ptr()) }
    }

    /// Sets the bill payment type.
    pub fn set_bill_payment(&self, payment_type: ffi::GncEntryPaymentType) {
        unsafe { ffi::gncEntrySetBillPayment(self.ptr.as_ptr(), payment_type) }
    }
}

impl Drop for Entry {
    fn drop(&mut self) {
        if self.owned {
            unsafe { ffi::gncEntryDestroy(self.ptr.as_ptr()) }
        }
    }
}

impl std::fmt::Debug for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Entry")
            .field("guid", &self.guid())
            .field("description", &self.description())
            .field("quantity", &self.quantity())
            .finish()
    }
}
