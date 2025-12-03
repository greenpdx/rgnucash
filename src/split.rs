//! Safe wrapper for GnuCash Split.

use std::ffi::{CStr, CString};
use std::ptr::NonNull;

use crate::ffi;
use crate::{Account, Book, Guid, Numeric, Transaction};

/// Reconcile state constants.
pub mod reconcile {
    /// Cleared
    pub const CLEARED: char = 'c';
    /// Reconciled
    pub const RECONCILED: char = 'y';
    /// Frozen into accounting period
    pub const FROZEN: char = 'f';
    /// Not reconciled
    pub const NOT_RECONCILED: char = 'n';
    /// Voided
    pub const VOIDED: char = 'v';
}

/// A GnuCash Split - a single entry in a transaction.
///
/// A Split represents one side of a double-entry transaction, linking
/// an amount to an account. Each Split has both an 'amount' (in the
/// account's commodity) and a 'value' (in the transaction's currency).
pub struct Split {
    ptr: NonNull<ffi::Split>,
    owned: bool,
}

unsafe impl Send for Split {}

impl Split {
    /// Creates a new Split in the given book.
    pub fn new(book: &Book) -> Self {
        let ptr = unsafe { ffi::xaccMallocSplit(book.as_ptr()) };
        Self {
            ptr: NonNull::new(ptr).expect("xaccMallocSplit returned null"),
            owned: true,
        }
    }

