//! Acceptance for `story:protocol-registry`: the crate ships its built-in protocols as data. A built-in is
//! every file at `protocols/<name>/<major>.yaml`, embedded in the crate when it is built;
//! `registry::list` names them, `registry::get` returns one as released together with Canon's
//! validated `protocol/1` model of it, and the `canon-engineering` binary lists and shows them.
//!
//! The expected set is not written down here: the test walks `protocols/` in the repository tree at
//! run time, so a protocol file added without a Rust edit is expected in the registry too.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use canon_engineering::registry;

/// The repository root, read at run time.
fn repository_root() -> PathBuf {
    let manifest = std::env::var_os("CARGO_MANIFEST_DIR").expect("cargo sets CARGO_MANIFEST_DIR");
    Path::new(&manifest).join("../..")
}

/// Every `protocols/<name>/<major>.yaml` in the repository tree, as (name, major). Files directly
/// under `protocols/`, such as `protocols/vocabulary.yaml`, are not protocols and are skipped.
fn builtins_on_disk(root: &Path) -> BTreeSet<(String, u32)> {
    let protocols = root.join("protocols");
    let mut found = BTreeSet::new();
    let entries = std::fs::read_dir(&protocols)
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
            .unwrap_or_else(|name| panic!("protocol directory name is not UTF-8: {name:?}"));
        let files =
            std::fs::read_dir(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        for file in files {
            let file = file.unwrap_or_else(|error| panic!("{}: {error}", path.display()));
            let file_name = file.file_name();
            let Some(file_name) = file_name.to_str() else {
                continue;
            };
            let Some(stem) = file_name.strip_suffix(".yaml") else {
                continue;
            };
            if stem.is_empty() || !stem.bytes().all(|byte| byte.is_ascii_digit()) {
                continue;
            }
            let major: u32 = stem
                .parse()
                .unwrap_or_else(|error| panic!("{}: {error}", file.path().display()));
            found.insert((name.clone(), major));
        }
    }
    found
}

/// The path of the built-in `name@major` in the repository tree.
fn protocol_path(root: &Path, name: &str, major: u32) -> PathBuf {
    root.join("protocols")
        .join(name)
        .join(format!("{major}.yaml"))
}

/// Runs the `canon-engineering` binary with `args`.
fn canon_engineering(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_canon-engineering"))
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("canon-engineering {}: {error}", args.join(" ")))
}

#[test]
fn registry_lists_fetches_and_validates_every_builtin() {
    let root = repository_root();
    let expected = builtins_on_disk(&root);

    // 1. `list` returns exactly the protocol files in the tree, including both shipped protocols,
    //    once each, and not the vocabulary.
    assert!(
        expected.contains(&("software-change".to_owned(), 1)),
        "the tree has protocols/software-change/1.yaml: {expected:?}"
    );
    assert!(
        expected.contains(&("incident-response".to_owned(), 1)),
        "the tree has protocols/incident-response/1.yaml: {expected:?}"
    );
    let listed: Vec<(String, u32)> = registry::list()
        .into_iter()
        .map(|(name, major)| (name.to_owned(), major))
        .collect();
    let listed_set: BTreeSet<(String, u32)> = listed.iter().cloned().collect();
    assert_eq!(
        listed.len(),
        listed_set.len(),
        "registry::list() names a built-in more than once: {listed:?}"
    );
    assert_eq!(
        listed_set, expected,
        "registry::list() differs from the protocols/<name>/<major>.yaml files in the tree"
    );
    assert!(
        !listed.iter().any(|(name, _)| name == "vocabulary"),
        "registry::list() lists protocols/vocabulary.yaml: {listed:?}"
    );

    // 2. `get` returns each built-in byte for byte as released, with Canon's validated model of it.
    for (name, major) in &expected {
        let path = protocol_path(&root, name, *major);
        let on_disk =
            std::fs::read(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        let builtin = registry::get(name, *major)
            .unwrap_or_else(|error| panic!("registry::get({name:?}, {major}): {error}"));
        assert_eq!(
            builtin.yaml.as_bytes(),
            on_disk.as_slice(),
            "registry::get({name:?}, {major}) differs from {}",
            path.display()
        );
        let text = String::from_utf8(on_disk)
            .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        let parsed = b10x_canon::model::parse(&text)
            .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        assert_eq!(
            builtin.model,
            parsed,
            "registry::get({name:?}, {major}) model is not Canon's parse of {}",
            path.display()
        );
        if let Err(problems) = b10x_canon::validate::validate(&builtin.model) {
            panic!("registry::get({name:?}, {major}) model fails Canon validation: {problems:?}");
        }
    }

    // 3. `get` refuses an unknown name and an unknown major, naming what was asked for.
    let unknown_name = registry::get("no-such-protocol", 1)
        .expect_err("registry::get(\"no-such-protocol\", 1) is refused");
    assert!(
        unknown_name.to_string().contains("no-such-protocol"),
        "the refusal names no-such-protocol: {unknown_name}"
    );
    let unknown_major = registry::get("software-change", 99)
        .expect_err("registry::get(\"software-change\", 99) is refused");
    assert!(
        unknown_major.to_string().contains("software-change@99"),
        "the refusal names software-change@99: {unknown_major}"
    );

    // 4. The `canon-engineering` binary lists and shows the same built-ins.
    let list = canon_engineering(&["protocols", "list"]);
    assert!(
        list.status.success(),
        "canon-engineering protocols list exits 0: {list:?}"
    );
    let stdout =
        String::from_utf8(list.stdout).expect("canon-engineering protocols list prints UTF-8");
    let mut printed: Vec<&str> = stdout.lines().collect();
    printed.sort_unstable();
    let mut wanted: Vec<String> = expected
        .iter()
        .map(|(name, major)| format!("{name}@{major}"))
        .collect();
    wanted.sort_unstable();
    assert_eq!(
        printed, wanted,
        "canon-engineering protocols list prints one <name>@<major> line per built-in"
    );

    let show = canon_engineering(&["protocols", "show", "software-change@1"]);
    assert!(
        show.status.success(),
        "canon-engineering protocols show software-change@1 exits 0: {show:?}"
    );
    let path = protocol_path(&root, "software-change", 1);
    let on_disk =
        std::fs::read(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    assert_eq!(
        show.stdout,
        on_disk,
        "canon-engineering protocols show software-change@1 prints {} byte for byte",
        path.display()
    );

    let refused = canon_engineering(&["protocols", "show", "no-such-protocol@1"]);
    assert!(
        !refused.status.success(),
        "canon-engineering protocols show no-such-protocol@1 exits non-zero: {refused:?}"
    );
    let stderr = String::from_utf8_lossy(&refused.stderr);
    assert!(
        stderr.contains("no-such-protocol@1"),
        "canon-engineering protocols show no-such-protocol@1 names it: {stderr}"
    );
}
