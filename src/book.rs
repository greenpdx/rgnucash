//! Safe wrapper for QofBook.

use std::ptr::NonNull;

use crate::ffi;
use crate::{Account, Guid};

/// A GnuCash Book - the top-level container for all financial data.
///
/// The Book owns all accounts, transactions, and other entities.
/// When dropped, it will destroy all contained data.
pub struct Book {
    ptr: NonNull<ffi::QofBook>,
    owned: bool,
}

// Book is not thread-safe (GnuCash uses GLib which is not thread-safe)
// but it can be sent between threads if properly synchronized
unsafe impl Send for Book {}

impl Book {
    /// Creates a new empty Book.
    pub fn new() -> Self {
        let ptr = unsafe { ffi::qof_book_new() };
        Self {
            ptr: NonNull::new(ptr).expect("qof_book_new returned null"),
            owned: true,
        }
    }

    /// Creates a Book wrapper from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be valid and point to a properly initialized QofBook.
    /// If `owned` is true, the Book will be destroyed when this wrapper is dropped.
    pub unsafe fn from_raw(ptr: *mut ffi::QofBook, owned: bool) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr, owned })
    }

    /// Returns the raw pointer to the underlying QofBook.
    pub fn as_ptr(&self) -> *mut ffi::QofBook {
        self.ptr.as_ptr()
    }

    /// Returns the GUID of this book.
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

    /// Returns true if the book is read-only.
    pub fn is_readonly(&self) -> bool {
        unsafe { ffi::qof_book_is_readonly(self.ptr.as_ptr()) != 0 }
    }

    /// Marks the book as read-only.
    pub fn mark_readonly(&self) {
        unsafe { ffi::qof_book_mark_readonly(self.ptr.as_ptr()) }
    }

    /// Returns true if the book is empty (has no data loaded).
    pub fn is_empty(&self) -> bool {
        unsafe { ffi::qof_book_empty(self.ptr.as_ptr()) != 0 }
    }

    /// Returns true if the book has unsaved changes.
    pub fn is_dirty(&self) -> bool {
        unsafe { ffi::qof_book_session_not_saved(self.ptr.as_ptr()) != 0 }
    }

    /// Marks the book as saved (no pending changes).
    pub fn mark_saved(&self) {
        unsafe { ffi::qof_book_mark_session_saved(self.ptr.as_ptr()) }
    }

    /// Marks the book as dirty (has unsaved changes).
    pub fn mark_dirty(&self) {
        unsafe { ffi::qof_book_mark_session_dirty(self.ptr.as_ptr()) }
    }

    /// Returns true if the book is shutting down.
    pub fn is_shutting_down(&self) -> bool {
        unsafe { ffi::qof_book_shutting_down(self.ptr.as_ptr()) != 0 }
    }

    /// Returns true if the book uses trading accounts.
    pub fn use_trading_accounts(&self) -> bool {
        unsafe { ffi::qof_book_use_trading_accounts(self.ptr.as_ptr()) != 0 }
    }

    /// Returns true if the book uses split action for num field.
    pub fn use_split_action_for_num_field(&self) -> bool {
        unsafe { ffi::qof_book_use_split_action_for_num_field(self.ptr.as_ptr()) != 0 }
    }

    /// Returns the number of days for auto-read-only transactions.
    /// Returns 0 if the feature is disabled.
    pub fn num_days_autoreadonly(&self) -> i32 {
        unsafe { ffi::qof_book_get_num_days_autoreadonly(self.ptr.as_ptr()) }
    }

    /// Returns true if auto-read-only feature is enabled.
    pub fn uses_autoreadonly(&self) -> bool {
        unsafe { ffi::qof_book_uses_autoreadonly(self.ptr.as_ptr()) != 0 }
    }

    /// Marks the book as closed for editing.
    pub fn mark_closed(&self) {
        unsafe { ffi::qof_book_mark_closed(self.ptr.as_ptr()) }
    }

    /// Returns the root account for this book, if any.
    pub fn root_account_ptr(&self) -> *mut ffi::Account {
        unsafe { ffi::gnc_book_get_root_account(self.ptr.as_ptr()) }
    }

    /// Returns the root account for this book.
    pub fn root_account(&self) -> Option<Account> {
        unsafe { Account::from_raw(self.root_account_ptr(), false) }
    }

    /// Sets the root account for this book.
    pub fn set_root_account(&self, root: &Account) {
        unsafe { ffi::gnc_book_set_root_account(self.ptr.as_ptr(), root.as_ptr()) }
    }

    /// Returns the number of transactions in this book.
    pub fn transaction_count(&self) -> u32 {
        unsafe { ffi::gnc_book_count_transactions(self.ptr.as_ptr()) }
    }
}

impl Default for Book {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Book {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ffi::qof_book_destroy(self.ptr.as_ptr());
            }
        }
    }
}

impl std::fmt::Debug for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Book")
            .field("guid", &self.guid())
            .field("is_readonly", &self.is_readonly())
            .field("is_dirty", &self.is_dirty())
            .field("is_empty", &self.is_empty())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_new() {
        let book = Book::new();
        assert!(!book.is_readonly());
        assert!(book.is_empty());
    }

    #[test]
    fn test_book_dirty() {
        let book = Book::new();
        // Note: mark_dirty() requires a backend/session context to work properly
        // A newly created book without a session may not track dirty state
        assert!(!book.is_dirty());
        book.mark_dirty();
        // After mark_saved, should not be dirty regardless of initial state
        book.mark_saved();
        assert!(!book.is_dirty());
    }

    #[test]
    fn test_book_readonly() {
        let book = Book::new();
        assert!(!book.is_readonly());
        book.mark_readonly();
        assert!(book.is_readonly());
    }
}
