//! Acceptance for `story:crate-rename`: the workspace's packages and targets carry the engineering
//! protocols names. The engineering crate is `b10x-canon-engineering`, with the library
//! `canon_engineering` and the binary `canon-engineering`; the docs generator is
//! `canon-engineering-docs`. The previous package names, `b10x-els` and `els-docs`, and the previous
//! binary name `els` are gone.
//!
//! The names are read at run time from `cargo metadata --no-deps --format-version 1`, run as a child
//! process in this crate's directory, so the test checks the manifests of the tree it runs in.

use std::path::PathBuf;
use std::process::Command;

use serde::Deserialize;

const ENGINEERING_PACKAGE: &str = "b10x-canon-engineering";
const ENGINEERING_LIBRARY: &str = "canon_engineering";
const ENGINEERING_BINARY: &str = "canon-engineering";
const DOCS_PACKAGE: &str = "canon-engineering-docs";
const RETIRED_PACKAGES: [&str; 2] = ["b10x-els", "els-docs"];
const RETIRED_BINARIES: [&str; 2] = ["els", "els-docs"];

#[derive(Deserialize)]
struct Metadata {
    packages: Vec<Package>,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    targets: Vec<Target>,
}

#[derive(Deserialize)]
struct Target {
    name: String,
    kind: Vec<String>,
}

impl Target {
    fn is(&self, kind: &str) -> bool {
        self.kind.iter().any(|each| each == kind)
    }
}

/// The workspace's own packages, as `cargo metadata --no-deps` reports them for this tree.
fn workspace_packages() -> Vec<Package> {
    let manifest_dir = PathBuf::from(
        std::env::var_os("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR is unset: run this test through cargo"),
    );
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let output = Command::new(cargo)
        .current_dir(&manifest_dir)
        .args([
            "metadata",
            "--no-deps",
            "--format-version",
            "1",
            "--offline",
        ])
        .output()
        .expect("cargo runs");
    assert!(
        output.status.success(),
        "cargo metadata failed in {}: {}",
        manifest_dir.display(),
        String::from_utf8_lossy(&output.stderr)
    );
    let metadata: Metadata =
        serde_json::from_slice(&output.stdout).expect("cargo metadata prints format-version 1");
    metadata.packages
}

fn package_names(packages: &[Package]) -> Vec<&str> {
    packages
        .iter()
        .map(|package| package.name.as_str())
        .collect()
}

#[test]
fn workspace_lists_the_canon_engineering_packages() {
    let packages = workspace_packages();
    let names = package_names(&packages);
    for expected in [ENGINEERING_PACKAGE, DOCS_PACKAGE] {
        assert!(
            names.contains(&expected),
            "package {expected} is not in the workspace; packages: {names:?}"
        );
    }
}

#[test]
fn workspace_lists_no_retired_package() {
    let packages = workspace_packages();
    let names = package_names(&packages);
    let retired: Vec<&str> = RETIRED_PACKAGES
        .into_iter()
        .filter(|name| names.contains(name))
        .collect();
    assert!(
        retired.is_empty(),
        "retired package names are still in the workspace: {retired:?}; packages: {names:?}"
    );
}

#[test]
fn engineering_package_builds_the_canon_engineering_binary() {
    let packages = workspace_packages();
    let package = packages
        .iter()
        .find(|package| package.name == ENGINEERING_PACKAGE)
        .unwrap_or_else(|| {
            panic!(
                "package {ENGINEERING_PACKAGE} is not in the workspace; packages: {:?}",
                package_names(&packages)
            )
        });
    let binaries: Vec<&str> = package
        .targets
        .iter()
        .filter(|target| target.is("bin"))
        .map(|target| target.name.as_str())
        .collect();
    assert_eq!(
        binaries,
        [ENGINEERING_BINARY],
        "package {ENGINEERING_PACKAGE} must build exactly the binary {ENGINEERING_BINARY}"
    );
}

#[test]
fn engineering_package_library_is_canon_engineering() {
    let packages = workspace_packages();
    let package = packages
        .iter()
        .find(|package| package.name == ENGINEERING_PACKAGE)
        .unwrap_or_else(|| {
            panic!(
                "package {ENGINEERING_PACKAGE} is not in the workspace; packages: {:?}",
                package_names(&packages)
            )
        });
    let libraries: Vec<&str> = package
        .targets
        .iter()
        .filter(|target| target.is("lib"))
        .map(|target| target.name.as_str())
        .collect();
    assert_eq!(
        libraries,
        [ENGINEERING_LIBRARY],
        "package {ENGINEERING_PACKAGE} must have exactly the library {ENGINEERING_LIBRARY}"
    );
}

#[test]
fn no_package_builds_a_retired_binary() {
    let packages = workspace_packages();
    let retired: Vec<String> = packages
        .iter()
        .flat_map(|package| {
            package
                .targets
                .iter()
                .filter(|target| {
                    target.is("bin") && RETIRED_BINARIES.contains(&target.name.as_str())
                })
                .map(move |target| format!("{} (package {})", target.name, package.name))
        })
        .collect();
    assert!(
        retired.is_empty(),
        "retired binary names are still built: {retired:?}"
    );
}
