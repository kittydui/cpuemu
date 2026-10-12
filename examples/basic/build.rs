use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
};

// Builds every asm/*.asm (NASM) and c/*.c (clang) into a flat binary loaded at 0x1000, and
// generates `programs.rs`: a `PROGRAMS` table and one #[test] per file with expectations.
// Expected results come from comment lines like `; expect rax = 30` or `// expect rax = 30`.
fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo::rerun-if-changed=asm");
    println!("cargo::rerun-if-changed=c");

    // Without a tool, skip its programs instead of failing the whole workspace build.
    let mut sources = Vec::new();
    if tool_exists("nasm", "-v") {
        sources.extend(files("asm", "asm"));
    } else {
        println!("cargo::warning=nasm not found on PATH, skipping the asm programs");
    }
    if tool_exists("clang", "--version") {
        sources.extend(files("c", "c"));
    } else {
        println!("cargo::warning=clang not found on PATH, skipping the C programs");
    }
    sources.sort();

    let mut programs = String::new();
    let mut tests = String::new();
    for (index, source) in sources.iter().enumerate() {
        let file_name = source.file_name().unwrap().to_str().unwrap();
        let binary = out.join(format!("{file_name}.bin"));
        let status = match source.extension().unwrap().to_str().unwrap() {
            "asm" => assemble(source, &binary),
            _ => compile_c(source, &binary),
        };
        assert!(status.success(), "failed to build {}", source.display());

        let binary = format!("/{file_name}.bin");
        let expect = expectations(source);
        programs += &format!(
            "    Program {{ name: {file_name:?}, code: include_bytes!(concat!(env!(\"OUT_DIR\"), {binary:?})), expect: &{expect:?} }},\n"
        );
        // Programs without expect lines are only run by `cargo run`, not tested.
        if !expect.is_empty() {
            tests += &format!(
                "    #[test]\n    fn {}() {{\n        super::assert_program(&super::PROGRAMS[{index}]);\n    }}\n",
                test_name(file_name)
            );
        }
    }

    let generated = format!(
        "static PROGRAMS: &[Program] = &[\n{programs}];\n\n#[cfg(test)]\nmod programs {{\n{tests}}}\n"
    );
    fs::write(out.join("programs.rs"), generated).unwrap();
}

fn assemble(source: &Path, binary: &Path) -> ExitStatus {
    Command::new("nasm")
        .args(["-f", "bin"])
        .arg(source)
        .arg("-o")
        .arg(binary)
        .status()
        .unwrap()
}

// Freestanding code with no libc, compiled for the System V ABI (the Linux target, so
// arguments arrive in rdi, rsi, ...) and linked into a flat binary starting at 0x1000.
// -O0 keeps to the simple instructions the emulator has so far; higher levels use things
// like imul and shr.
fn compile_c(source: &Path, binary: &Path) -> ExitStatus {
    Command::new("clang")
        .args([
            "--target=x86_64-unknown-linux-gnu",
            "-O0",
            "-ffreestanding",
            "-nostdlib",
            "-static",
            "-fno-pic",
            "-fno-stack-protector",
            "-fcf-protection=none",
            "-fno-asynchronous-unwind-tables",
            "-mno-sse",
            "-fuse-ld=lld",
            "-Wl,--oformat=binary",
            "-Wl,--image-base=0x1000",
            "-Wl,-Ttext=0x1000",
            "-Wl,-e,_start",
            "-o",
        ])
        .arg(binary)
        .arg(source)
        .status()
        .unwrap()
}

fn files(dir: &str, extension: &str) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };
    entries
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == extension))
        .collect()
}

fn tool_exists(tool: &str, version_flag: &str) -> bool {
    Command::new(tool).arg(version_flag).output().is_ok()
}

// `; expect rax = 30` or `// expect rax = 30` becomes ("rax", "30").
fn expectations(source: &Path) -> Vec<(String, String)> {
    fs::read_to_string(source)
        .unwrap()
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            let comment = line.strip_prefix(';').or_else(|| line.strip_prefix("//"))?;
            comment.trim().strip_prefix("expect ")
        })
        .map(|rest| {
            let (key, value) = rest.split_once('=').unwrap_or_else(|| {
                panic!("{}: expected `expect <name> = <value>`, got `{rest}`", source.display())
            });
            (key.trim().to_string(), value.trim().to_string())
        })
        .collect()
}

// `add.asm` becomes `add_asm`, `sum.c` becomes `sum_c`. The extension suffix keeps names
// unique between the two folders and stops them colliding with Rust keywords.
fn test_name(file_name: &str) -> String {
    let name: String = file_name
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else { '_' })
        .collect();
    if name.starts_with(|c: char| c.is_ascii_digit()) {
        format!("_{name}")
    } else {
        name
    }
}
