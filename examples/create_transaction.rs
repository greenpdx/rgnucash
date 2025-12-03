//! Example demonstrating how to create transactions with splits.
//!
//! This example creates a simple transaction with two splits:
//! - A debit to an expense account
//! - A credit from a bank account

use gnucash_sys::{init_engine, Account, Book, GNCAccountType, Numeric, Split, Transaction};

fn main() {
    // Initialize the GnuCash engine
    init_engine();

    println!("Creating book with sample transaction...\n");

    // Create a new book
    let book = Book::new();

    // Create account structure
    let root = book.root_account().expect("Book should have root account");

    // Create Assets:Bank account
    let assets = create_account(&book, &root, "Assets", GNCAccountType::ACCT_TYPE_ASSET);
    let bank = create_account(&book, &assets, "Checking", GNCAccountType::ACCT_TYPE_BANK);

    // Create Expenses:Groceries account
    let expenses = create_account(&book, &root, "Expenses", GNCAccountType::ACCT_TYPE_EXPENSE);
    let groceries = create_account(&book, &expenses, "Groceries", GNCAccountType::ACCT_TYPE_EXPENSE);

    println!("Created accounts:");
    println!("  Assets:Checking");
    println!("  Expenses:Groceries");
    println!();

    // Create a transaction: Buy groceries for $50.00
    println!("Creating transaction: Grocery shopping ($50.00)");

    let txn = Transaction::new(&book);
    txn.begin_edit();

    txn.set_description("Weekly grocery shopping");
    txn.set_num("1001");
    txn.set_notes("Bought food for the week");

    // Set the date (using Unix timestamp - Jan 15, 2024)
    // In production, use proper date handling
    txn.set_date(15, 1, 2024);

    // Create the expense split (debit - positive in expense account)
    let expense_split = Split::new(&book);
    expense_split.set_account(&groceries);
    expense_split.set_transaction(&txn);
    expense_split.set_memo("Groceries");

    // $50.00 = 5000 cents / 100
    let amount = Numeric::new(5000, 100);
    expense_split.set_amount(amount);
    expense_split.set_value(amount);

    // Create the bank split (credit - negative from bank account)
    let bank_split = Split::new(&book);
    bank_split.set_account(&bank);
    bank_split.set_transaction(&txn);
    bank_split.set_memo("Debit card");

    let neg_amount = Numeric::new(-5000, 100);
    bank_split.set_amount(neg_amount);
    bank_split.set_value(neg_amount);

    txn.commit_edit();

    println!("  Description: {}", txn.description().unwrap());
    println!("  Date: 2024-01-15");
    println!("  Splits: {}", txn.split_count());
    println!("  Is balanced: {}", txn.is_balanced());
    println!();

    // Print split details
    println!("Split details:");
    for (i, split) in txn.splits().enumerate() {
        let account = split.account().map(|a| a.name().unwrap_or_default()).unwrap_or_default();
        let amount = split.amount();
        let memo = split.memo().unwrap_or_default();

        println!("  Split {}:", i + 1);
        println!("    Account: {}", account);
        println!("    Amount: {}", amount);
        println!("    Memo: {}", memo);
    }

    // Check balances
    println!();
    println!("Account balances after transaction:");
    println!("  Checking: {}", bank.balance());
    println!("  Groceries: {}", groceries.balance());

    // Verify transaction balance
    let imbalance = txn.imbalance_value();
    println!();
    if imbalance.is_zero() {
        println!("Transaction is balanced (imbalance = 0)");
    } else {
        println!("WARNING: Transaction is unbalanced! Imbalance: {}", imbalance);
    }

    println!();
    println!("Note: This example creates an in-memory book.");
    println!("Use Session to persist to a file.");

    // Clean up - let splits be destroyed with transaction
    std::mem::forget(expense_split);
    std::mem::forget(bank_split);
    std::mem::forget(groceries);
    std::mem::forget(expenses);
    std::mem::forget(bank);
    std::mem::forget(assets);
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
