//! Safe wrapper for GnuCash Account.

use std::ffi::{CStr, CString};
use std::ptr::NonNull;

use crate::ffi;
use crate::iter::{AccountChildren, AccountDescendants, AccountSplits};
use crate::{Book, Guid, Numeric};

/// Account type enumeration.
pub use crate::ffi::GNCAccountType;

/// A GnuCash Account - a ledger for tracking splits.
///
/// Accounts are organized in a tree hierarchy and hold splits
/// denominated in a specific commodity.
pub struct Account {
    ptr: NonNull<ffi::Account>,
    owned: bool,
}

unsafe impl Send for Account {}

impl Account {
    /// Creates a new Account in the given book.
    pub fn new(book: &Book) -> Self {
        let ptr = unsafe { ffi::xaccMallocAccount(book.as_ptr()) };
        Self {
            ptr: NonNull::new(ptr).expect("xaccMallocAccount returned null"),
            owned: true,
        }
    }

    /// Creates an Account wrapper from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be valid and point to a properly initialized Account.
    pub unsafe fn from_raw(ptr: *mut ffi::Account, owned: bool) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr, owned })
    }

    /// Returns the raw pointer to the underlying Account.
    pub fn as_ptr(&self) -> *mut ffi::Account {
        self.ptr.as_ptr()
    }

    /// Returns the GUID of this account.
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

    /// Begins an edit session on this account.
    /// Must be called before making changes.
    pub fn begin_edit(&self) {
        unsafe { ffi::xaccAccountBeginEdit(self.ptr.as_ptr()) }
    }

    /// Commits changes made during the edit session.
    pub fn commit_edit(&self) {
        unsafe { ffi::xaccAccountCommitEdit(self.ptr.as_ptr()) }
    }

    // ==================== Getters ====================

    /// Returns the account's name.
    pub fn name(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::xaccAccountGetName(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the account's code.
    pub fn code(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::xaccAccountGetCode(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the account's description.
    pub fn description(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::xaccAccountGetDescription(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the account's notes.
    pub fn notes(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::xaccAccountGetNotes(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the account's color.
    pub fn color(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::xaccAccountGetColor(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Returns the account type.
    pub fn account_type(&self) -> GNCAccountType {
        unsafe { ffi::xaccAccountGetType(self.ptr.as_ptr()) }
    }

    /// Returns the fully qualified name (e.g., "Assets:Bank:Checking").
    /// The caller owns the returned string.
    pub fn full_name(&self) -> Option<String> {
        unsafe {
            let ptr = ffi::gnc_account_get_full_name(self.ptr.as_ptr());
            if ptr.is_null() {
                None
            } else {
                let s = CStr::from_ptr(ptr).to_string_lossy().into_owned();
                ffi::g_free(ptr as *mut _);
                Some(s)
            }
        }
    }

    /// Returns true if this is a placeholder account.
    pub fn is_placeholder(&self) -> bool {
        unsafe { ffi::xaccAccountGetPlaceholder(self.ptr.as_ptr()) != 0 }
    }

    /// Returns true if this account is hidden.
    pub fn is_hidden(&self) -> bool {
        unsafe { ffi::xaccAccountGetHidden(self.ptr.as_ptr()) != 0 }
    }

    /// Returns true if this account should be hidden (checks parents too).
    pub fn should_be_hidden(&self) -> bool {
        unsafe { ffi::xaccAccountIsHidden(self.ptr.as_ptr()) != 0 }
    }

    /// Returns true if this is the root account.
    pub fn is_root(&self) -> bool {
        unsafe { ffi::gnc_account_is_root(self.ptr.as_ptr()) != 0 }
    }

    // ==================== Setters ====================

    /// Sets the account's name.
    ///
    /// # Panics
    ///
    /// Panics if `name` contains a null byte.
    pub fn set_name(&self, name: &str) {
        let c_name = CString::new(name).unwrap();
        unsafe { ffi::xaccAccountSetName(self.ptr.as_ptr(), c_name.as_ptr()) }
    }

    /// Sets the account's code.
    ///
    /// # Panics
    ///
    /// Panics if `code` contains a null byte.
    pub fn set_code(&self, code: &str) {
        let c_code = CString::new(code).unwrap();
        unsafe { ffi::xaccAccountSetCode(self.ptr.as_ptr(), c_code.as_ptr()) }
    }

    /// Sets the account's description.
    ///
    /// # Panics
    ///
    /// Panics if `desc` contains a null byte.
    pub fn set_description(&self, desc: &str) {
        let c_desc = CString::new(desc).unwrap();
        unsafe { ffi::xaccAccountSetDescription(self.ptr.as_ptr(), c_desc.as_ptr()) }
    }

    /// Sets the account's notes.
    ///
    /// # Panics
    ///
    /// Panics if `notes` contains a null byte.
    pub fn set_notes(&self, notes: &str) {
        let c_notes = CString::new(notes).unwrap();
        unsafe { ffi::xaccAccountSetNotes(self.ptr.as_ptr(), c_notes.as_ptr()) }
    }

    /// Sets the account's color.
    ///
    /// # Panics
    ///
    /// Panics if `color` contains a null byte.
    pub fn set_color(&self, color: &str) {
        let c_color = CString::new(color).unwrap();
        unsafe { ffi::xaccAccountSetColor(self.ptr.as_ptr(), c_color.as_ptr()) }
    }

    /// Sets the account type.
    pub fn set_type(&self, account_type: GNCAccountType) {
        unsafe { ffi::xaccAccountSetType(self.ptr.as_ptr(), account_type) }
    }

    /// Sets the placeholder flag.
    pub fn set_placeholder(&self, val: bool) {
        unsafe { ffi::xaccAccountSetPlaceholder(self.ptr.as_ptr(), val as i32) }
    }

    /// Sets the hidden flag.
    pub fn set_hidden(&self, val: bool) {
        unsafe { ffi::xaccAccountSetHidden(self.ptr.as_ptr(), val as i32) }
    }

    // ==================== Hierarchy ====================

    /// Returns the parent account, if any.
    pub fn parent(&self) -> Option<Account> {
        unsafe {
            let ptr = ffi::gnc_account_get_parent(self.ptr.as_ptr());
            Self::from_raw(ptr, false)
        }
    }

    /// Returns the root account of the tree this account belongs to.
    pub fn root(&self) -> Option<Account> {
        unsafe {
            let ptr = ffi::gnc_account_get_root(self.ptr.as_ptr());
            Self::from_raw(ptr, false)
        }
    }

    /// Returns the number of immediate children.
    pub fn n_children(&self) -> i32 {
        unsafe { ffi::gnc_account_n_children(self.ptr.as_ptr()) }
    }

    /// Returns the number of all descendants.
    pub fn n_descendants(&self) -> i32 {
        unsafe { ffi::gnc_account_n_descendants(self.ptr.as_ptr()) }
    }

    /// Returns the depth of this account in the tree.
    pub fn depth(&self) -> i32 {
        unsafe { ffi::gnc_account_get_current_depth(self.ptr.as_ptr()) }
    }

    /// Returns the n'th child account.
    pub fn nth_child(&self, n: i32) -> Option<Account> {
        unsafe {
            let ptr = ffi::gnc_account_nth_child(self.ptr.as_ptr(), n);
            Self::from_raw(ptr, false)
        }
    }

    /// Appends a child account to this account.
    /// Note: After appending, the parent/book takes ownership of the child.
    pub fn append_child(&self, child: &Account) {
        unsafe { ffi::gnc_account_append_child(self.ptr.as_ptr(), child.ptr.as_ptr()) }
    }

    /// Marks this account as not owned by this wrapper.
    /// Call this after adding the account to a book/hierarchy.
    pub fn mark_unowned(&mut self) {
        self.owned = false;
    }

    /// Removes a child account from this account.
    pub fn remove_child(&self, child: &Account) {
        unsafe { ffi::gnc_account_remove_child(self.ptr.as_ptr(), child.ptr.as_ptr()) }
    }

    /// Checks if the given account is an ancestor of this account.
    pub fn has_ancestor(&self, ancestor: &Account) -> bool {
        unsafe { ffi::xaccAccountHasAncestor(self.ptr.as_ptr(), ancestor.ptr.as_ptr()) != 0 }
    }

    /// Looks up an account by name among descendants.
    pub fn lookup_by_name(&self, name: &str) -> Option<Account> {
        let c_name = CString::new(name).ok()?;
        unsafe {
            let ptr = ffi::gnc_account_lookup_by_name(self.ptr.as_ptr(), c_name.as_ptr());
            Self::from_raw(ptr, false)
        }
    }

    /// Looks up an account by full name (e.g., "Assets:Bank:Checking").
    pub fn lookup_by_full_name(&self, name: &str) -> Option<Account> {
        let c_name = CString::new(name).ok()?;
        unsafe {
            let ptr = ffi::gnc_account_lookup_by_full_name(self.ptr.as_ptr(), c_name.as_ptr());
            Self::from_raw(ptr, false)
        }
    }

    /// Looks up an account by code among descendants.
    pub fn lookup_by_code(&self, code: &str) -> Option<Account> {
        let c_code = CString::new(code).ok()?;
        unsafe {
            let ptr = ffi::gnc_account_lookup_by_code(self.ptr.as_ptr(), c_code.as_ptr());
            Self::from_raw(ptr, false)
        }
    }

    // ==================== Balances ====================

    /// Returns the current balance of the account.
    pub fn balance(&self) -> Numeric {
        unsafe { ffi::xaccAccountGetBalance(self.ptr.as_ptr()).into() }
    }

    /// Returns the cleared balance (only cleared transactions).
    pub fn cleared_balance(&self) -> Numeric {
        unsafe { ffi::xaccAccountGetClearedBalance(self.ptr.as_ptr()).into() }
    }

    /// Returns the reconciled balance (only reconciled transactions).
    pub fn reconciled_balance(&self) -> Numeric {
        unsafe { ffi::xaccAccountGetReconciledBalance(self.ptr.as_ptr()).into() }
    }

    /// Returns the present balance (excluding future-dated transactions).
    pub fn present_balance(&self) -> Numeric {
        unsafe { ffi::xaccAccountGetPresentBalance(self.ptr.as_ptr()).into() }
    }

    /// Returns the projected minimum balance.
    pub fn projected_minimum_balance(&self) -> Numeric {
        unsafe { ffi::xaccAccountGetProjectedMinimumBalance(self.ptr.as_ptr()).into() }
    }

    /// Returns the balance as of a specific date.
    pub fn balance_as_of_date(&self, date: i64) -> Numeric {
        unsafe { ffi::xaccAccountGetBalanceAsOfDate(self.ptr.as_ptr(), date).into() }
    }

    /// Recomputes the account balance.
    pub fn recompute_balance(&self) {
        unsafe { ffi::xaccAccountRecomputeBalance(self.ptr.as_ptr()) }
    }

    // ==================== Splits ====================

    /// Returns the number of splits in this account.
    pub fn splits_size(&self) -> usize {
        unsafe { ffi::xaccAccountGetSplitsSize(self.ptr.as_ptr()) }
    }

    /// Returns the smallest commodity unit for this account.
    pub fn commodity_scu(&self) -> i32 {
        unsafe { ffi::xaccAccountGetCommoditySCU(self.ptr.as_ptr()) }
    }

    // ==================== Iterators ====================

    /// Returns an iterator over the immediate children of this account.
    pub fn children(&self) -> AccountChildren {
        AccountChildren::new(self)
    }

    /// Returns an iterator over all descendants of this account (depth-first).
    pub fn descendants(&self) -> AccountDescendants {
        AccountDescendants::new(self)
    }

    /// Returns an iterator over the splits in this account.
    pub fn splits(&self) -> AccountSplits {
        AccountSplits::new(self)
    }
}

impl Drop for Account {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ffi::xaccAccountBeginEdit(self.ptr.as_ptr());
                ffi::xaccAccountDestroy(self.ptr.as_ptr());
            }
        }
    }
}

impl std::fmt::Debug for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Account")
            .field("guid", &self.guid())
            .field("name", &self.name())
            .field("type", &self.account_type())
            .field("balance", &self.balance())
            .finish()
    }
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.guid() == other.guid()
    }
}

impl Eq for Account {}

impl std::hash::Hash for Account {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.guid().hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_type_enum() {
        assert_eq!(GNCAccountType::ACCT_TYPE_BANK as i32, 0);
        assert_eq!(GNCAccountType::ACCT_TYPE_CASH as i32, 1);
        assert_eq!(GNCAccountType::ACCT_TYPE_ASSET as i32, 2);
    }
}
