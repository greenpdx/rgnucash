use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=build.rs");

    // GnuCash source directory (adjust as needed)
    let gnucash_src = PathBuf::from(
        env::var("GNUCASH_SRC").unwrap_or_else(|_| "../gnucash".to_string()),
    );

    // GnuCash build directory (for generated headers and libraries)
    let gnucash_build = PathBuf::from(
        env::var("GNUCASH_BUILD").unwrap_or_else(|_| "../gnucash/build".to_string()),
    );

    // Header include paths
    let engine_include = gnucash_src.join("libgnucash/engine");
    let core_utils_include = gnucash_src.join("libgnucash/core-utils");
    let qof_include = gnucash_src.join("libgnucash/engine");

    // Get glib-2.0 flags via pkg-config
    let glib = pkg_config::Config::new()
        .atleast_version("2.56")
        .probe("glib-2.0")
        .expect("glib-2.0 not found via pkg-config");

    // Library search path
    let lib_path = gnucash_build.join("lib");
    println!("cargo:rustc-link-search=native={}", lib_path.display());

    // Link against gnucash engine library
    println!("cargo:rustc-link-lib=gnc-engine");

    // Build bindgen bindings
    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", engine_include.display()))
        .clang_arg(format!("-I{}", core_utils_include.display()))
        .clang_arg(format!("-I{}", qof_include.display()))
        .clang_arg(format!("-I{}", gnucash_build.join("common").display()))
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
        // Block problematic types from glib
        .blocklist_type("_?GList")
        .blocklist_type("_?GSList")
        .blocklist_type("_?GHashTable")
        .blocklist_type("_?GValue")
        .blocklist_type("_?GDate")
        // Generate Rust enums for C enums
        .rustified_enum("GNCAccountType")
        .rustified_enum("GNCNumericErrorCode")
        .rustified_enum("GNCPlaceholderType")
        .rustified_enum("QofDateFormat")
        .rustified_enum("QofDateCompletion")
        // Derive common traits
        .derive_debug(true)
        .derive_default(true)
        .derive_eq(true)
        .derive_hash(true);

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
