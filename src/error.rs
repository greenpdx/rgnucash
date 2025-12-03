//! Error types for gnucash-sys.
//!
//! This module provides error handling for GnuCash operations.
//!
//! # Examples
//!
//! ```
//! use gnucash_sys::{Error, Result};
//!
//! fn process_data() -> Result<()> {
//!     // Operations that might fail return Result<T, Error>
//!     Err(Error::InvalidOperation("example error".to_string()))
//! }
//!
//! match process_data() {
//!     Ok(()) => println!("Success"),
//!     Err(e) => println!("Error: {}", e),
//! }
//! ```

use std::fmt;

/// Error type for GnuCash operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// A null pointer was encountered where a valid pointer was expected.
    NullPointer(&'static str),
    /// An invalid GUID string was provided.
    InvalidGuid(String),
    /// An operation failed on a read-only object.
    ReadOnly,
    /// The transaction is not balanced.
    Unbalanced,
    /// An invalid account type was specified.
    InvalidAccountType(i32),
    /// A string conversion error occurred.
    StringConversion(String),
    /// A numeric error occurred (e.g., division by zero).
    Numeric(String),
    /// An invalid operation was attempted.
    InvalidOperation(String),
    /// A session error occurred.
    Session(String),
    /// Generic error with a message.
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NullPointer(ctx) => write!(f, "null pointer in {}", ctx),
            Error::InvalidGuid(s) => write!(f, "invalid GUID: {}", s),
            Error::ReadOnly => write!(f, "operation failed: object is read-only"),
            Error::Unbalanced => write!(f, "transaction is not balanced"),
            Error::InvalidAccountType(t) => write!(f, "invalid account type: {}", t),
            Error::StringConversion(s) => write!(f, "string conversion error: {}", s),
            Error::Numeric(s) => write!(f, "numeric error: {}", s),
            Error::InvalidOperation(s) => write!(f, "invalid operation: {}", s),
            Error::Session(s) => write!(f, "session error: {}", s),
            Error::Other(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for Error {}

/// Result type for GnuCash operations.
pub type Result<T> = std::result::Result<T, Error>;
