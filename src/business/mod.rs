//! Safe wrappers for GnuCash business entities.
//!
//! This module provides safe Rust wrappers for GnuCash's business features:
//! - [`Customer`] - A customer who receives invoices
//! - [`Vendor`] - A vendor who sends bills
//! - [`Employee`] - An employee who can submit expense vouchers
//! - [`Job`] - A job/project associated with a customer or vendor
//! - [`Invoice`] - An invoice, bill, or expense voucher
//! - [`Entry`] - A line item in an invoice
//! - [`Address`] - A mailing address
//! - [`Owner`] - A union type representing customer, vendor, employee, or job
//! - [`BillTerm`] - Payment terms for invoices
//! - [`TaxTable`] - Tax rates and rules

mod address;
mod billterm;
mod customer;
mod employee;
mod entry;
mod invoice;
mod job;
mod owner;
mod taxtable;
mod vendor;

pub use address::Address;
pub use billterm::BillTerm;
pub use customer::Customer;
pub use employee::Employee;
pub use entry::Entry;
pub use invoice::Invoice;
pub use job::Job;
pub use owner::{Owner, OwnerType};
pub use taxtable::{TaxTable, TaxTableEntry};
pub use vendor::Vendor;

// Re-export enums
pub use crate::ffi::{
    GncAmountType, GncBillTermType, GncDiscountHow, GncEntryPaymentType, GncInvoiceType,
    GncTaxIncluded,
};
