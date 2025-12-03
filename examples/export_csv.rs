//! Example demonstrating how to export transactions to CSV format.
//!
//! This example opens a GnuCash file and exports all transactions
//! for a specified account to a CSV file.
//!
//! Usage: export_csv <gnucash_file> <account_path> [output.csv]

use std::env;
use std::fs::File;
use std::io::{self, Write};

use gnucash_sys::{init_engine, Account, Session, SessionOpenMode};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <gnucash_file> <account_path> [output.csv]", args[0]);
        eprintln!();
        eprintln!("Example:");
        eprintln!("  {} myfile.gnucash \"Assets:Checking\" transactions.csv", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let account_path = &args[2];
    let output_file = args.get(3).map(|s| s.as_str());

    init_engine();

    println!("Opening: {}", file_path);

    match Session::open(file_path, SessionOpenMode::SESSION_READ_ONLY) {
        Ok(session) => {
            if let Some(book) = session.book() {
                if let Some(root) = book.root_account() {
                    match find_account_by_path(&root, account_path) {
                        Some(account) => {
                            let result = export_to_csv(&account, output_file);
                            match result {
                                Ok(count) => println!("\nExported {} transactions", count),
                                Err(e) => eprintln!("Export failed: {}", e),
                            }
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

fn export_to_csv(account: &Account, output_file: Option<&str>) -> io::Result<usize> {
    let account_name = account.full_name().unwrap_or_else(|| "Unknown".to_string());

    // Create writer (file or stdout)
    let mut writer: Box<dyn Write> = match output_file {
        Some(path) => {
            println!("Exporting to: {}", path);
            Box::new(File::create(path)?)
        }
        None => {
            println!("Exporting to stdout:\n");
            Box::new(io::stdout())
        }
    };

    // Write CSV header
    writeln!(writer, "Date,Description,Memo,Debit,Credit,Balance,Reconciled")?;

    let mut count = 0;

    for split in account.splits() {
        count += 1;

        let memo = split.memo().unwrap_or_default();
        let value = split.value();
        let balance = split.balance();
        let reconciled = split.reconcile_state();

        // Get transaction info
        let (date, description) = if let Some(txn) = split.transaction() {
            let date = format_date(txn.date_posted());
            let desc = txn.description().unwrap_or_default();
            (date, desc)
        } else {
            ("N/A".to_string(), String::new())
        };

        // Determine debit/credit
        let (debit, credit) = if value.num() >= 0 {
            (format_amount(&value), String::new())
        } else {
            (String::new(), format_amount(&value.neg()))
        };

        // Escape CSV fields
        let description = escape_csv(&description);
        let memo = escape_csv(&memo);

        writeln!(
            writer,
            "{},{},{},{},{},{},{}",
            date,
            description,
            memo,
            debit,
            credit,
            format_amount(&balance),
            reconciled
        )?;
    }

    if output_file.is_some() {
        println!("Account: {}", account_name);
    }

    Ok(count)
}

fn format_date(timestamp: i64) -> String {
    if timestamp == 0 {
        return "N/A".to_string();
    }
    // Simple date formatting
    let days = timestamp / 86400;
    let years = 1970 + days / 365;
    let day_of_year = days % 365;
    let month = day_of_year / 30 + 1;
    let day = day_of_year % 30 + 1;
    format!("{:04}-{:02}-{:02}", years, month.min(12), day.min(28))
}

fn format_amount(n: &gnucash_sys::Numeric) -> String {
    if n.denom() == 0 {
        return String::new();
    }
    format!("{:.2}", n.to_f64())
}

fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}
