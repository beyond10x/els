//! Adversary pass 1 on `story:protocol-registry` (wave 2026-10-04-w13), at 99d62db plus the
//! uncommitted phase 2.
//!
//! Its shared-target case was declined; the hazard it found is documented in `AGENTS.md` § Work.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// The repository root, read at run time.
fn repository_root() -> PathBuf {
    let manifest = std::env::var_os("CARGO_MANIFEST_DIR").expect("cargo sets CARGO_MANIFEST_DIR");
    Path::new(&manifest).join("../..")
}

/// Runs this tree's `canon-engineering` binary with `args`.
fn canon_engineering(args: &[&str]) -> Output {
    run(Path::new(env!("CARGO_BIN_EXE_canon-engineering")), args)
}

fn run(binary: &Path, args: &[&str]) -> Output {
    Command::new(binary)
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("{} {}: {error}", binary.display(), args.join(" ")))
}

/// Every `protocols/<name>/<major>.yaml` in the tree, as (name, major).
fn builtins_on_disk(root: &Path) -> BTreeSet<(String, u32)> {
    let mut found = BTreeSet::new();
    for entry in std::fs::read_dir(root.join("protocols")).expect("protocols/ reads") {
        let entry = entry.expect("protocols/ entry");
        if !entry.file_type().expect("file type").is_dir() {
            continue;
        }
        let name = entry.file_name().into_string().expect("UTF-8 name");
        for file in std::fs::read_dir(entry.path()).expect("protocol dir reads") {
            let file_name = file.expect("entry").file_name();
            let Some(stem) = file_name.to_str().and_then(|n| n.strip_suffix(".yaml")) else {
                continue;
            };
            found.insert((name.clone(), stem.parse().expect("<major>.yaml")));
        }
    }
    found
}

/// A malformed `show` argument is refused on stderr with nothing on stdout. It exits 2, clap's
/// usage code, because clap's value parser applies build.rs's `<name>@<major>` rule to it; a
/// well-formed argument naming no built-in stays a refusal, exit 1.
#[test]
fn a_malformed_show_argument_is_refused_on_stderr() {
    for argument in [
        "software-change",
        "software-change@",
        "software-change@01",
        "software-change@+1",
        "software-change@-1",
        "software-change@4294967296",
        "software-change@one",
        "",
        "@1",
        "software-change@0",
        "Software-Change@1",
    ] {
        let out = canon_engineering(&["protocols", "show", argument]);
        assert_eq!(
            out.status.code(),
            Some(2),
            "canon-engineering protocols show {argument:?} exits 2: {out:?}"
        );
        assert!(
            out.stdout.is_empty(),
            "canon-engineering protocols show {argument:?} prints nothing on stdout: {out:?}"
        );
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            stderr.contains(&format!("'{argument}'")),
            "canon-engineering protocols show {argument:?} names it on stderr: {stderr}"
        );
    }
    for argument in ["no-such-protocol@1", "software-change@99"] {
        let out = canon_engineering(&["protocols", "show", argument]);
        assert_eq!(
            out.status.code(),
            Some(1),
            "canon-engineering protocols show {argument:?} is a refusal, exit 1: {out:?}"
        );
        assert!(out.stdout.is_empty(), "{out:?}");
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            stderr.starts_with("canon-engineering: ") && stderr.contains(argument),
            "canon-engineering protocols show {argument:?} names it on stderr: {stderr}"
        );
    }
}

/// A command line clap cannot parse is a usage error, exit 2, distinct from a refusal.
#[test]
fn a_usage_error_exits_2() {
    for args in [
        &[][..],
        &["protocols"][..],
        &["protocols", "show"][..],
        &["protocols", "list", "extra"][..],
        &["protocols", "remove", "software-change@1"][..],
    ] {
        let out = canon_engineering(args);
        assert_eq!(
            out.status.code(),
            Some(2),
            "canon-engineering {args:?} exits 2: {out:?}"
        );
        assert!(
            out.stdout.is_empty(),
            "canon-engineering {args:?} prints no stdout: {out:?}"
        );
    }
}

/// `list` prints exactly one sorted `<name>@<major>` line per file in the tree, each ending in a
/// newline, and `show` prints every one of them byte for byte (the acceptance shows only one).
#[test]
fn list_and_show_print_every_builtin_exactly() {
    let root = repository_root();
    let expected = builtins_on_disk(&root);
    let list = canon_engineering(&["protocols", "list"]);
    assert_eq!(list.status.code(), Some(0), "{list:?}");
    let wanted: String = expected
        .iter()
        .map(|(name, major)| format!("{name}@{major}\n"))
        .collect();
    assert_eq!(String::from_utf8(list.stdout).expect("UTF-8"), wanted);
    assert!(list.stderr.is_empty(), "{:?}", list.stderr);

    for (name, major) in &expected {
        let path = root.join(format!("protocols/{name}/{major}.yaml"));
        let on_disk = std::fs::read(&path).expect("protocol file reads");
        let show = canon_engineering(&["protocols", "show", &format!("{name}@{major}")]);
        assert_eq!(show.status.code(), Some(0), "{show:?}");
        assert_eq!(
            show.stdout,
            on_disk,
            "show {name}@{major} differs from {}",
            path.display()
        );
        assert!(show.stderr.is_empty(), "{:?}", show.stderr);
    }
}

// Declined: `a_checkout_sharing_a_target_dir_embeds_its_own_protocols` (see AGENTS.md § Work).
