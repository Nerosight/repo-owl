//! Embeds every `assets/logos/<slug>.png` into the binary, so adding a
//! language logo is just dropping in a PNG.

use std::{env, fs, path::Path};

fn main() {
    let dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("assets/logos");
    println!("cargo:rerun-if-changed={}", dir.display());

    let mut entries: Vec<_> = fs::read_dir(&dir)
        .expect("assets/logos should exist")
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "png"))
        .collect();
    entries.sort();

    let mut code = String::from("pub static LOGOS: &[(&str, &[u8])] = &[\n");
    for path in entries {
        let slug = path.file_stem().unwrap().to_str().unwrap();
        code += &format!("    ({slug:?}, include_bytes!({:?})),\n", path.display().to_string());
    }
    code += "];\n";

    let out = Path::new(&env::var("OUT_DIR").unwrap()).join("logos.rs");
    fs::write(out, code).unwrap();
}
