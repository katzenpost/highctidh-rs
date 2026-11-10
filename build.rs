//! Builds highctidh's CTIDH-1024 from the vendored copy in `highctidh/`,
//! codeberg.org/vula/highctidh v1.0.2026101300, with the flags its cgo
//! binding uses, in its portable C form.
//!
//! With the `external` feature or `HIGHCTIDH_LIB_DIR` set it builds nothing
//! and links a highctidh built elsewhere: the static library
//! `HIGHCTIDH_LIB_NAME` (default `highctidh1024`) from `HIGHCTIDH_LIB_DIR`,
//! or, with only the feature, whatever the final link provides.

fn main() {
    println!("cargo:rerun-if-changed=highctidh");
    println!("cargo:rerun-if-changed=bare-metal");
    println!("cargo:rerun-if-env-changed=HIGHCTIDH_LIB_DIR");
    println!("cargo:rerun-if-env-changed=HIGHCTIDH_LIB_NAME");
    if let Ok(lib_dir) = std::env::var("HIGHCTIDH_LIB_DIR") {
        let name = std::env::var("HIGHCTIDH_LIB_NAME").unwrap_or("highctidh1024".into());
        println!("cargo:rustc-link-search=native={lib_dir}");
        println!("cargo:rustc-link-lib=static={name}");
        return;
    }
    if std::env::var_os("CARGO_FEATURE_EXTERNAL").is_some() {
        return;
    }

    // Without an OS randombytes.c has no RNG to call; the final link must
    // provide highctidh_1024_randombytes, as the firmware's component does.
    let bare_metal = std::env::var("CARGO_CFG_TARGET_OS").is_ok_and(|os| os == "none");
    let dir = std::path::Path::new("highctidh/ctidh1024");
    let mut build = cc::Build::new();
    for entry in std::fs::read_dir(dir).expect("highctidh sources") {
        let path = entry.expect("highctidh source").path();
        if path.extension().is_some_and(|e| e == "c")
            && !(bare_metal && path.file_name().is_some_and(|n| n == "randombytes.c"))
        {
            build.file(&path);
        }
    }
    if bare_metal {
        // newlib has no <endian.h>.
        build.include("bare-metal");
    }
    build
        .include(dir)
        .define("BITS", "1024")
        .define("CGONUTS", None)
        .define("HIGHCTIDH_PORTABLE", "1")
        .define("GETRANDOM", None)
        .opt_level(2)
        .warnings(false)
        .compile("highctidh1024");
}
