# gnucash-sys

Rust FFI bindings and safe wrappers for the GnuCash accounting engine.

## Overview

This workspace provides Rust access to GnuCash's core accounting functionality:

- **gnucash-sys** - FFI bindings and safe wrappers for core types (Book, Account, Transaction, Split)
- **gnucash-ext** - Extended functionality (business entities, queries, builders)

## Features

- Safe Rust wrappers around GnuCash C API
- Full access to accounts, transactions, splits, and balances
- Session management for opening/creating GnuCash files
- Business entities (Customer, Vendor, Invoice, etc.)
- Query framework for searching
- Builder patterns for complex entity creation
- Compatible with docs.rs (pre-generated bindings)

## Requirements

### System Dependencies

Install GnuCash development libraries:

### Raspberry Pi OS
```bash
sudo apt install libgnucash-dev libglib2.0-dev
```

### Debian / Ubuntu (no libgnucash-dev available)
Build GnuCash from source to get the headers and libraries:
```bash

sudo apt-get install gnucash
sudo apt-get build-dep gnucash
git clone https://github.com/Gnucash/gnucash.git
cd gnucash && mkdir build && cd build
cmake -DCMAKE_INSTALL_PREFIX=/usr/local ..
make -j$(nproc) && sudo make install
```
Then set environment variables for the build:
```bash
export GNUCASH_LIB=/usr/local/lib
export GNUCASH_INCLUDE=/usr/local/include/gnucash
```

**Debian/Ubuntu:**
```bash
```

**Fedora:**
```bash
sudo dnf install gnucash gnucash-devel glib2-devel
```

**Arch Linux:**
```bash
sudo pacman -S gnucash glib2
```

**macOS (Homebrew):**
```bash
brew install gnucash glib pkg-config
```

### Rust

Rust 2024 edition (1.85+) is required.

## Building

### Using System Libraries (Recommended)

The build script automatically detects GnuCash using pkg-config:

```bash
# Clone the repository
git clone https://github.com/user/gnucash-sys.git
cd gnucash-sys

# Build
cargo build

# Run tests
cargo test

# Build examples
cargo build --examples
```

### Custom Library Paths

If GnuCash is installed in a non-standard location, set environment variables:

```bash
# Using .env file
cat > .env << EOF
GNUCASH_SRC=/path/to/gnucash/source
GNUCASH_BUILD=/path/to/gnucash/build
EOF

# Or export directly
export GNUCASH_LIB=/usr/lib/gnucash
export GNUCASH_INCLUDE=/usr/include/gnucash
```

### Build Configuration

The build script searches for libraries in this order:

1. Environment variables (`GNUCASH_LIB`, `GNUCASH_INCLUDE`)
2. pkg-config (`gnucash`, `gnucash-engine`)
3. Common system paths (`/usr/lib/gnucash`, `/usr/include/gnucash`)

## Quick Start

```rust
use gnucash_sys::{init_engine, Book, Account, GNCAccountType, Numeric};

fn main() {
    // Initialize the engine (required before any operations)
    init_engine();

    // Create a new book
    let book = Book::new();
    let root = book.root_account().expect("Book should have root");

    // Create an account
    let checking = Account::new(&book);
    checking.begin_edit();
    checking.set_name("Checking");
    checking.set_type(GNCAccountType::ACCT_TYPE_BANK);
    checking.commit_edit();
    root.append_child(&checking);

    // Check balance
    println!("Balance: ${:.2}", checking.balance().to_f64());
}
```

### Opening Existing Files

```rust
use gnucash_sys::{init_engine, Session, SessionOpenMode};

fn main() {
    init_engine();

    match Session::open("/path/to/file.gnucash", SessionOpenMode::SESSION_READ_ONLY) {
        Ok(session) => {
            if let Some(book) = session.book() {
                println!("Transactions: {}", book.transaction_count());

                if let Some(root) = book.root_account() {
                    for account in root.children() {
                        println!("Account: {:?}", account.name());
                    }
                }
            }
            session.end();
        }
        Err(e) => eprintln!("Failed to open: {:?}", e),
    }
}
```

