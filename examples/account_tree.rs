//! Example demonstrating account tree traversal and display.
//!
//! This example opens a GnuCash file and displays the complete
//! account hierarchy as a tree structure.
//!
//! Usage: account_tree <gnucash_file>

use std::env;

use gnucash_sys::{init_engine, Account, Session, SessionOpenMode};

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
    println!();

    match Session::open(file_path, SessionOpenMode::SESSION_READ_ONLY) {
        Ok(session) => {
            if let Some(book) = session.book() {
                if let Some(root) = book.root_account() {
                    println!("Account Tree");
                    println!("============");
                    println!();
                    print_account_tree(&root, 0, true);

                    println!();
                    println!("--- Statistics ---");
                    let stats = collect_stats(&root);
                    println!("Total accounts: {}", stats.total_accounts);
                    println!("Max depth: {}", stats.max_depth);
                    println!();
                    println!("Accounts by type:");
                    for (acc_type, count) in &stats.by_type {
                        println!("  {:?}: {}", acc_type, count);
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

fn print_account_tree(account: &Account, depth: usize, is_last: bool) {
    let name = account.name().unwrap_or_else(|| "(root)".to_string());
    let acc_type = account.account_type();
    let balance = account.balance();

    // Build the tree prefix
    let prefix = if depth == 0 {
        String::new()
    } else {
        let mut p = String::new();
        for _ in 0..depth - 1 {
            p.push_str("    ");
        }
        if is_last {
            p.push_str(" -> ");
        } else {
            p.push_str(" +- ");
        }
        p
    };

    // Print account info
    if account.is_root() {
        println!("[Root]");
    } else {
        let balance_str = if balance.denom() != 0 && !balance.is_zero() {
            format!(" ({:.2})", balance.to_f64())
        } else {
            String::new()
        };
        println!("{}{} [{:?}]{}", prefix, name, acc_type, balance_str);
    }

    // Process children
    let children: Vec<_> = account.children().collect();
    let child_count = children.len();

    for (i, child) in children.into_iter().enumerate() {
        let is_last_child = i == child_count - 1;
        print_account_tree(&child, depth + 1, is_last_child);
    }
}

struct AccountStats {
    total_accounts: usize,
    max_depth: usize,
    by_type: Vec<(gnucash_sys::GNCAccountType, usize)>,
}

fn collect_stats(root: &Account) -> AccountStats {
    use std::collections::HashMap;

    let mut total = 0;
    let mut max_depth = 0;
    let mut type_counts: HashMap<gnucash_sys::GNCAccountType, usize> = HashMap::new();

    fn traverse(
        account: &Account,
        depth: usize,
        total: &mut usize,
        max_depth: &mut usize,
        type_counts: &mut HashMap<gnucash_sys::GNCAccountType, usize>,
    ) {
        if !account.is_root() {
            *total += 1;
            *max_depth = (*max_depth).max(depth);
            *type_counts
                .entry(account.account_type())
                .or_insert(0) += 1;
        }

        for child in account.children() {
            traverse(&child, depth + 1, total, max_depth, type_counts);
        }
    }

    traverse(root, 0, &mut total, &mut max_depth, &mut type_counts);

    let mut by_type: Vec<_> = type_counts.into_iter().collect();
    by_type.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by count descending

    AccountStats {
        total_accounts: total,
        max_depth,
        by_type,
    }
}
