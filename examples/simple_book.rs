//! Simple example demonstrating how to create a new GnuCash book.
//!
//! This example creates a new book with a basic account structure:
//! - Root
//!   - Assets
//!     - Current Assets
//!       - Checking Account
//!   - Liabilities
//!   - Income
//!   - Expenses
//!   - Equity
//!     - Opening Balances
//!
//! Based on: gnucash/bindings/python/example_scripts/simple_book.py

use gnucash_sys::{init_engine, Account, Book, GNCAccountType};

fn main() {
    // Initialize the GnuCash engine
    init_engine();

    // Create a new book
    let book = Book::new();
    println!("Created new book: {:?}", book.guid());

    // Get the root account
    let root = book.root_account().expect("Book should have root account");
    println!("Root account: {:?}", root.name());

    // Create the main account categories
    let accounts = [
        ("Assets", GNCAccountType::ACCT_TYPE_ASSET),
        ("Liabilities", GNCAccountType::ACCT_TYPE_LIABILITY),
        ("Income", GNCAccountType::ACCT_TYPE_INCOME),
        ("Expenses", GNCAccountType::ACCT_TYPE_EXPENSE),
        ("Equity", GNCAccountType::ACCT_TYPE_EQUITY),
    ];

    for (name, account_type) in accounts {
        let mut account = Account::new(&book);
        account.begin_edit();
        account.set_name(name);
        account.set_type(account_type);
        account.commit_edit();
        root.append_child(&account);
        account.mark_unowned(); // Book now owns it
        println!("Created account: {}", name);
    }

    // Find the Assets account and add sub-accounts
    if let Some(assets) = root.lookup_by_name("Assets") {
        // Create Current Assets
        let mut current_assets = Account::new(&book);
        current_assets.begin_edit();
        current_assets.set_name("Current Assets");
        current_assets.set_type(GNCAccountType::ACCT_TYPE_ASSET);
        current_assets.commit_edit();
        assets.append_child(&current_assets);
        current_assets.mark_unowned();

        // Create Checking Account under Current Assets
        let mut checking = Account::new(&book);
        checking.begin_edit();
        checking.set_name("Checking Account");
        checking.set_type(GNCAccountType::ACCT_TYPE_BANK);
        checking.set_description("Primary checking account");
        checking.commit_edit();
        current_assets.append_child(&checking);
        checking.mark_unowned();

        println!("Created sub-accounts under Assets");
    }

    // Find Equity and add Opening Balances
    if let Some(equity) = root.lookup_by_name("Equity") {
        let mut opening = Account::new(&book);
        opening.begin_edit();
        opening.set_name("Opening Balances");
        opening.set_type(GNCAccountType::ACCT_TYPE_EQUITY);
        opening.commit_edit();
        equity.append_child(&opening);
        opening.mark_unowned();

        println!("Created Opening Balances under Equity");
    }

    // Print the account tree
    println!("\nAccount Tree:");
    print_account_tree(&root, 0);

    println!("\nBook created successfully!");
    println!("Note: This example creates an in-memory book.");
    println!("To persist to a file, use Session with a file URI.");
}

fn print_account_tree(account: &Account, depth: usize) {
    let indent = "  ".repeat(depth);
    let name = account.name().unwrap_or_else(|| "(unnamed)".to_string());
    let account_type = account.account_type();
    println!("{}{} [{:?}]", indent, name, account_type);

    for child in account.children() {
        print_account_tree(&child, depth + 1);
    }
}
