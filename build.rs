use std::env;
use std::path::PathBuf;

fn main() {
    // Skip build script on docs.rs - we use pre-generated bindings
    if env::var("DOCS_RS").is_ok() {
        return;
    }

    // Load .env file if present
    let _ = dotenvy::dotenv();

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=.env");

    // Get paths from environment or use defaults
    let lib_path = PathBuf::from(
        env::var("GNUCASH_LIB_PATH")
            .unwrap_or_else(|_| "/usr/lib/aarch64-linux-gnu/gnucash".to_string()),
    );
    let include_path = PathBuf::from(
        env::var("GNUCASH_INCLUDE_PATH")
            .unwrap_or_else(|_| "/usr/include/gnucash".to_string()),
    );

    println!("cargo:warning=GnuCash lib: {}", lib_path.display());
    println!("cargo:warning=GnuCash include: {}", include_path.display());

    // Get glib-2.0 flags via pkg-config
    let glib = pkg_config::Config::new()
        .atleast_version("2.56")
        .probe("glib-2.0")
        .expect("glib-2.0 not found via pkg-config");

    // Library search path
    println!("cargo:rustc-link-search=native={}", lib_path.display());

    // Add rpath so the library can be found at runtime
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_path.display());

    // Link against gnucash engine library
    println!("cargo:rustc-link-lib=gnc-engine");

    // Build bindgen bindings
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", include_path.display()))
        .clang_arg(format!("-I{}", manifest_dir.display()))
        // Core types
        .allowlist_type("GncGUID")
        .allowlist_type("_gncGuid")
        .allowlist_type("gnc_numeric")
        .allowlist_type("_gnc_numeric")
        .allowlist_type("time64")
        .allowlist_type("Time64")
        .allowlist_type("GNCNumericErrorCode")
        // Entity types
        .allowlist_type("Split")
        .allowlist_type("SplitClass")
        .allowlist_type("Transaction")
        .allowlist_type("TransactionClass")
        .allowlist_type("Account")
        .allowlist_type("AccountClass")
        .allowlist_type("QofBook")
        .allowlist_type("_QofBook")
        .allowlist_type("QofCollection")
        .allowlist_type("QofInstance")
        .allowlist_type("GNCAccountType")
        .allowlist_type("GNCPlaceholderType")
        .allowlist_type("SplitList")
        .allowlist_type("MonetaryList")
        .allowlist_type("GNCLot")
        // Session types
        .allowlist_type("QofSession")
        .allowlist_type("QofSessionImpl")
        .allowlist_type("SessionOpenMode")
        .allowlist_type("QofBackendError")
        // GUID functions
        .allowlist_function("guid_.*")
        .allowlist_function("string_to_guid")
        // gnc_numeric functions
        .allowlist_function("gnc_numeric_.*")
        .allowlist_function("double_to_gnc_numeric")
        // Date/time functions
        .allowlist_function("gnc_time.*")
        .allowlist_function("gnc_mktime")
        .allowlist_function("gnc_gmtime")
        .allowlist_function("gnc_localtime.*")
        .allowlist_function("gnc_dmy2time64.*")
        .allowlist_function("gnc_iso8601_to_time64_gmt")
        .allowlist_function("gnc_time64_to_iso8601_buff")
        .allowlist_function("time64_to_gdate")
        .allowlist_function("gdate_to_time64")
        // Entity functions
        .allowlist_function("xacc.*")
        .allowlist_function("gnc_.*")
        .allowlist_function("qof_.*")
        // Price types and functions
        .allowlist_type("GNCPrice")
        .allowlist_type("GNCPriceDB")
        .allowlist_type("PriceSource")
        .allowlist_function("gnc_price_.*")
        .allowlist_function("gnc_pricedb_.*")
        .rustified_enum("PriceSource")
        // Query types and functions
        .allowlist_type("QofQuery")
        .allowlist_type("QofQueryOp")
        .allowlist_type("QofQueryCompare")
        .allowlist_type("QofStringMatch")
        .allowlist_type("QofDateMatch")
        .allowlist_type("QofNumericMatch")
        .allowlist_type("QofGuidMatch")
        .allowlist_type("QofCharMatch")
        .rustified_enum("QofQueryOp")
        .rustified_enum("QofQueryCompare")
        .rustified_enum("QofStringMatch")
        .rustified_enum("QofDateMatch")
        .rustified_enum("QofNumericMatch")
        .rustified_enum("QofGuidMatch")
        .rustified_enum("QofCharMatch")
        // Business types
        .allowlist_type("GncAddress")
        .allowlist_type("GncCustomer")
        .allowlist_type("GncVendor")
        .allowlist_type("GncEmployee")
        .allowlist_type("GncJob")
        .allowlist_type("GncInvoice")
        .allowlist_type("GncEntry")
        .allowlist_type("GncBillTerm")
        .allowlist_type("GncTaxTable")
        .allowlist_type("GncTaxTableEntry")
        .allowlist_type("GncOwner")
        .allowlist_type("GncOwnerType")
        .allowlist_type("GncInvoiceType")
        .allowlist_type("GncEntryPaymentType")
        .allowlist_type("GncDiscountHow")
        .allowlist_type("GncAmountType")
        .allowlist_type("GncBillTermType")
        // Business functions
        .allowlist_function("gncAddress.*")
        .allowlist_function("gncCustomer.*")
        .allowlist_function("gncVendor.*")
        .allowlist_function("gncEmployee.*")
        .allowlist_function("gncJob.*")
        .allowlist_function("gncInvoice.*")
        .allowlist_function("gncEntry.*")
        .allowlist_function("gncBillTerm.*")
        .allowlist_function("gncTaxTable.*")
        .allowlist_function("gncOwner.*")
        // Business enums
        .rustified_enum("GncOwnerType")
        .rustified_enum("GncInvoiceType")
        .rustified_enum("GncEntryPaymentType")
        .rustified_enum("GncDiscountHow")
        .rustified_enum("GncAmountType")
        .rustified_enum("GncBillTermType")
        // GLib types we need
        .allowlist_type("GList")
        .allowlist_type("_GList")
        .allowlist_type("GDate")
        .allowlist_type("_GDate")
        .allowlist_type("GValue")
        .allowlist_type("_GValue")
        .allowlist_type("GHashTable")
        .allowlist_type("_GHashTable")
        .allowlist_function("g_free")
        .allowlist_function("g_list_.*")
        .allowlist_function("g_slist_.*")
        .allowlist_function("g_date_.*")
        .allowlist_type("GSList")
        .allowlist_type("_GSList")
        // Generate Rust enums for C enums
        .rustified_enum("GNCAccountType")
        .rustified_enum("GNCNumericErrorCode")
        .rustified_enum("GNCPlaceholderType")
        .rustified_enum("QofDateFormat")
        .rustified_enum("QofDateCompletion")
        .rustified_enum("SessionOpenMode")
        .rustified_enum("QofBackendError")
        // Derive common traits
        .derive_debug(true)
        .derive_default(true)
        .derive_eq(true)
        .derive_hash(true)
        // Rust 2024 compatibility
        .wrap_unsafe_ops(true)
        // Disable doc comments to avoid doctest failures from C code examples
        .generate_comments(false);

    // Add glib include paths
    for path in &glib.include_paths {
        builder = builder.clang_arg(format!("-I{}", path.display()));
    }

    let bindings = builder
        .generate()
        .expect("Unable to generate bindings");

    // Write bindings to OUT_DIR
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
