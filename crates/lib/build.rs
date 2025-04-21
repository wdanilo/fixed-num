#![deny(unfulfilled_lint_expectations)]

fn main() {
    println!("cargo:rustc-check-cfg=cfg(nightly)");
    println!("cargo:rustc-check-cfg=cfg(inherit_overflow_checks)");
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());
    let output = std::process::Command::new(rustc)
        .arg("--version")
        .output()
        .expect("failed to run rustc");

    let version = String::from_utf8(output.stdout).unwrap_or_default();
    if version.contains("nightly") {
        println!("cargo:rustc-cfg=nightly");
    }

    if ::std::panic::catch_unwind(|| {
        #[expect(arithmetic_overflow)]
        let _ = 255_u8 + 1;
    }).is_err() {
        println!("cargo:rustc-cfg=inherit_overflow_checks");
    }
}
