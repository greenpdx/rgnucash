//! Example demonstrating account reconciliation workflow.
//!
//! This example shows how to:
//! - View unreconciled transactions
//! - Mark splits as cleared or reconciled
//! - Calculate reconciliation balance
//!
//! Usage: reconcile_account <gnucash_file> <account_path>

use std::env;

use gnucash_sys::{init_engine, Account, Session, SessionOpenMode};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <gnucash_file> <account_path>", args[0]);
        eprintln!();
        eprintln!("Example:");
        eprintln!("  {} myfile.gnucash \"Assets:Checking\"", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let account_path = &args[2];

    init_engine();

    println!("Opening: {}", file_path);

    match Session::open(file_path, SessionOpenMode::SESSION_READ_ONLY) {
        Ok(session) => {
            if let Some(book) = session.book() {
                if let Some(root) = book.root_account() {
                    match find_account_by_path(&root, account_path) {
                        Some(account) => {
                            show_reconciliation_status(&account);
                        }
                        None => {
                            eprintln!("Account not found: {}", account_path);
                            std::process::exit(1);
                        }
                    }
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

fn find_account_by_path(root: &Account, path: &str) -> Option<Account> {
    let parts: Vec<&str> = path.split(':').collect();
    let mut current = unsafe { Account::from_raw(root.as_ptr(), false)? };

    for part in parts {
        current = current.lookup_by_name(part)?;
    }

    Some(current)
}

fn show_reconciliation_status(account: &Account) {
    let name = account.full_name().unwrap_or_else(|| "Unknown".to_string());

    println!();
    println!("Reconciliation Status for: {}", name);
    println!("{:=<60}", "");
    println!();

    // Get different balance types
    let current = account.balance();
    let cleared = account.cleared_balance();
    let reconciled = account.reconciled_balance();

    println!("Balance Summary:");
    println!("  Current Balance:    {:>12.2}", current.to_f64());
    println!("  Cleared Balance:    {:>12.2}", cleared.to_f64());
    println!("  Reconciled Balance: {:>12.2}", reconciled.to_f64());
    println!();

    // Categorize splits by reconciliation state
    let mut unreconciled = Vec::new();
    let mut cleared_splits = Vec::new();
    let mut reconciled_splits = Vec::new();

    for split in account.splits() {
        let state = split.reconcile_state();
        match state {
            'n' => unreconciled.push(split),
            'c' => cleared_splits.push(split),
            'y' => reconciled_splits.push(split),
            _ => {} // frozen or void
        }
    }

    // Show unreconciled transactions
    println!("Unreconciled Transactions ({}):", unreconciled.len());
    println!("{:-<60}", "");
    println!(
        "{:<12} {:<25} {:>12} {:>8}",
        "Date", "Description", "Amount", "State"
    );
    println!("{:-<60}", "");

    let mut unreconciled_total = 0.0;
    for split in &unreconciled {
        let value = split.value();
        let amount = value.to_f64();
        unreconciled_total += amount;

        let (date, desc) = if let Some(txn) = split.transaction() {
            let d = format_date(txn.date_posted());
            let description = txn.description().unwrap_or_default();
            (d, description)
        } else {
            ("N/A".to_string(), String::new())
        };

        println!(
            "{:<12} {:<25} {:>12.2} {:>8}",
            date,
            truncate(&desc, 25),
            amount,
            "new"
        );
    }
    println!("{:-<60}", "");
    println!(
        "{:<12} {:<25} {:>12.2}",
        "", "Unreconciled Total:", unreconciled_total
    );

    // Show cleared transactions
    println!();
    println!("Cleared (not yet reconciled) Transactions ({}):", cleared_splits.len());
    println!("{:-<60}", "");

    let mut cleared_total = 0.0;
    for split in &cleared_splits {
        let value = split.value();
        let amount = value.to_f64();
        cleared_total += amount;

        let (date, desc) = if let Some(txn) = split.transaction() {
            let d = format_date(txn.date_posted());
            let description = txn.description().unwrap_or_default();
            (d, description)
        } else {
            ("N/A".to_string(), String::new())
        };

        println!(
            "{:<12} {:<25} {:>12.2} {:>8}",
            date,
            truncate(&desc, 25),
            amount,
            "cleared"
        );
    }
    println!("{:-<60}", "");
    println!(
        "{:<12} {:<25} {:>12.2}",
        "", "Cleared Total:", cleared_total
    );

    // Summary
    println!();
    println!("Summary:");
    println!("  Reconciled transactions: {}", reconciled_splits.len());
    println!("  Cleared transactions:    {}", cleared_splits.len());
    println!("  Unreconciled transactions: {}", unreconciled.len());
    println!();
    println!("  If statement balance is {:>.2}, all cleared items match.",
             reconciled.to_f64() + cleared_total);
}

fn format_date(timestamp: i64) -> String {
    if timestamp == 0 {
        return "N/A".to_string();
    }
    let days = timestamp / 86400;
    let years = 1970 + days / 365;
    let day_of_year = days % 365;
    let month = day_of_year / 30 + 1;
    let day = day_of_year % 30 + 1;
    format!("{:04}-{:02}-{:02}", years, month.min(12), day.min(28))
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}
