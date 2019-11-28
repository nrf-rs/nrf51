use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        File::create(out.join("device.x"))
            .unwrap()
            .write_all(include_bytes!("device.x"))
            .unwrap();
        println!("cargo:rustc-link-search={}", out.display());
        println!("cargo:rerun-if-changed=device.x");
    }

    if let Some((flash, mem)) = memory_sizes() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

        let mut file = File::create(out.join("memory.x")).unwrap();

        write!(file, r#"MEMORY
{{
FLASH : ORIGIN = 0x00000000, LENGTH = {}
RAM : ORIGIN = 0x20000000, LENGTH = {}
}}
"#, flash, mem).unwrap();

        println!("cargo:rustc-link-search={}", out.display());
    }

    println!("cargo:rerun-if-changed=build.rs");
}

fn memory_sizes() -> Option<(&'static str, &'static str)> {
    let aa = env::var_os("CARGO_FEATURE_MEMORY_AA").is_some();
    let ab = env::var_os("CARGO_FEATURE_MEMORY_AB").is_some();
    let ac = env::var_os("CARGO_FEATURE_MEMORY_AC").is_some();

    match (aa, ab, ac) {
        (false, false, false) => None,
        (true, false, false) => Some(("256K", "16K")),
        (false, true, false) => Some(("128K", "16K")),
        (false, false, true) => Some(("256K", "32K")),
        _ => panic!("Multiple memory configuration features specified"),
    }
}