    /// Creates a Split wrapper from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be valid and point to a properly initialized Split.
    pub unsafe fn from_raw(ptr: *mut ffi::Split, owned: bool) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr, owned })
    }

    /// Returns the raw pointer to the underlying Split.
    pub fn as_ptr(&self) -> *mut ffi::Split {
        self.ptr.as_ptr()
    }

    /// Returns the GUID of this split.
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

    /// Reinitializes the split to default values.
    pub fn reinit(&self) {
        unsafe { ffi::xaccSplitReinit(self.ptr.as_ptr()) }
    }

    // ==================== Account/Transaction Linkage ====================

    /// Returns the account this split belongs to.
    pub fn account(&self) -> Option<Account> {
        unsafe {
            let ptr = ffi::xaccSplitGetAccount(self.ptr.as_ptr());
            Account::from_raw(ptr, false)
        }
    }

    /// Sets the account for this split.
    pub fn set_account(&self, account: &Account) {
        unsafe { ffi::xaccSplitSetAccount(self.ptr.as_ptr(), account.as_ptr()) }
    }

    /// Returns the parent transaction of this split.
    pub fn transaction(&self) -> Option<Transaction> {
        unsafe {
            let ptr = ffi::xaccSplitGetParent(self.ptr.as_ptr());
            Transaction::from_raw(ptr, false)
        }
    }

    /// Sets the parent transaction for this split.
    pub fn set_transaction(&self, trans: &Transaction) {
        unsafe { ffi::xaccSplitSetParent(self.ptr.as_ptr(), trans.as_ptr()) }
    }

    /// Returns the book this split belongs to.
    pub fn book(&self) -> Option<Book> {
        unsafe {
            let ptr = ffi::xaccSplitGetBook(self.ptr.as_ptr());
            Book::from_raw(ptr, false)
        }
    }

    // ==================== Memo/Action ====================

    /// Returns the split memo.
    pub fn memo(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::xaccSplitGetMemo(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Sets the split memo.
    pub fn set_memo(&self, memo: &str) {
        let c_memo = CString::new(memo).unwrap();
        unsafe { ffi::xaccSplitSetMemo(self.ptr.as_ptr(), c_memo.as_ptr()) }
    }

    /// Returns the split action (e.g., "Buy", "Sell", "Deposit").
    pub fn action(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::xaccSplitGetAction(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Sets the split action.
    pub fn set_action(&self, action: &str) {
        let c_action = CString::new(action).unwrap();
        unsafe { ffi::xaccSplitSetAction(self.ptr.as_ptr(), c_action.as_ptr()) }
    }

    /// Returns the split type ("normal" or "stock-split").
    pub fn split_type(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::xaccSplitGetType(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Marks this split as a stock split.
    pub fn make_stock_split(&self) {
        unsafe { ffi::xaccSplitMakeStockSplit(self.ptr.as_ptr()) }
    }

    // ==================== Amount/Value ====================

    /// Returns the amount in the account's commodity.
    pub fn amount(&self) -> Numeric {
        unsafe { ffi::xaccSplitGetAmount(self.ptr.as_ptr()).into() }
    }

    /// Sets the amount in the account's commodity.
    pub fn set_amount(&self, amount: Numeric) {
        unsafe { ffi::xaccSplitSetAmount(self.ptr.as_ptr(), amount.into()) }
    }

    /// Returns the value in the transaction's currency.
    pub fn value(&self) -> Numeric {
        unsafe { ffi::xaccSplitGetValue(self.ptr.as_ptr()).into() }
    }

    /// Sets the value in the transaction's currency.
    pub fn set_value(&self, value: Numeric) {
        unsafe { ffi::xaccSplitSetValue(self.ptr.as_ptr(), value.into()) }
    }

    /// Returns the share price (value / amount).
    pub fn share_price(&self) -> Numeric {
        unsafe { ffi::xaccSplitGetSharePrice(self.ptr.as_ptr()).into() }
    }

    /// Sets both share price and amount simultaneously.
    pub fn set_share_price_and_amount(&self, price: Numeric, amount: Numeric) {
        unsafe {
            ffi::xaccSplitSetSharePriceAndAmount(self.ptr.as_ptr(), price.into(), amount.into())
        }
    }

    // ==================== Balances ====================

    /// Returns the running balance up to and including this split.
    pub fn balance(&self) -> Numeric {
        unsafe { ffi::xaccSplitGetBalance(self.ptr.as_ptr()).into() }
    }

    /// Returns the running balance excluding closing transactions.
    pub fn noclosing_balance(&self) -> Numeric {
        unsafe { ffi::xaccSplitGetNoclosingBalance(self.ptr.as_ptr()).into() }
    }

    /// Returns the cleared balance (cleared + reconciled splits).
    pub fn cleared_balance(&self) -> Numeric {
        unsafe { ffi::xaccSplitGetClearedBalance(self.ptr.as_ptr()).into() }
    }

    /// Returns the reconciled balance (only reconciled splits).
    pub fn reconciled_balance(&self) -> Numeric {
        unsafe { ffi::xaccSplitGetReconciledBalance(self.ptr.as_ptr()).into() }
    }

    // ==================== Reconciliation ====================

    /// Returns the reconcile state ('n', 'c', 'y', 'f', 'v').
    pub fn reconcile_state(&self) -> char {
        unsafe { ffi::xaccSplitGetReconcile(self.ptr.as_ptr()) as u8 as char }
    }

    /// Sets the reconcile state.
    pub fn set_reconcile_state(&self, state: char) {
        unsafe { ffi::xaccSplitSetReconcile(self.ptr.as_ptr(), state as i8) }
    }

    /// Returns the date when this split was reconciled.
    pub fn date_reconciled(&self) -> i64 {
        unsafe { ffi::xaccSplitGetDateReconciled(self.ptr.as_ptr()) }
    }

    /// Sets the date when this split was reconciled.
    pub fn set_date_reconciled(&self, time: i64) {
        unsafe { ffi::xaccSplitSetDateReconciledSecs(self.ptr.as_ptr(), time) }
    }

    /// Returns true if this split is reconciled.
    pub fn is_reconciled(&self) -> bool {
        self.reconcile_state() == reconcile::RECONCILED
    }

    /// Returns true if this split is cleared.
    pub fn is_cleared(&self) -> bool {
        self.reconcile_state() == reconcile::CLEARED
    }

    // ==================== Other Split ====================

    /// Returns the other split in a two-split transaction.
    /// Returns None if the transaction has more than two splits.
    pub fn other_split(&self) -> Option<Split> {
        unsafe {
            let ptr = ffi::xaccSplitGetOtherSplit(self.ptr.as_ptr());
            Self::from_raw(ptr, false)
        }
    }

    /// Returns the full name of the corresponding account.
    pub fn corr_account_full_name(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::xaccSplitGetCorrAccountFullName(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                let s = CStr::from_ptr(ptr).to_string_lossy().into_owned();
                ffi::g_free(ptr as *mut _);
                Some(s)
            }
        }
    }

    /// Returns the name of the corresponding account.
    pub fn corr_account_name(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::xaccSplitGetCorrAccountName(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the code of the corresponding account.
    pub fn corr_account_code(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::xaccSplitGetCorrAccountCode(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    // ==================== Voiding ====================

    /// Returns the original amount before voiding.
    pub fn void_former_amount(&self) -> Numeric {
        unsafe { ffi::xaccSplitVoidFormerAmount(self.ptr.as_ptr()).into() }
    }

    /// Returns the original value before voiding.
    pub fn void_former_value(&self) -> Numeric {
        unsafe { ffi::xaccSplitVoidFormerValue(self.ptr.as_ptr()).into() }
    }

    // ==================== Peer Splits ====================

    /// Returns true if this split has peer splits.
    pub fn has_peers(&self) -> bool {
        unsafe { ffi::xaccSplitHasPeers(self.ptr.as_ptr()) != 0 }
    }

    /// Returns true if the other split is a peer of this one.
    pub fn is_peer(&self, other: &Split) -> bool {
        unsafe { ffi::xaccSplitIsPeerSplit(self.ptr.as_ptr(), other.ptr.as_ptr()) != 0 }
    }

    /// Adds a peer split to this split's lot-split list.
    pub fn add_peer(&self, other: &Split, timestamp: i64) {
        unsafe { ffi::xaccSplitAddPeerSplit(self.ptr.as_ptr(), other.ptr.as_ptr(), timestamp) }
    }

    /// Removes a peer split from this split's lot-split list.
    pub fn remove_peer(&self, other: &Split) {
        unsafe { ffi::xaccSplitRemovePeerSplit(self.ptr.as_ptr(), other.ptr.as_ptr()) }
    }
}

impl Drop for Split {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ffi::xaccSplitDestroy(self.ptr.as_ptr());
            }
        }
    }
}

impl std::fmt::Debug for Split {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Split")
            .field("guid", &self.guid())
            .field("memo", &self.memo())
            .field("amount", &self.amount())
            .field("value", &self.value())
            .field("reconcile", &self.reconcile_state())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reconcile_constants() {
        assert_eq!(reconcile::CLEARED, 'c');
        assert_eq!(reconcile::RECONCILED, 'y');
        assert_eq!(reconcile::FROZEN, 'f');
        assert_eq!(reconcile::NOT_RECONCILED, 'n');
        assert_eq!(reconcile::VOIDED, 'v');
    }
}
