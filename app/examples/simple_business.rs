//! Example demonstrating business features.
//!
//! This example creates customers, vendors, employees, jobs, and invoices.
//!
//! Based on: gnucash/bindings/python/example_scripts/simple_business_create.py

use gnucash_ext::business::billterm::{BillTerm, BillTermType};
use gnucash_ext::business::tax::{AmountType, DiscountHow};
use gnucash_ext::{
    init_engine, Book, Customer, Employee, Entry, GNCAccountType, Invoice, Job, Numeric, Vendor,
};

fn main() {
    init_engine();

    println!("Creating business entities...\n");

    let book = Book::new();

    // Create a root account (needed for posting invoices)
    let root = gnucash_ext::Account::new(&book);
    root.begin_edit();
    root.set_name("Root");
    root.set_type(GNCAccountType::ACCT_TYPE_ROOT);
    root.commit_edit();
    book.set_root_account(&root);

    // Create a bill term (Net 30) — used by both customer + invoice
    // below to demonstrate the new set_terms wrappers.
    println!("Creating bill term (Net 30)...");
    let net30 = BillTerm::new(&book);
    net30.begin_edit();
    net30.set_name("Net 30");
    net30.set_description("Payment due in 30 days");
    net30.set_type(BillTermType::GNC_TERM_TYPE_DAYS);
    net30.set_due_days(30);
    net30.commit_edit();

    // Create a customer
    println!("\nCreating customer...");
    let customer = Customer::new(&book);
    customer.begin_edit();
    customer.set_id("CUST001");
    customer.set_name("Acme Corporation");
    customer.set_notes("Our best customer");
    customer.set_active(true);
    // Default this customer's invoices to Net-30 unless overridden.
    customer.set_terms(&net30);
    // (Also available: customer.set_tax_table(&t) +
    //                  customer.set_tax_table_override(true).)

    // Set customer address
    if let Some(addr) = customer.addr() {
        addr.set_name("Acme Corporation");
        addr.set_addr1("123 Main Street");
        addr.set_addr2("Suite 100");
        addr.set_addr3("Anytown, ST 12345");
        addr.set_phone("555-1234");
        addr.set_email("billing@acme.com");
    }

    customer.commit_edit();
    println!("  Customer: {} ({})", customer.name().unwrap(), customer.id().unwrap());

    // Create a vendor
    println!("\nCreating vendor...");
    let vendor = Vendor::new(&book);
    vendor.begin_edit();
    vendor.set_id("VEND001");
    vendor.set_name("Office Supplies Inc");
    vendor.set_notes("Office supply vendor");
    vendor.set_active(true);

    if let Some(addr) = vendor.addr() {
        addr.set_name("Office Supplies Inc");
        addr.set_addr1("456 Commerce Blvd");
        addr.set_addr3("Business City, ST 54321");
    }

    vendor.commit_edit();
    println!("  Vendor: {} ({})", vendor.name().unwrap(), vendor.id().unwrap());

    // Create an employee
    println!("\nCreating employee...");
    let employee = Employee::new(&book);
    employee.begin_edit();
    employee.set_id("EMP001");
    employee.set_username("jsmith");
    employee.set_active(true);
    employee.set_workday(Numeric::new(8, 1)); // 8 hours
    employee.set_rate(Numeric::new(5000, 100)); // $50.00/hour

    if let Some(addr) = employee.addr() {
        addr.set_name("John Smith");
        addr.set_addr1("789 Employee Lane");
    }

    employee.commit_edit();
    println!(
        "  Employee: {} ({})",
        employee.username().unwrap(),
        employee.id().unwrap()
    );

    // Create a job for the customer
    println!("\nCreating job...");
    let job = Job::new(&book);
    job.begin_edit();
    job.set_id("JOB001");
    job.set_name("Website Redesign");
    job.set_reference("Project #2024-001");
    job.set_active(true);

    // Link job to customer
    let customer_owner = customer.to_owner();
    job.set_owner(&customer_owner);

    job.commit_edit();
    println!("  Job: {} ({})", job.name().unwrap(), job.id().unwrap());

    // Create an invoice for the customer
    println!("\nCreating invoice...");
    let invoice = Invoice::new(&book);
    invoice.begin_edit();
    invoice.set_id("INV-001");
    invoice.set_notes("Invoice for consulting services");

    // Set the invoice owner to the customer
    invoice.set_owner(&customer_owner);

    // Demo the Invoice setters added in this gap-fill PR.
    invoice.set_terms(&net30);
    invoice.set_active(true);
    invoice.set_doc_link("https://acme.example/po/2024-001.pdf");
    // (Also available: invoice.set_is_credit_note(true) for an NC,
    //                  invoice.set_bill_to(&other_owner).)

    invoice.commit_edit();

    // Add entries to the invoice
    println!("Adding invoice entries...");

    let entry1 = Entry::new(&book);
    entry1.set_date(1704067200); // Jan 1, 2024
    entry1.set_description("Consulting - Day 1");
    entry1.set_quantity(Numeric::new(8, 1)); // 8 hours
    entry1.set_inv_price(Numeric::new(15000, 100)); // $150/hour
    // 10% loyalty discount, applied to the price before tax — uses
    // the Entry::set_inv_discount_type / _how setters added in this
    // gap-fill PR. Without these, the discount would still apply but
    // its interpretation (% vs absolute) and ordering (pre/post tax)
    // would default to whatever the last edit on this entry chose.
    entry1.set_inv_discount(Numeric::new(10, 1)); // 10
    entry1.set_inv_discount_type(AmountType::GNC_AMT_TYPE_PERCENT);
    entry1.set_inv_discount_how(DiscountHow::GNC_DISC_PRETAX);
    invoice.add_entry(&entry1);
    println!("  Entry 1: Consulting - Day 1 (8 hrs @ $150, 10% pretax discount)");

    let entry2 = Entry::new(&book);
    entry2.set_date(1704153600); // Jan 2, 2024
    entry2.set_description("Consulting - Day 2");
    entry2.set_quantity(Numeric::new(6, 1)); // 6 hours
    entry2.set_inv_price(Numeric::new(15000, 100)); // $150/hour
    invoice.add_entry(&entry2);
    println!("  Entry 2: Consulting - Day 2 (6 hrs @ $150)");

    // Display invoice summary
    println!("\n--- Invoice Summary ---");
    println!("Invoice ID: {}", invoice.id().unwrap());
    println!("Customer: {}", customer.name().unwrap());
    println!("Total: {}", invoice.total());
    println!("Is Posted: {}", invoice.is_posted());
    println!("Is Paid: {}", invoice.is_paid());

    // Display all created entities
    println!("\n--- Created Entities ---");
    println!("Customer: {:?}", customer);
    println!("Vendor: {:?}", vendor);
    println!("Employee: {:?}", employee);
    println!("Job: {:?}", job);
    println!("Invoice: {:?}", invoice);

    // Clean up - forget owned entities to prevent double-free
    std::mem::forget(entry1);
    std::mem::forget(entry2);
    std::mem::forget(invoice);
    std::mem::forget(job);
    std::mem::forget(customer);
    std::mem::forget(vendor);
    std::mem::forget(employee);
    std::mem::forget(net30);
    std::mem::forget(root);
}
