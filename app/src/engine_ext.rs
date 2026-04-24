//! Extension traits that add engine-level methods to `gnucash_sys`
//! types without moving the types themselves across crates.
//!
//! - [`BookExt`]: `Book::commodity_table()` returning the
//!   [`CommodityTable`](crate::business::CommodityTable) attached to
//!   the book.
//! - [`AccountExt`]: `Account::commodity()` /
//!   `Account::set_commodity()` — every transaction on an account
//!   requires the account's commodity to be set.
//!
//! Usage:
//!
//! ```ignore
//! use gnucash_ext::{BookExt, AccountExt, init_engine};
//! use gnucash_ext::business::{Commodity, CommodityTable};
//!
//! let book = gnucash_ext::Book::new();
//! let table = book.commodity_table().unwrap();
//! let crc = table.insert(
//!     &Commodity::new(&book, "Costa Rican Colón", "CURRENCY", "CRC", None, 100).unwrap(),
//! ).unwrap();
//!
//! let account = gnucash_ext::Account::new(&book);
//! account.begin_edit();
//! account.set_commodity(&crc);
//! account.commit_edit();
//! ```

use gnucash_sys::{ffi, Account, Book};

use crate::business::{Commodity, CommodityTable};

pub trait BookExt {
    /// Return the commodity table associated with this book. Books
    /// always have one (libgnucash creates it lazily on first access),
    /// so `None` here indicates a serious problem — e.g. a book whose
    /// backend failed to initialize.
    fn commodity_table(&self) -> Option<CommodityTable>;
}

impl BookExt for Book {
    fn commodity_table(&self) -> Option<CommodityTable> {
        unsafe {
            let ptr = ffi::gnc_commodity_table_get_table(self.as_ptr());
            CommodityTable::from_raw(ptr)
        }
    }
}

pub trait AccountExt {
    /// Returns the commodity this account is denominated in.
    /// `None` for an account that has not had a commodity set yet.
    fn commodity(&self) -> Option<Commodity>;

    /// Sets the commodity. Must be called between `begin_edit` and
    /// `commit_edit` like all other account mutators.
    fn set_commodity(&self, commodity: &Commodity);
}

impl AccountExt for Account {
    fn commodity(&self) -> Option<Commodity> {
        unsafe {
            let ptr = ffi::xaccAccountGetCommodity(self.as_ptr());
            Commodity::from_raw(ptr, false)
        }
    }

    fn set_commodity(&self, commodity: &Commodity) {
        unsafe {
            ffi::xaccAccountSetCommodity(self.as_ptr(), commodity.as_ptr());
        }
    }
}
