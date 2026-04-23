//! Safe wrapper for QofQuery - the GnuCash query framework.

use std::ffi::CString;
use std::ptr::NonNull;

use gnucash_sys::ffi;
use gnucash_sys::{Account, Book, Guid, Split, Transaction};

use crate::business::Invoice;

/// Re-export query enums.
pub use gnucash_sys::ffi::QofQueryOp;

/// A query for searching GnuCash objects.
pub struct Query {
    ptr: NonNull<ffi::QofQuery>,
}

unsafe impl Send for Query {}

impl Query {
    /// Creates a new query.
    pub fn new() -> Self {
        let ptr = unsafe { ffi::qof_query_create() };
        Self {
            ptr: NonNull::new(ptr).expect("qof_query_create returned null"),
        }
    }

    /// Creates a new query for a specific object type.
    pub fn for_type(obj_type: &str) -> Self {
        let query = Self::new();
        query.set_search_for(obj_type);
        query
    }

    /// Creates a Query wrapper from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be valid.
    pub unsafe fn from_raw(ptr: *mut ffi::QofQuery) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr })
    }

    /// Returns the raw pointer.
    pub fn as_ptr(&self) -> *mut ffi::QofQuery {
        self.ptr.as_ptr()
    }

    /// Sets the object type to search for.
    pub fn set_search_for(&self, obj_type: &str) {
        let c_type = CString::new(obj_type).unwrap();
        unsafe { ffi::qof_query_search_for(self.ptr.as_ptr(), c_type.as_ptr()) }
    }

    /// Sets the book to search in.
    pub fn set_book(&self, book: &Book) {
        unsafe { ffi::qof_query_set_book(self.ptr.as_ptr(), book.as_ptr()) }
    }

    /// Clears the query, removing all terms.
    pub fn clear(&self) {
        unsafe { ffi::qof_query_clear(self.ptr.as_ptr()) }
    }

    /// Purges all query terms.
    pub fn purge_terms(&self) {
        unsafe { ffi::qof_query_purge_terms(self.ptr.as_ptr(), std::ptr::null_mut()) }
    }

    /// Sets the maximum number of results.
    pub fn set_max_results(&self, max: i32) {
        unsafe { ffi::qof_query_set_max_results(self.ptr.as_ptr(), max) }
    }

    /// Runs the query and returns results as a raw GList pointer.
    fn run_raw(&self) -> *mut ffi::GList {
        unsafe { ffi::qof_query_run(self.ptr.as_ptr()) }
    }

    /// Runs the query and returns results as splits.
    pub fn run_splits(&self) -> Vec<Split> {
        let list = self.run_raw();
        let mut results = Vec::new();
        let mut current = list;
        while !current.is_null() {
            unsafe {
                let data = (*current).data as *mut ffi::Split;
                if let Some(split) = Split::from_raw(data, false) {
                    results.push(split);
                }
                current = (*current).next;
            }
        }
        results
    }

    /// Runs the query and returns results as transactions.
    pub fn run_transactions(&self) -> Vec<Transaction> {
        let list = self.run_raw();
        let mut results = Vec::new();
        let mut current = list;
        while !current.is_null() {
            unsafe {
                let data = (*current).data as *mut ffi::Transaction;
                if let Some(txn) = Transaction::from_raw(data, false) {
                    results.push(txn);
                }
                current = (*current).next;
            }
        }
        results
    }

    /// Runs the query and returns results as accounts.
    pub fn run_accounts(&self) -> Vec<Account> {
        let list = self.run_raw();
        let mut results = Vec::new();
        let mut current = list;
        while !current.is_null() {
            unsafe {
                let data = (*current).data as *mut ffi::Account;
                if let Some(acct) = Account::from_raw(data, false) {
                    results.push(acct);
                }
                current = (*current).next;
            }
        }
        results
    }

    /// Runs the query and returns results as invoices. Pair with
    /// `set_search_for("gncInvoice")` (or `Query::for_type("gncInvoice")`)
    /// and whatever predicate terms you need.
    pub fn run_invoices(&self) -> Vec<Invoice> {
        let list = self.run_raw();
        let mut results = Vec::new();
        let mut current = list;
        while !current.is_null() {
            unsafe {
                let data = (*current).data as *mut ffi::GncInvoice;
                if let Some(inv) = Invoice::from_raw(data, false) {
                    results.push(inv);
                }
                current = (*current).next;
            }
        }
        results
    }

    // ==================== Predicate methods ====================

    /// Adds a GUID match predicate.
    pub fn add_guid_match(&self, param_list: &[&str], guid: &Guid, op: QofQueryOp) {
        let c_params = make_gsl(param_list);
        unsafe {
            ffi::qof_query_add_guid_match(
                self.ptr.as_ptr(),
                c_params,
                guid.as_ffi() as *const ffi::GncGUID,
                op,
            );
        }
        // Note: qof_query takes ownership of the param_list
    }

    /// Adds a boolean match predicate.
    pub fn add_boolean_match(&self, param_list: &[&str], value: bool, op: QofQueryOp) {
        let c_params = make_gsl(param_list);
        unsafe {
            ffi::qof_query_add_boolean_match(self.ptr.as_ptr(), c_params, value as i32, op);
        }
        // Note: qof_query takes ownership of the param_list
    }

    /// Merges another query into this one.
    pub fn merge(&self, other: &Query, op: QofQueryOp) {
        unsafe { ffi::qof_query_merge_in_place(self.ptr.as_ptr(), other.ptr.as_ptr(), op) }
    }

    /// Inverts the query.
    pub fn invert(&self) -> Query {
        unsafe {
            let ptr = ffi::qof_query_invert(self.ptr.as_ptr());
            Query::from_raw(ptr).expect("qof_query_invert returned null")
        }
    }

    /// Checks if the query has any terms.
    pub fn has_terms(&self) -> bool {
        unsafe { ffi::qof_query_has_terms(self.ptr.as_ptr()) != 0 }
    }

    /// Returns the number of terms in the query.
    pub fn num_terms(&self) -> i32 {
        unsafe { ffi::qof_query_num_terms(self.ptr.as_ptr()) }
    }
}

