//! Example demonstrating balance sheet report generation.
//!
//! This example opens a GnuCash file and generates a simple
//! balance sheet showing assets, liabilities, and equity.
//!
//! Usage: balance_sheet <gnucash_file>

use std::env;

use gnucash_sys::{init_engine, Account, GNCAccountType, Session, SessionOpenMode};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <gnucash_file>", args[0]);
        eprintln!();
        eprintln!("Example:");
        eprintln!("  {} myfile.gnucash", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    init_engine();

    println!("Opening: {}", file_path);

    match Session::open(file_path, SessionOpenMode::SESSION_READ_ONLY) {
        Ok(session) => {
            if let Some(book) = session.book() {
                if let Some(root) = book.root_account() {
                    generate_balance_sheet(&root);
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

fn generate_balance_sheet(root: &Account) {
    println!();
    println!("================================================================================");
    println!("                              BALANCE SHEET");
    println!("================================================================================");
    println!();

    // Collect account balances by type
    let mut assets = Vec::new();
    let mut liabilities = Vec::new();
    let mut equity = Vec::new();

    collect_accounts_by_type(root, &mut assets, &mut liabilities, &mut equity, 0);

    // Print Assets
    println!("ASSETS");
    println!("{:-<60}", "");
    let total_assets = print_section(&assets);
    println!("{:-<60}", "");
    println!("{:<45} {:>14.2}", "Total Assets", total_assets);
    println!();

    // Print Liabilities
    println!("LIABILITIES");
    println!("{:-<60}", "");
    let total_liabilities = print_section(&liabilities);
    println!("{:-<60}", "");
    println!("{:<45} {:>14.2}", "Total Liabilities", total_liabilities.abs());
    println!();

    // Print Equity
    println!("EQUITY");
    println!("{:-<60}", "");
    let total_equity = print_section(&equity);
    println!("{:-<60}", "");
    println!("{:<45} {:>14.2}", "Total Equity", total_equity.abs());
    println!();

    // Summary
    println!("================================================================================");
    println!("{:<45} {:>14.2}", "Total Liabilities + Equity",
             total_liabilities.abs() + total_equity.abs());
    println!("================================================================================");
    println!();

    // Verify balance
    let diff = total_assets - (total_liabilities.abs() + total_equity.abs());
    if diff.abs() < 0.01 {
        println!("Balance Sheet is balanced.");
    } else {
        println!("WARNING: Balance Sheet is out of balance by {:.2}", diff);
    }
}

struct AccountBalance {
    name: String,
    balance: f64,
    depth: usize,
}

fn collect_accounts_by_type(
    account: &Account,
    assets: &mut Vec<AccountBalance>,
    liabilities: &mut Vec<AccountBalance>,
    equity: &mut Vec<AccountBalance>,
    depth: usize,
) {
    let acc_type = account.account_type();
    let name = account.name().unwrap_or_default();
    let balance = account.balance().to_f64();

    // Categorize by account type
    match acc_type {
        GNCAccountType::ACCT_TYPE_ASSET
        | GNCAccountType::ACCT_TYPE_BANK
        | GNCAccountType::ACCT_TYPE_CASH
        | GNCAccountType::ACCT_TYPE_STOCK
        | GNCAccountType::ACCT_TYPE_MUTUAL
        | GNCAccountType::ACCT_TYPE_RECEIVABLE => {
            if !account.is_root() {
                assets.push(AccountBalance {
                    name: name.clone(),
                    balance,
                    depth,
                });
            }
        }
        GNCAccountType::ACCT_TYPE_LIABILITY
        | GNCAccountType::ACCT_TYPE_CREDIT
        | GNCAccountType::ACCT_TYPE_PAYABLE => {
            if !account.is_root() {
                liabilities.push(AccountBalance {
                    name: name.clone(),
                    balance,
                    depth,
                });
            }
        }
        GNCAccountType::ACCT_TYPE_EQUITY => {
            if !account.is_root() {
                equity.push(AccountBalance {
                    name: name.clone(),
                    balance,
                    depth,
                });
            }
        }
        _ => {}
    }

    // Recurse into children
    for child in account.children() {
        let child_depth = if account.is_root() { 0 } else { depth + 1 };
        collect_accounts_by_type(&child, assets, liabilities, equity, child_depth);
    }
}

fn print_section(accounts: &[AccountBalance]) -> f64 {
    let mut total = 0.0;
    let mut printed_totals = std::collections::HashSet::new();

    for acc in accounts {
        let indent = "  ".repeat(acc.depth);
        let name = format!("{}{}", indent, acc.name);

        // Only add leaf accounts to total (avoid double counting)
        let is_leaf = !accounts.iter().any(|other| {
            other.depth > acc.depth && other.name != acc.name
        });

        if is_leaf || acc.depth == 0 {
            // Check if we've already counted a parent
            let key = format!("{}:{}", acc.depth, acc.name);
            if !printed_totals.contains(&key) {
                if acc.balance.abs() > 0.001 {
                    println!("{:<45} {:>14.2}", name, acc.balance.abs());
                }
                if acc.depth == 0 {
                    total += acc.balance;
                }
                printed_totals.insert(key);
            }
        } else if acc.balance.abs() > 0.001 {
            println!("{:<45} {:>14.2}", name, acc.balance.abs());
        }
    }

    // Recalculate total from top-level accounts only
    total = accounts
        .iter()
        .filter(|a| a.depth == 0)
        .map(|a| a.balance)
        .sum();

    total
}
