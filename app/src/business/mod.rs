//! Business module - entities for invoicing and customer management.
//!
//! This module provides safe wrappers for GnuCash business entities:
//! - [`Address`] - Mailing address
//! - [`Customer`] - Customer entity
//! - [`Vendor`] - Vendor/supplier entity
//! - [`Employee`] - Employee entity
//! - [`Job`] - Job linked to a customer
//! - [`Invoice`] - Invoice/bill document
//! - [`Entry`] - Line item in an invoice
//! - [`BillTerm`] - Payment terms
//! - [`TaxTable`] - Tax rate table
//! - [`Owner`] - Polymorphic owner (Customer, Vendor, Employee, or Job)
//! - [`Commodity`] - Currency or security denomination

pub mod address;
pub mod billterm;
pub mod commodity;
pub mod customer;
pub mod employee;
pub mod entry;
pub mod invoice;
pub mod job;
pub mod owner;
pub mod tax;
pub mod vendor;

pub use address::Address;
pub use billterm::BillTerm;
pub use commodity::{Commodity, CommodityTable};
pub use customer::Customer;
pub use employee::Employee;
pub use entry::Entry;
pub use invoice::Invoice;
pub use job::Job;
pub use owner::{Owner, OwnerType, TypedOwner};
pub use tax::{TaxTable, TaxTableEntry};
pub use vendor::Vendor;
