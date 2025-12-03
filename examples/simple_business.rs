//! Example demonstrating business features: customers, vendors, invoices.
//!
//! This example creates a book with business entities:
//! - Customer with billing address
//! - Vendor
//! - Employee
//! - Invoice with entries
//!
//! Based on: gnucash/bindings/python/example_scripts/simple_business_create.py

use gnucash_sys::{
    init_engine, Account, Book, Customer, Employee, Entry, GNCAccountType, Invoice, Numeric,
    Owner, Vendor,
};

fn main() {
    // Initialize the GnuCash engine
    init_engine();

    println!("Creating book with business entities...\n");

    // Create a new book
    let book = Book::new();

    // Create basic account structure
    let root = book.root_account().expect("Book should have root account");

    // Create accounts needed for business
    let assets = create_account(&book, &root, "Assets", GNCAccountType::ACCT_TYPE_ASSET);
    let receivable = create_account(
        &book,
        &assets,
        "Accounts Receivable",
        GNCAccountType::ACCT_TYPE_RECEIVABLE,
    );
    let _payable = create_account(
        &book,
        &assets,
        "Accounts Payable",
        GNCAccountType::ACCT_TYPE_PAYABLE,
    );
    let income = create_account(&book, &root, "Income", GNCAccountType::ACCT_TYPE_INCOME);
    let sales = create_account(&book, &income, "Sales", GNCAccountType::ACCT_TYPE_INCOME);

    // Create a Customer
    println!("Creating Customer...");
    let customer = Customer::new(&book);
    customer.begin_edit();
    customer.set_id("CUST001");
    customer.set_name("Acme Corporation");
    customer.set_notes("Our biggest customer");
    customer.set_active(true);

    // Set customer address
    if let Some(addr) = customer.address() {
        addr.begin_edit();
        addr.set_name("Acme Corporation");
        addr.set_addr1("123 Main Street");
        addr.set_addr2("Suite 100");
        addr.set_addr3("Springfield, IL 62701");
        addr.set_phone("555-123-4567");
        addr.set_email("billing@acme.com");
        addr.commit_edit();
    }

    customer.commit_edit();
    println!("  Created: {} - {}", customer.id().unwrap(), customer.name().unwrap());

    // Create a Vendor
    println!("Creating Vendor...");
    let vendor = Vendor::new(&book);
    vendor.begin_edit();
    vendor.set_id("VEND001");
    vendor.set_name("Office Supplies Inc.");
    vendor.set_notes("Office supply vendor");
    vendor.set_active(true);

    if let Some(addr) = vendor.address() {
        addr.begin_edit();
        addr.set_name("Office Supplies Inc.");
        addr.set_addr1("456 Commerce Blvd");
        addr.set_phone("555-987-6543");
        addr.commit_edit();
    }

    vendor.commit_edit();
    println!("  Created: {} - {}", vendor.id().unwrap(), vendor.name().unwrap());

    // Create an Employee
    println!("Creating Employee...");
    let employee = Employee::new(&book);
    employee.begin_edit();
    employee.set_id("EMP001");
    employee.set_name("John Smith");
    employee.set_username("jsmith");
    employee.set_active(true);
    employee.set_rate(Numeric::new(5000, 100)); // $50.00/hour
    employee.set_workday(Numeric::new(800, 100)); // 8 hours

    if let Some(addr) = employee.address() {
        addr.begin_edit();
        addr.set_name("John Smith");
        addr.set_addr1("789 Employee Lane");
        addr.set_phone("555-111-2222");
        addr.commit_edit();
    }

    employee.commit_edit();
    println!(
        "  Created: {} - {} (rate: {})",
        employee.id().unwrap(),
        employee.name().unwrap(),
        employee.rate()
    );

    // Create an Invoice for the Customer
    println!("\nCreating Invoice...");
    let invoice = Invoice::new(&book);
    invoice.begin_edit();
    invoice.set_id("INV-001");
    invoice.set_notes("Professional services rendered");

    // Set the owner to the customer
    let owner = Owner::from_customer(&customer);
    invoice.set_owner(&owner);

    invoice.commit_edit();

    // Add entries to the invoice
    println!("Adding invoice entries...");

    let entry1 = Entry::new(&book);
    entry1.begin_edit();
    entry1.set_description("Consulting services - January");
    entry1.set_quantity(Numeric::new(10, 1)); // 10 hours
    entry1.set_inv_price(Numeric::new(15000, 100)); // $150.00/hour
    entry1.set_inv_account(&sales);
    entry1.commit_edit();
    invoice.add_entry(&entry1);
    println!("  Added: {} - qty: {}, price: {}",
        entry1.description().unwrap(),
        entry1.quantity(),
        entry1.inv_price()
    );

    let entry2 = Entry::new(&book);
    entry2.begin_edit();
    entry2.set_description("Software license fee");
    entry2.set_quantity(Numeric::new(1, 1)); // 1 unit
    entry2.set_inv_price(Numeric::new(50000, 100)); // $500.00
    entry2.set_inv_account(&sales);
    entry2.commit_edit();
    invoice.add_entry(&entry2);
    println!("  Added: {} - qty: {}, price: {}",
        entry2.description().unwrap(),
        entry2.quantity(),
        entry2.inv_price()
    );

    // Calculate expected total
    // Entry 1: 10 * $150 = $1500
    // Entry 2: 1 * $500 = $500
    // Total: $2000
    println!("\nInvoice Summary:");
    println!("  Invoice ID: {}", invoice.id().unwrap());
    println!("  Owner: {} ({:?})", owner.name().unwrap(), owner.owner_type());
    println!("  Is Posted: {}", invoice.is_posted());
    println!("  Is Paid: {}", invoice.is_paid());

    // Post the invoice (in a real scenario)
    // Note: Posting requires proper date handling and account setup
    // invoice.post_to_account(&receivable, now, due_date, "Invoice payment", false, false);

    println!("\n--- Summary ---");
    println!("Created:");
    println!("  1 Customer: {}", customer.name().unwrap());
    println!("  1 Vendor: {}", vendor.name().unwrap());
    println!("  1 Employee: {}", employee.name().unwrap());
    println!("  1 Invoice with 2 entries");
    println!("\nNote: This example creates an in-memory book.");
    println!("Use Session to persist to a file.");

    // Prevent drops from trying to destroy - in real code you'd
    // let the session own these
    std::mem::forget(entry1);
    std::mem::forget(entry2);
    std::mem::forget(invoice);
    std::mem::forget(customer);
    std::mem::forget(vendor);
    std::mem::forget(employee);
    std::mem::forget(receivable);
    std::mem::forget(sales);
    std::mem::forget(income);
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
