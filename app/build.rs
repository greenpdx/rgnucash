use std::env;
use std::path::PathBuf;

fn main() {
    // Load .env file from parent directory if present
    let _ = dotenvy::from_filename("../.env");
    let _ = dotenvy::dotenv();

    // Get library path from environment or use default
    let lib_path = PathBuf::from(
        env::var("GNUCASH_LIB_PATH")
            .unwrap_or_else(|_| "/usr/lib/aarch64-linux-gnu/gnucash".to_string()),
    );

    // Add rpath so the library can be found at runtime
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_path.display());
}
