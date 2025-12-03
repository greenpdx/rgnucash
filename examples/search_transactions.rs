//! Example demonstrating transaction search functionality.
//!
//! This example opens a GnuCash file and searches for transactions
//! matching various criteria.
//!
//! Usage: search_transactions <gnucash_file> [search_term]

use std::env;

use gnucash_sys::{init_engine, Account, Session, SessionOpenMode, Split};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <gnucash_file> [search_term]", args[0]);
        eprintln!();
        eprintln!("Example:");
        eprintln!("  {} myfile.gnucash", args[0]);
        eprintln!("  {} myfile.gnucash \"grocery\"", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let search_term = args.get(2).map(|s| s.to_lowercase());

    init_engine();

    println!("Opening: {}", file_path);

    match Session::open(file_path, SessionOpenMode::SESSION_READ_ONLY) {
        Ok(session) => {
            if let Some(book) = session.book() {
                if let Some(root) = book.root_account() {
                    match &search_term {
                        Some(term) => {
                            println!("Searching for: \"{}\"", term);
                            println!();
                            search_transactions(&root, term);
                        }
                        None => {
                            println!("Listing recent transactions:");
                            println!();
                            list_recent_transactions(&root, 20);
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

fn search_transactions(root: &Account, search_term: &str) {
    let mut results: Vec<TransactionInfo> = Vec::new();

    // Collect all transactions from all accounts
    collect_transactions(root, &mut results, Some(search_term));

    // Sort by date descending
    results.sort_by(|a, b| b.date.cmp(&a.date));

    // Display results
    println!("Found {} matching transactions:", results.len());
    println!("{:-<80}", "");
    println!(
        "{:<12} {:<20} {:<25} {:>12}",
        "Date", "Account", "Description", "Amount"
    );
    println!("{:-<80}", "");

    let mut total = 0.0;
    for txn in &results {
        println!(
            "{:<12} {:<20} {:<25} {:>12.2}",
            format_date(txn.date),
            truncate(&txn.account, 20),
            truncate(&txn.description, 25),
            txn.amount
        );
        total += txn.amount;
    }

    println!("{:-<80}", "");
    println!(
        "{:<12} {:<20} {:<25} {:>12.2}",
        "", "", "Total:", total
    );
}

fn list_recent_transactions(root: &Account, limit: usize) {
    let mut results: Vec<TransactionInfo> = Vec::new();

    // Collect all transactions
    collect_transactions(root, &mut results, None);

    // Sort by date descending
    results.sort_by(|a, b| b.date.cmp(&a.date));

    // Take only the most recent
    let recent: Vec<_> = results.into_iter().take(limit).collect();

    println!("Most recent {} transactions:", recent.len());
    println!("{:-<80}", "");
    println!(
        "{:<12} {:<20} {:<25} {:>12}",
        "Date", "Account", "Description", "Amount"
    );
    println!("{:-<80}", "");

    for txn in &recent {
        println!(
            "{:<12} {:<20} {:<25} {:>12.2}",
            format_date(txn.date),
            truncate(&txn.account, 20),
            truncate(&txn.description, 25),
            txn.amount
        );
    }
}

struct TransactionInfo {
    date: i64,
    account: String,
    description: String,
    amount: f64,
}

fn collect_transactions(
    account: &Account,
    results: &mut Vec<TransactionInfo>,
    search_term: Option<&str>,
) {
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    collect_transactions_inner(account, results, search_term, &mut seen);
}

fn collect_transactions_inner(
    account: &Account,
    results: &mut Vec<TransactionInfo>,
    search_term: Option<&str>,
    seen: &mut std::collections::HashSet<String>,
) {
    let account_name = account.name().unwrap_or_default();

    for split in account.splits() {
        if let Some(txn) = split.transaction() {
            let guid = txn.guid().to_string();

            // Skip if we've already seen this transaction
            if !seen.insert(guid) {
                continue;
            }

            let description = txn.description().unwrap_or_default();
            let date = txn.date_posted();

            // Check if matches search term
            let matches = match search_term {
                Some(term) => {
                    description.to_lowercase().contains(term)
                        || account_name.to_lowercase().contains(term)
                        || split_matches_term(&split, term)
                }
                None => true,
            };

            if matches {
                let value = split.value();
                results.push(TransactionInfo {
                    date,
                    account: account_name.clone(),
                    description,
                    amount: value.to_f64(),
                });
            }
        }
    }

    // Recurse into children
    for child in account.children() {
        collect_transactions_inner(&child, results, search_term, seen);
    }
}

fn split_matches_term(split: &Split, term: &str) -> bool {
    if let Some(memo) = split.memo() {
        if memo.to_lowercase().contains(term) {
            return true;
        }
    }
    false
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
