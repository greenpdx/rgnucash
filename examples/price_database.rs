//! Example demonstrating price database operations.
//!
//! This example shows how to:
//! - Access the price database
//! - Add price quotes
//! - Look up prices
//!
//! Based on: gnucash/bindings/python/example_scripts/price_database_example.py

use gnucash_sys::{init_engine, Book, Numeric, Price, PriceDB, PriceSource};

fn main() {
    init_engine();

    println!("Price Database Example\n");

    let book = Book::new();

    // Get or create the price database
    let pricedb = PriceDB::get_db(&book).expect("Failed to get price database");

    println!("Created price database");

    // Create some sample prices
    // In a real application, these would be for actual commodities (stocks, currencies)

    println!("\nAdding sample prices...");

    // Add price: 1 USD = 0.85 EUR on Jan 1, 2024
    let price1 = Price::new(&book);
    price1.begin_edit();
    price1.set_time(1704067200); // Jan 1, 2024 00:00:00 UTC
    price1.set_source(PriceSource::PRICE_SOURCE_USER_PRICE);
    price1.set_type_string("last");
    price1.set_value(Numeric::new(85, 100)); // 0.85
    price1.commit_edit();

    if pricedb.add_price(&price1) {
        println!("  Added: USD/EUR = 0.85 (Jan 1, 2024)");
    }

    // Add price: 1 USD = 0.84 EUR on Jan 15, 2024
    let price2 = Price::new(&book);
    price2.begin_edit();
    price2.set_time(1705276800); // Jan 15, 2024 00:00:00 UTC
    price2.set_source(PriceSource::PRICE_SOURCE_USER_PRICE);
    price2.set_type_string("last");
    price2.set_value(Numeric::new(84, 100)); // 0.84
    price2.commit_edit();

    if pricedb.add_price(&price2) {
        println!("  Added: USD/EUR = 0.84 (Jan 15, 2024)");
    }

    // Add price: 1 USD = 0.86 EUR on Feb 1, 2024
    let price3 = Price::new(&book);
    price3.begin_edit();
    price3.set_time(1706745600); // Feb 1, 2024 00:00:00 UTC
    price3.set_source(PriceSource::PRICE_SOURCE_FQ);
    price3.set_type_string("last");
    price3.set_value(Numeric::new(86, 100)); // 0.86
    price3.commit_edit();

    if pricedb.add_price(&price3) {
        println!("  Added: USD/EUR = 0.86 (Feb 1, 2024)");
    }

    // Display prices
    println!("\n--- Price Information ---");

    display_price(&price1, "Price 1");
    display_price(&price2, "Price 2");
    display_price(&price3, "Price 3");

    // Demonstrate price inversion
    println!("\n--- Price Inversion ---");
    if let Some(inverted) = price1.invert() {
        println!("Original:  {} (1 USD = 0.85 EUR)", price1.value());
        println!("Inverted:  {} (1 EUR = {:.4} USD)", inverted.value(), 1.0 / 0.85);
    }

    // Price comparison
    println!("\n--- Price Comparison ---");
    if price1 == price1 {
        println!("price1 == price1: true");
    }
    if price1 != price2 {
        println!("price1 != price2: true (different values)");
    }

    // Remove a price
    println!("\n--- Removing Price ---");
    if pricedb.remove_price(&price2) {
        println!("Removed price2 from database");
    }

    println!("\nNote: In production, prices would be associated with");
    println!("actual commodities (stocks, currencies) from the commodity table.");

    // Clean up
    std::mem::forget(price1);
    std::mem::forget(price3);
}

fn display_price(price: &Price, label: &str) {
    let time = price.time();
    let value = price.value();
    let source = price.source();
    let type_str = price.type_string().unwrap_or_else(|| "N/A".to_string());

    println!("{}:", label);
    println!("  Time:   {} (Unix timestamp)", time);
    println!("  Value:  {}", value);
    println!("  Source: {:?}", source);
    println!("  Type:   {}", type_str);
}