impl Default for Query {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Query {
    fn clone(&self) -> Self {
        unsafe {
            let ptr = ffi::qof_query_copy(self.ptr.as_ptr());
            Query::from_raw(ptr).expect("qof_query_copy returned null")
        }
    }
}

impl Drop for Query {
    fn drop(&mut self) {
        unsafe { ffi::qof_query_destroy(self.ptr.as_ptr()) }
    }
}

impl std::fmt::Debug for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Query")
            .field("has_terms", &self.has_terms())
            .field("num_terms", &self.num_terms())
            .finish()
    }
}

// Helper to create a GSList from a slice of strings
// Note: qof_query takes ownership of this list, so we don't free it
fn make_gsl(strings: &[&str]) -> *mut ffi::GSList {
    let mut list: *mut ffi::GSList = std::ptr::null_mut();
    for s in strings.iter().rev() {
        let c_str = CString::new(*s).unwrap();
        unsafe {
            list = ffi::g_slist_prepend(list, c_str.into_raw() as *mut _);
        }
    }
    list
}

/// Common object type constants for queries.
pub mod obj_types {
    pub const SPLIT: &str = "Split";
    pub const TRANSACTION: &str = "Trans";
    pub const ACCOUNT: &str = "Account";
}

/// Common parameter path constants.
pub mod params {
    pub const SPLIT_TRANS: &str = "trans";
    pub const SPLIT_ACCOUNT: &str = "account";
    pub const SPLIT_VALUE: &str = "value";
    pub const SPLIT_AMOUNT: &str = "amount";
    pub const SPLIT_MEMO: &str = "memo";
    pub const SPLIT_RECONCILE: &str = "reconcile-flag";

    pub const TRANS_DATE_POSTED: &str = "date-posted";
    pub const TRANS_DATE_ENTERED: &str = "date-entered";
    pub const TRANS_DESCRIPTION: &str = "desc";
    pub const TRANS_NUM: &str = "num";

    pub const ACCOUNT_NAME: &str = "name";
    pub const ACCOUNT_CODE: &str = "code";
    pub const ACCOUNT_TYPE: &str = "account-type";

    pub const QOF_PARAM_GUID: &str = "guid";
}
