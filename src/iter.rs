//! Iterators for GnuCash collections.

use crate::ffi;
use crate::{Account, Split, Transaction};

/// Iterator over the children of an Account.
pub struct AccountChildren {
    parent: *mut ffi::Account,
    index: i32,
    count: i32,
}

impl AccountChildren {
    /// Creates a new iterator over the children of the given account.
    pub fn new(account: &Account) -> Self {
        let count = unsafe { ffi::gnc_account_n_children(account.as_ptr()) };
        Self {
            parent: account.as_ptr(),
            index: 0,
            count,
        }
    }
}

impl Iterator for AccountChildren {
    type Item = Account;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.count {
            return None;
        }
        let ptr = unsafe { ffi::gnc_account_nth_child(self.parent, self.index) };
        self.index += 1;
        unsafe { Account::from_raw(ptr, false) }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.count - self.index) as usize;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for AccountChildren {}

/// Iterator over all descendants of an Account (depth-first).
pub struct AccountDescendants {
    stack: Vec<AccountChildren>,
}

impl AccountDescendants {
    /// Creates a new iterator over all descendants of the given account.
    pub fn new(account: &Account) -> Self {
        Self {
            stack: vec![AccountChildren::new(account)],
        }
    }
}

impl Iterator for AccountDescendants {
    type Item = Account;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(iter) = self.stack.last_mut() {
            if let Some(account) = iter.next() {
                // Push children iterator for depth-first traversal
                self.stack.push(AccountChildren::new(&account));
                return Some(account);
            } else {
                self.stack.pop();
            }
        }
        None
    }
}

/// Iterator over the splits in a Transaction.
pub struct TransactionSplits {
    trans: *mut ffi::Transaction,
    index: i32,
    count: i32,
}

impl TransactionSplits {
    /// Creates a new iterator over the splits of the given transaction.
    pub fn new(trans: &Transaction) -> Self {
        let count = unsafe { ffi::xaccTransCountSplits(trans.as_ptr()) };
        Self {
            trans: trans.as_ptr(),
            index: 0,
            count,
        }
    }
}

impl Iterator for TransactionSplits {
    type Item = Split;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.count {
            return None;
        }
        let ptr = unsafe { ffi::xaccTransGetSplit(self.trans, self.index) };
        self.index += 1;
        unsafe { Split::from_raw(ptr, false) }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.count - self.index) as usize;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for TransactionSplits {}

/// Iterator over the splits in an Account.
///
/// Note: This iterator walks the GList returned by xaccAccountGetSplitList.
pub struct AccountSplits {
    current: *mut ffi::GList,
}

impl AccountSplits {
    /// Creates a new iterator over the splits of the given account.
    pub fn new(account: &Account) -> Self {
        let list = unsafe { ffi::xaccAccountGetSplitList(account.as_ptr()) };
        Self { current: list }
    }
}

impl Iterator for AccountSplits {
    type Item = Split;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            return None;
        }
        unsafe {
            let data = (*self.current).data;
            self.current = (*self.current).next;
            Split::from_raw(data as *mut ffi::Split, false)
        }
    }
}
