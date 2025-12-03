//! Example demonstrating how to list all accounts in a GnuCash file.
//!
//! This example opens a GnuCash file and prints all accounts with their
//! balances in a tree structure.
//!
//! Usage: list_accounts <gnucash_file>

use std::env;

use gnucash_sys::{init_engine, Account, GNCAccountType, Session, SessionOpenMode};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <gnucash_file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    // Initialize the GnuCash engine
    init_engine();

    println!("Opening: {}", file_path);

    match Session::open(file_path, SessionOpenMode::SESSION_READ_ONLY) {
        Ok(session) => {
            if let Some(book) = session.book() {
                if let Some(root) = book.root_account() {
                    println!("\nAccount Hierarchy:");
                    println!("{:-<60}", "");
                    print_account(&root, 0);
                    println!("{:-<60}", "");

                    // Print summary statistics
                    let total_accounts = count_accounts(&root);
                    println!("\nTotal accounts: {}", total_accounts);
                    println!("Total transactions: {}", book.transaction_count());
                }
            }
            session.end();
        }
        Err(e) => {
            eprintln!("Failed to open file: {:?}", e);
            std::process::exit(1);
        }
    }
}

fn print_account(account: &Account, depth: usize) {
    let indent = "  ".repeat(depth);
    let name = account.name().unwrap_or_else(|| "(root)".to_string());
    let account_type = account.account_type();
    let balance = account.balance();

    // Format balance for display
    let balance_str = if balance.denom() > 0 {
        format!("{:.2}", balance.to_f64())
    } else {
        "-".to_string()
    };

    // Get account type abbreviation
    let type_abbrev = match account_type {
        GNCAccountType::ACCT_TYPE_BANK => "BANK",
        GNCAccountType::ACCT_TYPE_CASH => "CASH",
        GNCAccountType::ACCT_TYPE_ASSET => "ASSET",
        GNCAccountType::ACCT_TYPE_CREDIT => "CREDIT",
        GNCAccountType::ACCT_TYPE_LIABILITY => "LIAB",
        GNCAccountType::ACCT_TYPE_STOCK => "STOCK",
        GNCAccountType::ACCT_TYPE_MUTUAL => "MUTUAL",
        GNCAccountType::ACCT_TYPE_INCOME => "INCOME",
        GNCAccountType::ACCT_TYPE_EXPENSE => "EXPENSE",
        GNCAccountType::ACCT_TYPE_EQUITY => "EQUITY",
        GNCAccountType::ACCT_TYPE_RECEIVABLE => "A/R",
        GNCAccountType::ACCT_TYPE_PAYABLE => "A/P",
        GNCAccountType::ACCT_TYPE_ROOT => "ROOT",
        GNCAccountType::ACCT_TYPE_TRADING => "TRADE",
        _ => "OTHER",
    };

    // Print account info
    if depth == 0 {
        println!("{}{}", indent, name);
    } else {
        println!(
            "{}{:<30} {:>8} {:>12}",
            indent,
            truncate(&name, 30 - depth * 2),
            type_abbrev,
            balance_str
        );
    }

    // Recursively print children
    for child in account.children() {
        print_account(&child, depth + 1);
    }
}

fn count_accounts(account: &Account) -> usize {
    let mut count = 1;
    for child in account.children() {
        count += count_accounts(&child);
    }
    count
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}
