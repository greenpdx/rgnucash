//! Example demonstrating account analysis and transaction iteration.
//!
//! This example opens a GnuCash file and analyzes transactions in a
//! specified account, showing debits and credits.
//!
//! Based on: gnucash/bindings/python/example_scripts/account_analysis.py
//!
//! Usage: account_analysis <gnucash_file> <account_path>
//!
//! Example:
//!   account_analysis myfile.gnucash "Assets:Current Assets:Checking"

use std::env;

use gnucash_sys::{init_engine, Account, Numeric, Session, SessionOpenMode};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <gnucash_file> <account_path>", args[0]);
        eprintln!();
        eprintln!("Example:");
        eprintln!(
            "  {} myfile.gnucash \"Assets:Current Assets:Checking\"",
            args[0]
        );
        std::process::exit(1);
    }

    let file_path = &args[1];
    let account_path = &args[2];

    // Initialize the GnuCash engine
    init_engine();

    println!("Opening: {}", file_path);
    println!("Analyzing account: {}", account_path);
    println!();

    match Session::open(file_path, SessionOpenMode::SESSION_READ_ONLY) {
        Ok(session) => {
            if let Some(book) = session.book() {
                if let Some(root) = book.root_account() {
                    // Find the account by path
                    match find_account_by_path(&root, account_path) {
                        Some(account) => {
                            analyze_account(&account);
                        }
                        None => {
                            eprintln!("Account not found: {}", account_path);
                            eprintln!();
                            eprintln!("Available accounts:");
                            print_account_paths(&root, "");
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
    let mut current = root.clone_ref();

    for part in parts {
        match current.lookup_by_name(part) {
            Some(child) => current = child,
            None => return None,
        }
    }

    Some(current)
}

fn analyze_account(account: &Account) {
    let name = account.name().unwrap_or_else(|| "(unnamed)".to_string());
    let full_name = account.full_name().unwrap_or_else(|| name.clone());

    println!("Account: {}", full_name);
    println!("Type: {:?}", account.account_type());
    println!("Description: {}", account.description().unwrap_or_default());
    println!();

    // Get account balance
    let balance = account.balance();
    let cleared = account.cleared_balance();
    let reconciled = account.reconciled_balance();

    println!("Balances:");
    println!("  Current:    {}", format_numeric(&balance));
    println!("  Cleared:    {}", format_numeric(&cleared));
    println!("  Reconciled: {}", format_numeric(&reconciled));
    println!();

    // Analyze splits
    let mut total_debits = Numeric::zero();
    let mut total_credits = Numeric::zero();
    let mut split_count = 0;

    println!("Transactions:");
    println!("{:-<80}", "");
    println!(
        "{:<12} {:<30} {:>12} {:>12} {:>12}",
        "Date", "Description", "Debit", "Credit", "Balance"
    );
    println!("{:-<80}", "");

    for split in account.splits() {
        split_count += 1;

        let value = split.value();
        let memo = split.memo().unwrap_or_default();

        // Get transaction info
        let (date_str, description) = if let Some(txn) = split.transaction() {
            let date = txn.date_posted();
            let desc = txn.description().unwrap_or_default();
            (format_date(date), desc)
        } else {
            ("N/A".to_string(), memo)
        };

        let running_balance = split.balance();

        // Determine if debit or credit
        let is_debit = value.num() > 0;
        let (debit_str, credit_str) = if is_debit {
            total_debits = add_numeric(&total_debits, &value);
            (format_numeric(&value), "".to_string())
        } else {
            let abs_value = value.neg();
            total_credits = add_numeric(&total_credits, &abs_value);
            ("".to_string(), format_numeric(&abs_value))
        };

        println!(
            "{:<12} {:<30} {:>12} {:>12} {:>12}",
            date_str,
            truncate(&description, 30),
            debit_str,
            credit_str,
            format_numeric(&running_balance)
        );
    }

    println!("{:-<80}", "");
    println!(
        "{:<12} {:<30} {:>12} {:>12}",
        "",
        "TOTALS",
        format_numeric(&total_debits),
        format_numeric(&total_credits)
    );
    println!();
    println!("Total splits: {}", split_count);
}

fn format_numeric(n: &Numeric) -> String {
    if n.denom() == 0 {
        return "N/A".to_string();
    }
    format!("{:.2}", n.to_f64())
}

fn add_numeric(a: &Numeric, b: &Numeric) -> Numeric {
    // Simple addition - in real code you'd use gnc_numeric_add
    let denom = if a.denom() > b.denom() {
        a.denom()
    } else {
        b.denom()
    };
    let a_scaled = a.num() * (denom / a.denom().max(1));
    let b_scaled = b.num() * (denom / b.denom().max(1));
    Numeric::new(a_scaled + b_scaled, denom)
}

fn format_date(timestamp: i64) -> String {
    if timestamp == 0 {
        return "N/A".to_string();
    }
    // Simple date formatting - in production use chrono
    let days_since_epoch = timestamp / 86400;
    let years = days_since_epoch / 365 + 1970;
    let remaining_days = days_since_epoch % 365;
    let month = remaining_days / 30 + 1;
    let day = remaining_days % 30 + 1;
    format!("{:04}-{:02}-{:02}", years, month.min(12), day.min(28))
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

fn print_account_paths(account: &Account, prefix: &str) {
    let name = account.name().unwrap_or_else(|| "(root)".to_string());
    let path = if prefix.is_empty() {
        name.clone()
    } else {
        format!("{}:{}", prefix, name)
    };

    if !account.is_root() {
        println!("  {}", path);
    }

    for child in account.children() {
        let child_prefix = if account.is_root() { "" } else { &path };
        print_account_paths(&child, child_prefix);
    }
}

// Helper trait for cloning account references
trait CloneRef {
    fn clone_ref(&self) -> Self;
}

impl CloneRef for Account {
    fn clone_ref(&self) -> Self {
        unsafe { Account::from_raw(self.as_ptr(), false).unwrap() }
    }
}
