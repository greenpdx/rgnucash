//! Safe wrapper for GnuCash Invoice.

use std::ffi::{CStr, CString};
use std::ptr::NonNull;

use crate::ffi;
use crate::{Account, Book, Guid, Numeric, Transaction};

use super::{BillTerm, Entry, Owner};

/// A GnuCash Invoice - an invoice, bill, or expense voucher.
pub struct Invoice {
    ptr: NonNull<ffi::GncInvoice>,
    owned: bool,
}

unsafe impl Send for Invoice {}

impl Invoice {
    /// Creates a new Invoice in the given book.
    pub fn new(book: &Book) -> Self {
        let ptr = unsafe { ffi::gncInvoiceCreate(book.as_ptr()) };
        Self {
            ptr: NonNull::new(ptr).expect("gncInvoiceCreate returned null"),
            owned: true,
        }
    }

    /// Creates a copy of another invoice.
    pub fn copy(other: &Invoice) -> Self {
        let ptr = unsafe { ffi::gncInvoiceCopy(other.as_ptr()) };
        Self {
            ptr: NonNull::new(ptr).expect("gncInvoiceCopy returned null"),
            owned: true,
        }
    }

    /// Creates an Invoice wrapper from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be valid and point to a properly initialized GncInvoice.
    pub unsafe fn from_raw(ptr: *mut ffi::GncInvoice, owned: bool) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr, owned })
    }

    /// Returns the raw pointer to the underlying GncInvoice.
    pub fn as_ptr(&self) -> *mut ffi::GncInvoice {
        self.ptr.as_ptr()
    }

    /// Returns the GUID of this invoice.
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

    /// Begins an edit session on this invoice.
    pub fn begin_edit(&self) {
        unsafe { ffi::gncInvoiceBeginEdit(self.ptr.as_ptr()) }
    }

    /// Commits changes made during the edit session.
    pub fn commit_edit(&self) {
        unsafe { ffi::gncInvoiceCommitEdit(self.ptr.as_ptr()) }
    }

    // ==================== Getters ====================

    /// Returns the invoice ID.
    pub fn id(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncInvoiceGetID(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the owner of this invoice.
    pub fn owner(&self) -> Option<Owner> {
        unsafe {
            let ptr = ffi::gncInvoiceGetOwner(self.ptr.as_ptr());
            // Returns const, need to cast
            Owner::from_raw(ptr as *mut _, false)
        }
    }

    /// Returns the date the invoice was opened.
    pub fn date_opened(&self) -> i64 {
        unsafe { ffi::gncInvoiceGetDateOpened(self.ptr.as_ptr()) }
    }

    /// Returns the date the invoice was posted.
    pub fn date_posted(&self) -> i64 {
        unsafe { ffi::gncInvoiceGetDatePosted(self.ptr.as_ptr()) }
    }

    /// Returns the due date.
    pub fn date_due(&self) -> i64 {
        unsafe { ffi::gncInvoiceGetDateDue(self.ptr.as_ptr()) }
    }

    /// Returns the billing ID.
    pub fn billing_id(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncInvoiceGetBillingID(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the invoice notes.
    pub fn notes(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncInvoiceGetNotes(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the document link.
    pub fn doc_link(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncInvoiceGetDocLink(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the payment terms.
    pub fn terms(&self) -> Option<BillTerm> {
        unsafe {
            let ptr = ffi::gncInvoiceGetTerms(self.ptr.as_ptr());
            BillTerm::from_raw(ptr, false)
        }
    }

    /// Returns the invoice type.
    pub fn invoice_type(&self) -> ffi::GncInvoiceType {
        unsafe { ffi::gncInvoiceGetType(self.ptr.as_ptr()) }
    }

    /// Returns the invoice type as a string.
    pub fn type_string(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gncInvoiceGetTypeString(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the owner type.
    pub fn owner_type(&self) -> ffi::GncOwnerType {
        unsafe { ffi::gncInvoiceGetOwnerType(self.ptr.as_ptr()) }
    }

    /// Returns true if the invoice is active.
    pub fn is_active(&self) -> bool {
        unsafe { ffi::gncInvoiceGetActive(self.ptr.as_ptr()) != 0 }
    }

    /// Returns true if this is a credit note.
    pub fn is_credit_note(&self) -> bool {
        unsafe { ffi::gncInvoiceGetIsCreditNote(self.ptr.as_ptr()) != 0 }
    }

    /// Returns the "bill to" owner.
    pub fn bill_to(&self) -> Option<Owner> {
        unsafe {
            let ptr = ffi::gncInvoiceGetBillTo(self.ptr.as_ptr());
            Owner::from_raw(ptr, false)
        }
    }

    /// Returns the amount to charge.
    pub fn to_charge_amount(&self) -> Numeric {
        unsafe { ffi::gncInvoiceGetToChargeAmount(self.ptr.as_ptr()).into() }
    }

    /// Returns the posted transaction.
    pub fn posted_txn(&self) -> Option<Transaction> {
        unsafe {
            let ptr = ffi::gncInvoiceGetPostedTxn(self.ptr.as_ptr());
            Transaction::from_raw(ptr, false)
        }
    }

    /// Returns the posted account.
    pub fn posted_account(&self) -> Option<Account> {
        unsafe {
            let ptr = ffi::gncInvoiceGetPostedAcc(self.ptr.as_ptr());
            Account::from_raw(ptr, false)
        }
    }

    /// Returns the total amount.
    pub fn total(&self) -> Numeric {
        unsafe { ffi::gncInvoiceGetTotal(self.ptr.as_ptr()).into() }
    }

    /// Returns the subtotal (before tax).
    pub fn total_subtotal(&self) -> Numeric {
        unsafe { ffi::gncInvoiceGetTotalSubtotal(self.ptr.as_ptr()).into() }
    }

    /// Returns the total tax amount.
    pub fn total_tax(&self) -> Numeric {
        unsafe { ffi::gncInvoiceGetTotalTax(self.ptr.as_ptr()).into() }
    }

    /// Returns true if the invoice is posted.
    pub fn is_posted(&self) -> bool {
        unsafe { ffi::gncInvoiceIsPosted(self.ptr.as_ptr()) != 0 }
    }

    /// Returns true if the invoice is paid.
    pub fn is_paid(&self) -> bool {
        unsafe { ffi::gncInvoiceIsPaid(self.ptr.as_ptr()) != 0 }
    }

    // ==================== Setters ====================

    /// Sets the invoice ID.
    pub fn set_id(&self, id: &str) {
        let c_id = CString::new(id).unwrap();
        unsafe { ffi::gncInvoiceSetID(self.ptr.as_ptr(), c_id.as_ptr()) }
    }

    /// Sets the owner of this invoice.
    pub fn set_owner(&self, owner: &Owner) {
        unsafe { ffi::gncInvoiceSetOwner(self.ptr.as_ptr(), owner.as_ptr()) }
    }

    /// Sets the date opened.
    pub fn set_date_opened(&self, date: i64) {
        unsafe { ffi::gncInvoiceSetDateOpened(self.ptr.as_ptr(), date) }
    }

    /// Sets the date posted.
    pub fn set_date_posted(&self, date: i64) {
        unsafe { ffi::gncInvoiceSetDatePosted(self.ptr.as_ptr(), date) }
    }

    /// Sets the billing ID.
    pub fn set_billing_id(&self, id: &str) {
        let c_id = CString::new(id).unwrap();
        unsafe { ffi::gncInvoiceSetBillingID(self.ptr.as_ptr(), c_id.as_ptr()) }
    }

    /// Sets the invoice notes.
    pub fn set_notes(&self, notes: &str) {
        let c_notes = CString::new(notes).unwrap();
        unsafe { ffi::gncInvoiceSetNotes(self.ptr.as_ptr(), c_notes.as_ptr()) }
    }

    /// Sets the document link.
    pub fn set_doc_link(&self, link: &str) {
        let c_link = CString::new(link).unwrap();
        unsafe { ffi::gncInvoiceSetDocLink(self.ptr.as_ptr(), c_link.as_ptr()) }
    }

    /// Sets the payment terms.
    pub fn set_terms(&self, terms: &BillTerm) {
        unsafe { ffi::gncInvoiceSetTerms(self.ptr.as_ptr(), terms.as_ptr()) }
    }

    /// Sets whether the invoice is active.
    pub fn set_active(&self, active: bool) {
        unsafe { ffi::gncInvoiceSetActive(self.ptr.as_ptr(), active as i32) }
    }

    /// Sets whether this is a credit note.
    pub fn set_is_credit_note(&self, credit_note: bool) {
        unsafe { ffi::gncInvoiceSetIsCreditNote(self.ptr.as_ptr(), credit_note as i32) }
    }

    /// Sets the "bill to" owner.
    pub fn set_bill_to(&self, bill_to: &Owner) {
        unsafe { ffi::gncInvoiceSetBillTo(self.ptr.as_ptr(), bill_to.as_ptr()) }
    }

    /// Sets the amount to charge.
    pub fn set_to_charge_amount(&self, amount: Numeric) {
        unsafe { ffi::gncInvoiceSetToChargeAmount(self.ptr.as_ptr(), amount.into()) }
    }

    // ==================== Entries ====================

    /// Adds an entry to this invoice.
    pub fn add_entry(&self, entry: &Entry) {
        unsafe { ffi::gncInvoiceAddEntry(self.ptr.as_ptr(), entry.as_ptr()) }
    }

    /// Removes an entry from this invoice.
    pub fn remove_entry(&self, entry: &Entry) {
        unsafe { ffi::gncInvoiceRemoveEntry(self.ptr.as_ptr(), entry.as_ptr()) }
    }

    /// Sorts the entries in this invoice.
    pub fn sort_entries(&self) {
        unsafe { ffi::gncInvoiceSortEntries(self.ptr.as_ptr()) }
    }

    /// Removes all entries from this invoice.
    pub fn remove_entries(&self) {
        unsafe { ffi::gncInvoiceRemoveEntries(self.ptr.as_ptr()) }
    }

    // ==================== Posting ====================

    /// Posts the invoice to an account.
    pub fn post_to_account(
        &self,
        acc: &Account,
        posted_date: i64,
        due_date: i64,
        memo: &str,
        accumulate_splits: bool,
        autopay: bool,
    ) -> Option<Transaction> {
        let c_memo = CString::new(memo).unwrap();
        unsafe {
            let txn = ffi::gncInvoicePostToAccount(
                self.ptr.as_ptr(),
                acc.as_ptr(),
                posted_date,
                due_date,
                c_memo.as_ptr(),
                accumulate_splits as i32,
                autopay as i32,
            );
            Transaction::from_raw(txn, false)
        }
    }

    /// Unpost this invoice.
    /// Returns true if successful.
    pub fn unpost(&self, reset_tax_tables: bool) -> bool {
        unsafe { ffi::gncInvoiceUnpost(self.ptr.as_ptr(), reset_tax_tables as i32) != 0 }
    }
}

impl Drop for Invoice {
    fn drop(&mut self) {
        if self.owned {
            unsafe { ffi::gncInvoiceDestroy(self.ptr.as_ptr()) }
        }
    }
}

impl std::fmt::Debug for Invoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Invoice")
            .field("guid", &self.guid())
            .field("id", &self.id())
            .field("type", &self.invoice_type())
            .field("total", &self.total())
            .field("is_posted", &self.is_posted())
            .field("is_paid", &self.is_paid())
            .finish()
    }
}