## Workspace Structure

```
gnucash-sys/
├── src/                    # Core FFI bindings and wrappers
│   ├── lib.rs             # Main library entry
│   ├── book.rs            # Book wrapper
│   ├── account.rs         # Account wrapper
│   ├── transaction.rs     # Transaction wrapper
│   ├── split.rs           # Split wrapper
│   ├── session.rs         # Session management
│   ├── price.rs           # Price database
│   ├── types.rs           # Guid, Numeric types
│   └── ...
├── examples/              # Example programs
├── docs/
│   └── API.md            # Core API documentation
├── app/                   # gnucash-ext crate
│   ├── src/
│   │   ├── lib.rs        # Extended library
│   │   ├── business/     # Business entities
│   │   ├── query.rs      # Query framework
│   │   ├── builder.rs    # Builder patterns
│   │   └── ...
│   ├── examples/         # Business examples
│   ├── docs/
│   │   └── API.md       # Extended API documentation
│   └── README.md
├── Cargo.toml            # Workspace configuration
├── build.rs              # Build script for bindgen
└── README.md             # This file
```

## Documentation

### API Reference

- [gnucash-sys API](docs/API.md) - Core types (Book, Account, Transaction, Split, Session)
- [gnucash-ext API](app/docs/API.md) - Extended types (Customer, Vendor, Invoice, Query, Builder)

### Examples

- [Core Examples](examples/README.md) - Examples using gnucash-sys
- [Business Examples](app/examples/) - Examples using gnucash-ext

### Running Examples

```bash
# List available examples
cargo run --example

# Core examples
cargo run --example simple_book
cargo run --example simple_session
cargo run --example list_accounts /path/to/file.gnucash
cargo run --example account_tree /path/to/file.gnucash
cargo run --example create_transaction
cargo run --example export_csv /path/to/file.gnucash "Assets:Checking"
cargo run --example balance_sheet /path/to/file.gnucash

# Business examples (gnucash-ext)
cargo run -p gnucash-ext --example simple_business
```

## Core Types

| Type | Description |
|------|-------------|
| `Session` | Connection to GnuCash file/database |
| `Book` | Top-level container for financial data |
| `Account` | Ledger in hierarchical tree |
| `Transaction` | Double-entry accounting record |
| `Split` | Single entry linking amount to account |
| `Guid` | 128-bit unique identifier |
| `Numeric` | Rational number for precise arithmetic |
| `Price` | Price quote for commodity |
| `PriceDB` | Database of prices |

## Extended Types (gnucash-ext)

| Type | Description |
|------|-------------|
| `Customer` | Customer for invoicing |
| `Vendor` | Vendor/supplier |
| `Employee` | Employee for expense vouchers |
| `Job` | Project linked to customer |
| `Invoice` | Invoice, bill, or voucher |
| `Entry` | Line item in invoice |
| `Owner` | Polymorphic owner type |
| `Query` | QofQuery for searching |
| `TransactionBuilder` | Fluent transaction creation |
| `InvoiceBuilder` | Fluent invoice creation |

## Safety

The safe wrappers handle memory management via RAII (Drop trait). Key patterns:

- **Edit cycle**: Call `begin_edit()` before modifications, `commit_edit()` after
- **Ownership**: Use `mark_unowned()` after adding entities to hierarchies
- **Sessions**: Always call `session.end()` when done

Note: The underlying GnuCash library is not thread-safe. While wrapper types implement `Send`, concurrent access requires external synchronization.

## License

MIT

## Contributing

Contributions welcome! Please ensure:

1. Code builds without warnings
2. Examples compile and run
3. Documentation is updated for API changes

## Related Projects

- [GnuCash](https://gnucash.org/) - The GnuCash accounting software
- [GnuCash Source](https://github.com/Gnucash/gnucash) - GnuCash source code
