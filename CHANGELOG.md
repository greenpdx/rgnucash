# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2024-01-01

### Added

- Initial release of gnucash-sys FFI bindings
- Safe wrappers for core GnuCash types:
  - `Book` - Top-level container for financial data
  - `Account` - Hierarchical ledger accounts
  - `Transaction` - Double-entry accounting records
  - `Split` - Individual entries in transactions
  - `Guid` - 128-bit unique identifiers
  - `Numeric` - Rational numbers (numerator/denominator)
  - `Session` - Connection to GnuCash data files
  - `Price` and `PriceDB` - Price quote management
- Iterator support for account hierarchies and transaction splits
- Serde support behind optional `serde` feature flag
- Pre-generated bindings for docs.rs compatibility
- Comprehensive examples:
  - `simple_book` - Basic book creation
  - `simple_session` - Opening GnuCash files
  - `list_accounts` - Account hierarchy traversal
  - `account_analysis` - Balance calculations
  - `create_transaction` - Creating transactions
  - `opening_balances` - Setting up opening balances
  - `price_database` - Working with prices
  - `export_csv` - Exporting data to CSV
  - `account_tree` - Displaying account trees
  - `reconcile_account` - Account reconciliation
  - `search_transactions` - Finding transactions
  - `balance_sheet` - Generating balance sheets

### Security

- All FFI calls wrapped in safe Rust abstractions
- Memory management handled via RAII (Drop trait)
- Thread safety documented (types are `Send` but require external synchronization)

[Unreleased]: https://github.com/user/gnucash-sys/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/user/gnucash-sys/releases/tag/v0.1.0
