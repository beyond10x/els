//! Embeds every built-in protocol, `protocols/<name>/<major>.yaml`, in the crate. It writes
//! `$OUT_DIR/builtins.rs`: one `(name, major, include_str!(path))` entry per file, sorted by name
//! and major, which `src/registry.rs` includes. A protocol file added to `protocols/` is embedded on
//! the next build without a Rust edit; files directly under `protocols/`, such as
//! `protocols/vocabulary.yaml`, are not protocols and are skipped.
//!
//! The rule for names and majors is `src/builtin_name.rs`, which the `canon-engineering` binary
//! also applies to the `<name>@<major>` it is given: a directory name is lowercase letters, digits
//! and hyphens, and a file is `<major>.yaml` with `<major>` a positive decimal without leading
//! zeros. Anything else in a protocol directory that ends in `.yaml` fails the build rather than
//! being skipped.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

#[allow(dead_code)] // the build script uses the name and major rules, not `reference`
#[path = "src/builtin_name.rs"]
mod builtin_name;

fn main() {
    let manifest = std::env::var_os("CARGO_MANIFEST_DIR").expect("cargo sets CARGO_MANIFEST_DIR");
    let protocols = Path::new(&manifest).join("../../protocols");
    let protocols = protocols
        .canonicalize()
        .unwrap_or_else(|error| panic!("{}: {error}", protocols.display()));
    // Cargo scans a directory named here for any change below it, so an added protocol re-runs
    // this script.
    println!("cargo:rerun-if-changed={}", protocols.display());

    let mut builtins = builtins(&protocols);
    builtins.sort();

    let mut out = String::from("&[\n");
    for (name, major, path) in &builtins {
        let path = path
            .to_str()
            .unwrap_or_else(|| panic!("{}: path is not UTF-8", path.display()));
        writeln!(out, "    ({name:?}, {major}, include_str!({path:?})),")
            .expect("writing to a String");
    }
    out.push_str("]\n");

    let out_dir = std::env::var_os("OUT_DIR").expect("cargo sets OUT_DIR");
    let target = Path::new(&out_dir).join("builtins.rs");
    std::fs::write(&target, out).unwrap_or_else(|error| panic!("{}: {error}", target.display()));
}

/// Every `<name>/<major>.yaml` below `protocols`, as (name, major, path).
fn builtins(protocols: &Path) -> Vec<(String, u32, PathBuf)> {
    let mut found = Vec::new();
    let entries = std::fs::read_dir(protocols)
        .unwrap_or_else(|error| panic!("{}: {error}", protocols.display()));
    for entry in entries {
        let entry = entry.unwrap_or_else(|error| panic!("{}: {error}", protocols.display()));
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let name = entry
            .file_name()
            .into_string()
            .unwrap_or_else(|name| panic!("protocol directory {name:?} is not UTF-8"));
        assert!(
            builtin_name::is_name(&name),
            "protocol directory `{name}` must be lowercase letters, digits and hyphens"
        );
        let files =
            std::fs::read_dir(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        for file in files {
            let file = file.unwrap_or_else(|error| panic!("{}: {error}", path.display()));
            let file_name = file
                .file_name()
                .into_string()
                .unwrap_or_else(|name| panic!("protocol file {name:?} is not UTF-8"));
            let Some(stem) = file_name.strip_suffix(".yaml") else {
                continue;
            };
            let major = builtin_name::major(stem).unwrap_or_else(|| {
                panic!("protocols/{name}/{file_name}: a protocol file is named <major>.yaml")
            });
            found.push((name.clone(), major, file.path()));
        }
    }
    found
}
