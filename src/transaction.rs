//! Safe wrapper for GnuCash Transaction.

use std::ffi::{CStr, CString};
use std::ptr::NonNull;

use crate::ffi;
use crate::iter::TransactionSplits;
use crate::{Book, Guid, Numeric};

/// Transaction type constants.
pub mod txn_type {
    pub const NONE: char = '\0';
    pub const INVOICE: char = 'I';
    pub const PAYMENT: char = 'P';
    pub const LINK: char = 'L';
}

/// A GnuCash Transaction - a double-entry accounting record.
///
/// A Transaction contains one or more Splits that must balance to zero.
/// Each transaction has a date, description, and currency.
pub struct Transaction {
    ptr: NonNull<ffi::Transaction>,
    owned: bool,
}

unsafe impl Send for Transaction {}

impl Transaction {
    /// Creates a new Transaction in the given book.
    pub fn new(book: &Book) -> Self {
        let ptr = unsafe { ffi::xaccMallocTransaction(book.as_ptr()) };
        Self {
            ptr: NonNull::new(ptr).expect("xaccMallocTransaction returned null"),
            owned: true,
        }
    }

    /// Creates a Transaction wrapper from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be valid and point to a properly initialized Transaction.
    pub unsafe fn from_raw(ptr: *mut ffi::Transaction, owned: bool) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr, owned })
    }

    /// Returns the raw pointer to the underlying Transaction.
    pub fn as_ptr(&self) -> *mut ffi::Transaction {
        self.ptr.as_ptr()
    }

    /// Returns the GUID of this transaction.
    pub fn guid(&self) -> Guid {
        unsafe {
            let instance = self.ptr.as_ptr() as *const ffi::QofInstance;
            let guid_ptr = ffi::qof_instance_get_guid(instance);
            if guid_ptr.is_null() {
                Guid::from_bytes([0; 16])
            } else {
                Guid::from_bytes((*guid_ptr).reserved)
            }
        }
    }

    // ==================== Edit Cycle ====================

    /// Begins an edit session on this transaction.
    /// Must be called before making changes.
    pub fn begin_edit(&self) {
        unsafe { ffi::xaccTransBeginEdit(self.ptr.as_ptr()) }
    }

    /// Commits changes made during the edit session.
    pub fn commit_edit(&self) {
        unsafe { ffi::xaccTransCommitEdit(self.ptr.as_ptr()) }
    }

    /// Rolls back changes made during the edit session.
    pub fn rollback_edit(&self) {
        unsafe { ffi::xaccTransRollbackEdit(self.ptr.as_ptr()) }
    }

    /// Returns true if the transaction is open for editing.
    pub fn is_open(&self) -> bool {
        unsafe { ffi::xaccTransIsOpen(self.ptr.as_ptr()) != 0 }
    }

    // ==================== Getters ====================

    /// Returns the transaction description.
    pub fn description(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::xaccTransGetDescription(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the transaction number (ID).
    pub fn num(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::xaccTransGetNum(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the transaction notes.
    pub fn notes(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::xaccTransGetNotes(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the document link URL.
    pub fn doc_link(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::xaccTransGetDocLink(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the transaction type (NONE, INVOICE, PAYMENT, LINK).
    pub fn txn_type(&self) -> char {
        unsafe { ffi::xaccTransGetTxnType(self.ptr.as_ptr()) as u8 as char }
    }

    /// Returns true if this is a closing transaction.
    pub fn is_closing(&self) -> bool {
        unsafe { ffi::xaccTransGetIsClosingTxn(self.ptr.as_ptr()) != 0 }
    }

    /// Returns true if this transaction is voided.
    pub fn is_void(&self) -> bool {
        unsafe { ffi::xaccTransGetVoidStatus(self.ptr.as_ptr()) != 0 }
    }

    /// Returns the reason this transaction was voided, if any.
    pub fn void_reason(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::xaccTransGetVoidReason(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the read-only reason, if set.
    pub fn read_only_reason(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::xaccTransGetReadOnly(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns true if this transaction is read-only due to posted date.
    pub fn is_readonly_by_posted_date(&self) -> bool {
        unsafe { ffi::xaccTransIsReadonlyByPostedDate(self.ptr.as_ptr()) != 0 }
    }

    // ==================== Setters ====================

    /// Sets the transaction description.
    pub fn set_description(&self, desc: &str) {
        let c_desc = CString::new(desc).unwrap();
        unsafe { ffi::xaccTransSetDescription(self.ptr.as_ptr(), c_desc.as_ptr()) }
    }

    /// Sets the transaction number (ID).
    pub fn set_num(&self, num: &str) {
        let c_num = CString::new(num).unwrap();
        unsafe { ffi::xaccTransSetNum(self.ptr.as_ptr(), c_num.as_ptr()) }
    }

    /// Sets the transaction notes.
    pub fn set_notes(&self, notes: &str) {
        let c_notes = CString::new(notes).unwrap();
        unsafe { ffi::xaccTransSetNotes(self.ptr.as_ptr(), c_notes.as_ptr()) }
    }

    /// Sets the document link URL.
    pub fn set_doc_link(&self, link: &str) {
        let c_link = CString::new(link).unwrap();
        unsafe { ffi::xaccTransSetDocLink(self.ptr.as_ptr(), c_link.as_ptr()) }
    }

    /// Sets the transaction type.
    pub fn set_txn_type(&self, txn_type: char) {
        unsafe { ffi::xaccTransSetTxnType(self.ptr.as_ptr(), txn_type as i8) }
    }

    /// Sets whether this is a closing transaction.
    pub fn set_is_closing(&self, is_closing: bool) {
        unsafe { ffi::xaccTransSetIsClosingTxn(self.ptr.as_ptr(), is_closing as i32) }
    }

    /// Sets the read-only flag with a reason.
    pub fn set_read_only(&self, reason: &str) {
        let c_reason = CString::new(reason).unwrap();
        unsafe { ffi::xaccTransSetReadOnly(self.ptr.as_ptr(), c_reason.as_ptr()) }
    }

    /// Clears the read-only flag.
    pub fn clear_read_only(&self) {
        unsafe { ffi::xaccTransClearReadOnly(self.ptr.as_ptr()) }
    }

    // ==================== Dates ====================

    /// Returns the posted date as a Unix timestamp.
    pub fn date_posted(&self) -> i64 {
        unsafe { ffi::xaccTransRetDatePosted(self.ptr.as_ptr()) }
    }

    /// Returns the entered date as a Unix timestamp.
    pub fn date_entered(&self) -> i64 {
        unsafe { ffi::xaccTransRetDateEntered(self.ptr.as_ptr()) }
    }

    /// Returns the due date as a Unix timestamp (for invoices).
    pub fn date_due(&self) -> i64 {
        unsafe { ffi::xaccTransRetDateDue(self.ptr.as_ptr()) }
    }

    /// Returns the void time as a Unix timestamp.
    pub fn void_time(&self) -> i64 {
        unsafe { ffi::xaccTransGetVoidTime(self.ptr.as_ptr()) }
    }

    /// Sets the posted date using day, month, year.
    pub fn set_date(&self, day: i32, month: i32, year: i32) {
        unsafe { ffi::xaccTransSetDate(self.ptr.as_ptr(), day, month, year) }
    }

    /// Sets the posted date as a Unix timestamp (normalized to date only).
    pub fn set_date_posted(&self, time: i64) {
        unsafe { ffi::xaccTransSetDatePostedSecsNormalized(self.ptr.as_ptr(), time) }
    }

    /// Sets the entered date as a Unix timestamp.
    pub fn set_date_entered(&self, time: i64) {
        unsafe { ffi::xaccTransSetDateEnteredSecs(self.ptr.as_ptr(), time) }
    }

    /// Sets the due date as a Unix timestamp.
    pub fn set_date_due(&self, time: i64) {
        unsafe { ffi::xaccTransSetDateDue(self.ptr.as_ptr(), time) }
    }

    // ==================== Splits ====================

    /// Returns the number of splits in this transaction.
    pub fn split_count(&self) -> i32 {
        unsafe { ffi::xaccTransCountSplits(self.ptr.as_ptr()) }
    }

    /// Returns the split at the given index.
    pub fn get_split(&self, index: i32) -> Option<*mut ffi::Split> {
        unsafe {
            let ptr = ffi::xaccTransGetSplit(self.ptr.as_ptr(), index);
            if ptr.is_null() {
                None
            } else {
                Some(ptr)
            }
        }
    }

    /// Returns the index of a split in this transaction.
    pub fn get_split_index(&self, split: *const ffi::Split) -> i32 {
        unsafe { ffi::xaccTransGetSplitIndex(self.ptr.as_ptr(), split) }
    }

    /// Sorts the splits in the transaction (debits first, then credits).
    pub fn sort_splits(&self) {
        unsafe { ffi::xaccTransSortSplits(self.ptr.as_ptr()) }
    }

    /// Removes all splits from the transaction.
    pub fn clear_splits(&self) {
        unsafe { ffi::xaccTransClearSplits(self.ptr.as_ptr()) }
    }

    // ==================== Balance ====================

    /// Returns the imbalance value of the transaction.
    /// Should be zero for a properly balanced transaction.
    pub fn imbalance_value(&self) -> Numeric {
        unsafe { ffi::xaccTransGetImbalanceValue(self.ptr.as_ptr()).into() }
    }

    /// Returns true if the transaction is balanced.
    pub fn is_balanced(&self) -> bool {
        unsafe { ffi::xaccTransIsBalanced(self.ptr.as_ptr()) != 0 }
    }

    /// Returns the total value applied to a specific account.
    pub fn account_value(&self, account: *const ffi::Account) -> Numeric {
        unsafe { ffi::xaccTransGetAccountValue(self.ptr.as_ptr(), account).into() }
    }

    /// Returns the total amount (in account commodity) for a specific account.
    pub fn account_amount(&self, account: *const ffi::Account) -> Numeric {
        unsafe { ffi::xaccTransGetAccountAmount(self.ptr.as_ptr(), account).into() }
    }

    // ==================== Voiding ====================

    /// Voids the transaction with a reason.
    pub fn void(&self, reason: &str) {
        let c_reason = CString::new(reason).unwrap();
        unsafe { ffi::xaccTransVoid(self.ptr.as_ptr(), c_reason.as_ptr()) }
    }

    /// Unvoids a voided transaction.
    pub fn unvoid(&self) {
        unsafe { ffi::xaccTransUnvoid(self.ptr.as_ptr()) }
    }

    /// Creates a reverse transaction that cancels this one.
    pub fn reverse(&self) -> Option<Transaction> {
        unsafe {
            let ptr = ffi::xaccTransReverse(self.ptr.as_ptr());
            Self::from_raw(ptr, true)
        }
    }

    /// Returns the transaction that reversed this one, if any.
    pub fn reversed_by(&self) -> Option<Transaction> {
        unsafe {
            let ptr = ffi::xaccTransGetReversedBy(self.ptr.as_ptr());
            Self::from_raw(ptr, false)
        }
    }

    // ==================== Misc ====================

    /// Returns true if this transaction uses trading accounts.
    pub fn use_trading_accounts(&self) -> bool {
        unsafe { ffi::xaccTransUseTradingAccounts(self.ptr.as_ptr()) != 0 }
    }

    /// Returns true if the transaction has reconciled splits.
    pub fn has_reconciled_splits(&self) -> bool {
        unsafe { ffi::xaccTransHasReconciledSplits(self.ptr.as_ptr()) != 0 }
    }

    // ==================== Iterators ====================

    /// Returns an iterator over the splits in this transaction.
    pub fn splits(&self) -> TransactionSplits {
        TransactionSplits::new(self)
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                // Only destroy if not already open
                if ffi::xaccTransIsOpen(self.ptr.as_ptr()) == 0 {
                    ffi::xaccTransBeginEdit(self.ptr.as_ptr());
                }
                ffi::xaccTransDestroy(self.ptr.as_ptr());
                ffi::xaccTransCommitEdit(self.ptr.as_ptr());
            }
        }
    }
}

impl std::fmt::Debug for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Transaction")
            .field("guid", &self.guid())
            .field("description", &self.description())
            .field("date_posted", &self.date_posted())
            .field("split_count", &self.split_count())
            .field("is_balanced", &self.is_balanced())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_txn_type_constants() {
        assert_eq!(txn_type::NONE, '\0');
        assert_eq!(txn_type::INVOICE, 'I');
        assert_eq!(txn_type::PAYMENT, 'P');
        assert_eq!(txn_type::LINK, 'L');
    }
}
