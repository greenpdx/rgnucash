//! # gnucash-ext
//!
//! Extended functionality for gnucash-sys.
//!
//! This crate provides additional features on top of the core gnucash-sys library:
//!
//! - [`business`] - Business entities (Customer, Vendor, Employee, Invoice, etc.)
//! - [`price`] - Price database and price entries
//! - [`query`] - QOF query interface
//! - [`builder`] - Builder patterns for entity creation

// Re-export gnucash-sys for convenience
pub use gnucash_sys;

/// Safe wrapper for GncPrice and GncPriceDB.
pub mod price;

/// Safe wrapper for QofQuery.
pub mod query;

/// Builder patterns for entity creation.
pub mod builder;

/// Business entities (Customer, Vendor, Employee, Invoice, etc.).
pub mod business;

/// Extension traits adding commodity-handling methods to
/// `gnucash_sys::Book` / `gnucash_sys::Account`.
pub mod engine_ext;

pub use engine_ext::{AccountExt, BookExt};

// Re-export commonly used types from gnucash-sys
pub use gnucash_sys::{
    init_engine, is_engine_initialized, Account, Book, Error, GNCAccountType, Guid, Numeric,
    Result, Session, SessionOpenMode, Split, Transaction,
};

// Re-export price types
pub use price::{Price, PriceDB, PriceSource};

// Re-export query types
pub use query::{obj_types, params, QofQueryOp, Query};

// Re-export builders
pub use builder::{InvoiceBuilder, TransactionBuilder};

// Re-export business entities
pub use business::{
    Address, BillTerm, Commodity, CommodityTable, Customer, Employee, Entry, Invoice, Job, Owner,
    OwnerType, TaxTable, TaxTableEntry, TypedOwner, Vendor,
};
