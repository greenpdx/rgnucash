//! Example demonstrating how to create accounts with opening balances.
//!
//! This example creates a complete chart of accounts with opening balances,
//! simulating the setup of a new accounting system.
//!
//! Based on: gnucash/bindings/python/example_scripts/new_book_with_opening_balances.py

use gnucash_sys::{
    init_engine, Account, Book, GNCAccountType, Numeric, Split, Transaction,
};

fn main() {
    init_engine();

    println!("Creating book with opening balances...\n");

    let book = Book::new();
    let root = book.root_account().expect("Book should have root account");

    // Create the chart of accounts
    println!("Creating chart of accounts...");

    // Assets
    let assets = create_account(&book, &root, "Assets", GNCAccountType::ACCT_TYPE_ASSET);
    let current = create_account(&book, &assets, "Current Assets", GNCAccountType::ACCT_TYPE_ASSET);
    let checking = create_account(&book, &current, "Checking", GNCAccountType::ACCT_TYPE_BANK);
    let savings = create_account(&book, &current, "Savings", GNCAccountType::ACCT_TYPE_BANK);
    let cash = create_account(&book, &current, "Cash on Hand", GNCAccountType::ACCT_TYPE_CASH);

    let fixed = create_account(&book, &assets, "Fixed Assets", GNCAccountType::ACCT_TYPE_ASSET);
    let _equipment = create_account(&book, &fixed, "Equipment", GNCAccountType::ACCT_TYPE_ASSET);

    // Liabilities
    let liabilities = create_account(&book, &root, "Liabilities", GNCAccountType::ACCT_TYPE_LIABILITY);
    let credit_card = create_account(&book, &liabilities, "Credit Card", GNCAccountType::ACCT_TYPE_CREDIT);

    // Equity
    let equity = create_account(&book, &root, "Equity", GNCAccountType::ACCT_TYPE_EQUITY);
    let opening = create_account(&book, &equity, "Opening Balances", GNCAccountType::ACCT_TYPE_EQUITY);

    // Income (no opening balance needed)
    let income = create_account(&book, &root, "Income", GNCAccountType::ACCT_TYPE_INCOME);
    let _salary = create_account(&book, &income, "Salary", GNCAccountType::ACCT_TYPE_INCOME);

    // Expenses (no opening balance needed)
    let expenses = create_account(&book, &root, "Expenses", GNCAccountType::ACCT_TYPE_EXPENSE);
    let _groceries = create_account(&book, &expenses, "Groceries", GNCAccountType::ACCT_TYPE_EXPENSE);
    let _utilities = create_account(&book, &expenses, "Utilities", GNCAccountType::ACCT_TYPE_EXPENSE);

    println!("Created {} accounts", count_accounts(&root));

    // Define opening balances (in cents for precision)
    let balances = [
        (&checking, 250000i64),    // $2,500.00
        (&savings, 1000000i64),    // $10,000.00
        (&cash, 15000i64),         // $150.00
        (&credit_card, -75000i64), // -$750.00 (owed)
    ];

    println!("\nCreating opening balance transactions...");

    for (account, balance_cents) in balances {
        create_opening_balance(&book, account, &opening, balance_cents);

        let name = account.name().unwrap();
        let balance_dollars = balance_cents as f64 / 100.0;
        println!("  {} = ${:.2}", name, balance_dollars);
    }

    // Verify balances
    println!("\n--- Account Balances ---");
    print_balances(&root, 0);

    // Calculate totals
    let total_assets: f64 = [&checking, &savings, &cash]
        .iter()
        .map(|a| a.balance().to_f64())
        .sum();

    let total_liabilities = credit_card.balance().to_f64();
    let total_equity = opening.balance().to_f64();

    println!("\n--- Summary ---");
    println!("Total Assets:      ${:>12.2}", total_assets);
    println!("Total Liabilities: ${:>12.2}", total_liabilities.abs());
    println!("Total Equity:      ${:>12.2}", total_equity.abs());
    println!("                   {:->14}", "");
    println!("Balance Check:     ${:>12.2}", total_assets + total_liabilities + total_equity);

    // Clean up
    std::mem::forget(checking);
    std::mem::forget(savings);
    std::mem::forget(cash);
    std::mem::forget(credit_card);
    std::mem::forget(opening);
}

fn create_account(
    book: &Book,
    parent: &Account,
    name: &str,
    account_type: GNCAccountType,
) -> Account {
    let mut account = Account::new(book);
    account.begin_edit();
    account.set_name(name);
    account.set_type(account_type);
    account.commit_edit();
    parent.append_child(&account);
    account.mark_unowned();
    account
}

fn create_opening_balance(
    book: &Book,
    account: &Account,
    equity_account: &Account,
    amount_cents: i64,
) {
    let txn = Transaction::new(book);
    txn.begin_edit();

    let account_name = account.name().unwrap_or_default();
    txn.set_description(&format!("Opening Balance - {}", account_name));
    txn.set_date(1, 1, 2024); // January 1, 2024

    // Create split for the account
    let account_split = Split::new(book);
    account_split.set_account(account);
    account_split.set_transaction(&txn);
    let amount = Numeric::new(amount_cents, 100);
    account_split.set_amount(amount);
    account_split.set_value(amount);

    // Create balancing split for equity
    let equity_split = Split::new(book);
    equity_split.set_account(equity_account);
    equity_split.set_transaction(&txn);
    let neg_amount = Numeric::new(-amount_cents, 100);
    equity_split.set_amount(neg_amount);
    equity_split.set_value(neg_amount);

    txn.commit_edit();

    std::mem::forget(account_split);
    std::mem::forget(equity_split);
    std::mem::forget(txn);
}

fn count_accounts(account: &Account) -> usize {
    let mut count = 0;
    for child in account.children() {
        count += 1 + count_accounts(&child);
    }
    count
}

fn print_balances(account: &Account, depth: usize) {
    let indent = "  ".repeat(depth);
    let name = account.name().unwrap_or_else(|| "(root)".to_string());
    let balance = account.balance();

    if !account.is_root() && !balance.is_zero() {
        println!("{}{:<30} ${:>10.2}", indent, name, balance.to_f64());
    }

    for child in account.children() {
        print_balances(&child, depth + 1);
    }
}
