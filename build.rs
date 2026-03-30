use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src/images");

    let src = fs::read_to_string("src/images").expect("failed to read src/images");

    let images: Vec<&str> = src
        .split("\nnewline\n")
        .filter(|s| !s.is_empty())
        .collect();

    let mut out = String::from("pub static IMAGES: &[&str] = &[\n");
    for img in &images {
        // escape backslashes and quotes, then wrap in raw string delimiters
        out.push_str(&format!("    r#\"{}\"#,\n", img));
    }
    out.push_str("];\n");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    fs::write(Path::new(&out_dir).join("images.rs"), out)
        .expect("failed to write generated images.rs");
}
