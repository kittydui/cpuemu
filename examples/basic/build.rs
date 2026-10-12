use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

// Assembles every asm/*.asm with NASM and generates `programs.rs`: a `PROGRAMS` table and
// one #[test] per file. Expected results come from comment lines like `; expect rax = 30`.
fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo::rerun-if-changed=asm");

    let mut sources: Vec<PathBuf> = fs::read_dir("asm")
        .expect("missing asm/ folder")
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "asm"))
        .collect();
    sources.sort();

    // Without NASM, build with no programs instead of failing the whole workspace build.
    if Command::new("nasm").arg("-v").output().is_err() {
        println!("cargo::warning=nasm not found on PATH, skipping the asm programs");
        sources.clear();
    }

    let mut programs = String::new();
    let mut tests = String::new();
    for (index, source) in sources.iter().enumerate() {
        let name = source.file_stem().unwrap().to_str().unwrap();
        let status = Command::new("nasm")
            .args(["-f", "bin"])
            .arg(source)
            .arg("-o")
            .arg(out.join(format!("{name}.bin")))
            .status()
            .unwrap();
        assert!(status.success(), "nasm failed on {}", source.display());

        let binary = format!("/{name}.bin");
        let expect = expectations(source);
        programs += &format!(
            "    Program {{ name: {name:?}, code: include_bytes!(concat!(env!(\"OUT_DIR\"), {binary:?})), expect: &{expect:?} }},\n"
        );
        tests += &format!(
            "    #[test]\n    fn {}() {{\n        super::assert_program(&super::PROGRAMS[{index}]);\n    }}\n",
            test_name(name)
        );
    }

    let generated = format!(
        "static PROGRAMS: &[Program] = &[\n{programs}];\n\n#[cfg(test)]\nmod asm {{\n{tests}}}\n"
    );
    fs::write(out.join("programs.rs"), generated).unwrap();
}

// `; expect rax = 30` becomes ("rax", "30").
fn expectations(source: &Path) -> Vec<(String, String)> {
    fs::read_to_string(source)
        .unwrap()
        .lines()
        .filter_map(|line| line.trim().strip_prefix(';')?.trim().strip_prefix("expect "))
        .map(|rest| {
            let (key, value) = rest.split_once('=').unwrap_or_else(|| {
                panic!("{}: expected `; expect <name> = <value>`, got `{rest}`", source.display())
            });
            (key.trim().to_string(), value.trim().to_string())
        })
        .collect()
}

// File names become test function names, so make them valid identifiers.
fn test_name(name: &str) -> String {
    let name: String = name
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else { '_' })
        .collect();
    const KEYWORDS: &[&str] = &[
        "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum",
        "extern", "false", "fn", "for", "gen", "if", "impl", "in", "let", "loop", "match", "mod",
        "move", "mut", "pub", "ref", "return", "self", "static", "struct", "super", "trait",
        "true", "try", "type", "unsafe", "use", "where", "while", "yield",
    ];
    if name.starts_with(|c: char| c.is_ascii_digit()) {
        format!("_{name}")
    } else if KEYWORDS.contains(&name.as_str()) {
        format!("{name}_")
    } else {
        name
    }
}
