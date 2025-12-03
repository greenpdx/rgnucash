//! Example demonstrating session handling and file operations.
//!
//! This example shows how to:
//! - Open an existing GnuCash file
//! - Handle errors when file doesn't exist
//! - Create a new file
//! - Work with the session API
//!
//! Based on: gnucash/bindings/python/example_scripts/simple_session.py
//!
//! Usage: simple_session <gnucash_file>
//!
//! Example:
//!   simple_session /path/to/existing.gnucash
//!   simple_session xml:///tmp/new_file.gnucash

use std::env;

use gnucash_sys::{init_engine, Session, SessionOpenMode};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <gnucash_file>", args[0]);
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  {} /path/to/existing.gnucash", args[0]);
        eprintln!("  {} xml:///tmp/new_file.gnucash", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    // Initialize the GnuCash engine
    init_engine();

    // Determine if we're creating a new file or opening existing
    let is_new = file_path.starts_with("xml://")
        || file_path.starts_with("sqlite3://")
        || file_path.starts_with("mysql://")
        || file_path.starts_with("postgres://");

    if is_new {
        create_new_file(file_path);
    } else {
        open_existing_file(file_path);
    }
}

fn create_new_file(uri: &str) {
    println!("Creating new GnuCash file: {}", uri);

    match Session::open(uri, SessionOpenMode::SESSION_NEW_STORE) {
        Ok(session) => {
            println!("Session created successfully!");

            if let Some(book) = session.book() {
                println!("Book GUID: {}", book.guid());
                println!("Book is empty: {}", book.is_empty());
                println!("Book is dirty: {}", book.is_dirty());

                // Get root account
                if let Some(root) = book.root_account() {
                    root.begin_edit();
                    root.set_description("Created by Rust gnucash-sys");
                    root.commit_edit();
                    println!("Set root account description");
                }

                // Mark as dirty so it saves
                book.mark_dirty();
            }

            // Save and end session
            if session.save().is_ok() {
                println!("Session saved!");
            } else {
                eprintln!("Failed to save session");
            }

            session.end();
            println!("Session ended!");
        }
        Err(e) => {
            eprintln!("Failed to create session: {:?}", e);
            std::process::exit(1);
        }
    }
}

fn open_existing_file(path: &str) {
    println!("Opening existing GnuCash file: {}", path);

    match Session::open(path, SessionOpenMode::SESSION_NORMAL_OPEN) {
        Ok(session) => {
            println!("Session opened successfully!");

            if let Some(book) = session.book() {
                println!("Book GUID: {}", book.guid());
                println!("Book is readonly: {}", book.is_readonly());
                println!("Book is dirty: {}", book.is_dirty());
                println!("Transaction count: {}", book.transaction_count());

                // List top-level accounts
                if let Some(root) = book.root_account() {
                    println!("\nTop-level accounts:");
                    for child in root.children() {
                        let name = child.name().unwrap_or_else(|| "(unnamed)".to_string());
                        let balance = child.balance();
                        println!("  {} - Balance: {}", name, balance);
                    }
                }
            }

            session.end();
            println!("\nSession ended!");
        }
        Err(e) => {
            eprintln!("Failed to open session: {:?}", e);
            eprintln!();
            eprintln!("Common errors:");
            eprintln!("  - File not found");
            eprintln!("  - File is locked by another process");
            eprintln!("  - Invalid file format");
            std::process::exit(1);
        }
    }
}
