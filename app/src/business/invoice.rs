//! Safe wrapper for GncInvoice.

use std::ffi::{CStr, CString};
use std::ptr::NonNull;

use gnucash_sys::ffi;
use gnucash_sys::{Account, Book, Guid, Numeric, Transaction};

use super::{Commodity, Entry, Owner};

pub use ffi::GncInvoiceType as InvoiceType;

/// An invoice or bill.
pub struct Invoice {
    ptr: NonNull<ffi::GncInvoice>,
    #[allow(dead_code)]
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

    /// Creates an Invoice wrapper from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be valid.
    pub unsafe fn from_raw(ptr: *mut ffi::GncInvoice, owned: bool) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr, owned })
    }

    /// Returns the raw pointer.
    pub fn as_ptr(&self) -> *mut ffi::GncInvoice {
        self.ptr.as_ptr()
    }

    /// Begins an edit session.
    pub fn begin_edit(&self) {
        unsafe { ffi::gncInvoiceBeginEdit(self.ptr.as_ptr()) }
    }

    /// Commits changes.
    pub fn commit_edit(&self) {
        unsafe { ffi::gncInvoiceCommitEdit(self.ptr.as_ptr()) }
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

    /// Returns the owner of this invoice.
    pub fn owner(&self) -> Owner {
        unsafe {
            let owner_ptr = ffi::gncInvoiceGetOwner(self.ptr.as_ptr());
            if owner_ptr.is_null() {
                Owner::new()
            } else {
                Owner::from_raw(*owner_ptr)
            }
        }
    }

    /// Returns the date opened.
    pub fn date_opened(&self) -> i64 {
        unsafe { ffi::gncInvoiceGetDateOpened(self.ptr.as_ptr()) }
    }

    /// Returns the date posted.
    pub fn date_posted(&self) -> i64 {
        unsafe { ffi::gncInvoiceGetDatePosted(self.ptr.as_ptr()) }
    }

    /// Returns the date due.
    pub fn date_due(&self) -> i64 {
        unsafe { ffi::gncInvoiceGetDateDue(self.ptr.as_ptr()) }
    }

    /// Returns the total amount.
    pub fn total(&self) -> Numeric {
        unsafe { ffi::gncInvoiceGetTotal(self.ptr.as_ptr()).into() }
    }

    /// Returns the subtotal (before tax).
    pub fn total_subtotal(&self) -> Numeric {
        unsafe { ffi::gncInvoiceGetTotalSubtotal(self.ptr.as_ptr()).into() }
    }

    /// Returns the total tax.
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

    /// Returns the invoice type.
    pub fn invoice_type(&self) -> InvoiceType {
        unsafe { ffi::gncInvoiceGetType(self.ptr.as_ptr()) }
    }

    /// Returns the posted transaction.
    pub fn posted_txn(&self) -> Option<Transaction> {
        unsafe {
            let ptr = ffi::gncInvoiceGetPostedTxn(self.ptr.as_ptr());
            Transaction::from_raw(ptr, false)
        }
    }

    /// Returns the posted account.
    pub fn posted_acc(&self) -> Option<Account> {
        unsafe {
            let ptr = ffi::gncInvoiceGetPostedAcc(self.ptr.as_ptr());
            Account::from_raw(ptr, false)
        }
    }

    // ==================== Setters ====================

    /// Sets the invoice ID.
    pub fn set_id(&self, id: &str) {
        let c_id = CString::new(id).unwrap();
        unsafe { ffi::gncInvoiceSetID(self.ptr.as_ptr(), c_id.as_ptr()) }
    }

    /// Sets the invoice notes.
    pub fn set_notes(&self, notes: &str) {
        let c_notes = CString::new(notes).unwrap();
        unsafe { ffi::gncInvoiceSetNotes(self.ptr.as_ptr(), c_notes.as_ptr()) }
    }

    /// Sets the billing ID.
    pub fn set_billing_id(&self, billing_id: &str) {
        let c_billing_id = CString::new(billing_id).unwrap();
        unsafe { ffi::gncInvoiceSetBillingID(self.ptr.as_ptr(), c_billing_id.as_ptr()) }
    }

    /// Sets the owner of this invoice.
    pub fn set_owner(&self, owner: &Owner) {
        unsafe { ffi::gncInvoiceSetOwner(self.ptr.as_ptr(), owner.as_ptr()) }
    }

    /// Sets the date opened.
    pub fn set_date_opened(&self, date: i64) {
        unsafe { ffi::gncInvoiceSetDateOpened(self.ptr.as_ptr(), date) }
    }

    // ==================== Entry Management ====================

    /// Adds an entry to this invoice.
    pub fn add_entry(&self, entry: &super::Entry) {
        unsafe { ffi::gncInvoiceAddEntry(self.ptr.as_ptr(), entry.as_ptr()) }
    }

    /// Removes an entry from this invoice.
    pub fn remove_entry(&self, entry: &super::Entry) {
        unsafe { ffi::gncInvoiceRemoveEntry(self.ptr.as_ptr(), entry.as_ptr()) }
    }

    /// Returns all entries attached to this invoice, in the order
    /// GnuCash stores them (the same order the GUI and PDF rendering
    /// use).
    pub fn entries(&self) -> Vec<Entry> {
        let mut result = Vec::new();
        unsafe {
            let mut current = ffi::gncInvoiceGetEntries(self.ptr.as_ptr()) as *mut ffi::GList;
            while !current.is_null() {
                let entry_ptr = (*current).data as *mut ffi::GncEntry;
                if let Some(entry) = Entry::from_raw(entry_ptr, false) {
                    result.push(entry);
                }
                current = (*current).next;
            }
        }
        result
    }

    /// Returns the book this invoice belongs to, or `None` if the
    /// invoice has been detached from any book.
    pub fn book(&self) -> Option<Book> {
        unsafe {
            let instance = self.ptr.as_ptr() as ffi::gconstpointer;
            let book_ptr = ffi::qof_instance_get_book(instance);
            Book::from_raw(book_ptr, false)
        }
    }

    /// Returns the currency (commodity) in which this invoice is
    /// denominated. `None` only if the invoice has not been given a
    /// currency yet.
    pub fn currency(&self) -> Option<Commodity> {
        unsafe {
            let ptr = ffi::gncInvoiceGetCurrency(self.ptr.as_ptr());
            Commodity::from_raw(ptr, false)
        }
    }

    /// Post the invoice to an A/R (or A/P, for bills) account,
    /// creating the posted transaction and flipping [`is_posted`] to
    /// `true`.
    ///
    /// * `post_date` and `due_date` are `time64` seconds-since-epoch
    ///   values — pair with `gnc_time()` (for "now") or a chrono
    ///   timestamp's `timestamp()`.
    /// * `memo` lands on the posted transaction's description.
    /// * `accumulate_splits`: collapse multiple entries that share an
    ///   income/expense account into a single split per account.
    /// * `auto_pay`: if credits are available on the customer/vendor,
    ///   apply them to this invoice automatically.
    ///
    /// Returns the generated `Transaction` on success. `None` indicates
    /// libgnucash refused to post — typically because the A/R account
    /// has no commodity set, or because the invoice has no entries,
    /// or because posting is already in progress.
    ///
    /// [`is_posted`]: Self::is_posted
    pub fn post_to_account(
        &self,
        account: &Account,
        post_date: i64,
        due_date: i64,
        memo: &str,
        accumulate_splits: bool,
        auto_pay: bool,
    ) -> Option<Transaction> {
        let c_memo = match CString::new(memo) {
            Ok(c) => c,
            Err(_) => return None,
        };
        unsafe {
            let txn = ffi::gncInvoicePostToAccount(
                self.ptr.as_ptr(),
                account.as_ptr(),
                post_date,
                due_date,
                c_memo.as_ptr(),
                if accumulate_splits { 1 } else { 0 },
                if auto_pay { 1 } else { 0 },
            );
            Transaction::from_raw(txn, false)
        }
    }

}

impl std::fmt::Debug for Invoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Invoice")
            .field("id", &self.id())
            .field("total", &self.total())
            .field("is_posted", &self.is_posted())
            .field("is_paid", &self.is_paid())
            .finish()
    }
}
