use std::{env, path::PathBuf, process::Command};

fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    for name in ["nop_example"] {
        let source = format!("asm/{name}.asm");
        println!("cargo::rerun-if-changed={source}");
        let status = Command::new("nasm")
            .args(["-f", "bin", &source, "-o"])
            .arg(out.join(format!("{name}.bin")))
            .status()
            .expect("nasm not found on PATH");
        assert!(status.success(), "nasm failed on {source}");
    }
}
