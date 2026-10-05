//! Adversary pass 1 on `story:crate-rename`: what `crate_names.rs` does not pin. The binary's own
//! clap surface (`--help`, a usage error) must name `canon-engineering` and never the project's
//! earlier name, the library's one human-facing error names no project, and the acceptance's
//! `grep -rniw els` over `README.md`, `AGENTS.md`, `docs/`, `website/` and `crates/` returns only
//! hits of the kinds the coordinator scoped out (the docs-system product id, the managed-worktree
//! repository id, fixture format ids, protocol ids, story ids, the retired names this suite asserts
//! are gone, the historic design proposal and the dated showcase snapshot).
//!
//! `story:repository-rename` tightened it: the repository is `beyond10x/engineering-protocols` and
//! its site is served under `/engineering-protocols/`, so the old repository URL, the old `/els/`
//! route and the old site ids are no longer an allowed kind, and no file this repository ships
//! names them.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// Runs this tree's `canon-engineering` binary with `args`.
fn canon_engineering(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_canon-engineering"))
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("canon-engineering {}: {error}", args.join(" ")))
}

/// Whether `text` holds `word` as a whole word, ignoring ASCII case (`grep -iw` semantics: a word
/// character is a letter, a digit or `_`).
fn has_word(text: &str, word: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    let word = word.to_ascii_lowercase();
    let is_word = |c: char| c.is_ascii_alphanumeric() || c == '_';
    let mut from = 0;
    while let Some(found) = lower[from..].find(&word) {
        let start = from + found;
        let end = start + word.len();
        let before = lower[..start].chars().next_back();
        let after = lower[end..].chars().next();
        if !before.is_some_and(is_word) && !after.is_some_and(is_word) {
            return true;
        }
        from = start + 1;
    }
    false
}

#[test]
fn has_word_matches_grep_iw() {
    assert!(has_word("the ELS crate", "els"));
    assert!(has_word("els", "els"));
    assert!(has_word("/els/", "els"));
    assert!(has_word("els-docs", "els"));
    assert!(!has_word("models", "els"));
    assert!(!has_word("els_x", "els"));
    assert!(!has_word("labels", "els"));
}

#[test]
fn help_names_the_renamed_command_and_not_the_old_project() {
    let out = canon_engineering(&["--help"]);
    assert_eq!(out.status.code(), Some(0), "--help exits 0: {out:?}");
    let stdout = String::from_utf8(out.stdout).expect("UTF-8 help");
    assert!(
        stdout.contains("Usage: canon-engineering <COMMAND>"),
        "--help names the binary canon-engineering:\n{stdout}"
    );
    assert!(
        !has_word(&stdout, "els"),
        "--help still names the old project:\n{stdout}"
    );

    for args in [
        &["protocols", "--help"][..],
        &["protocols", "show", "--help"][..],
    ] {
        let out = canon_engineering(args);
        assert_eq!(out.status.code(), Some(0), "{args:?} exits 0: {out:?}");
        let stdout = String::from_utf8(out.stdout).expect("UTF-8 help");
        assert!(
            stdout.contains("Usage: canon-engineering protocols"),
            "{args:?} names the binary canon-engineering:\n{stdout}"
        );
        assert!(
            !has_word(&stdout, "els"),
            "{args:?} still names the old project:\n{stdout}"
        );
    }
}

#[test]
fn a_usage_error_names_the_renamed_command() {
    let out = canon_engineering(&["no-such-command"]);
    assert_eq!(out.status.code(), Some(2), "usage error exits 2: {out:?}");
    let stderr = String::from_utf8(out.stderr).expect("UTF-8 stderr");
    assert!(
        stderr.contains("Usage: canon-engineering"),
        "the usage error names canon-engineering:\n{stderr}"
    );
    assert!(
        !has_word(&stderr, "els"),
        "the usage error still names the old project:\n{stderr}"
    );
}

#[test]
fn the_unknown_term_error_names_no_project() {
    let error = canon_engineering::vocabulary::lookup("no.such.term").expect_err("unknown term");
    let text = error.to_string();
    assert_eq!(
        text,
        "`no.such.term` is not a term of the engineering vocabulary"
    );
    assert!(!has_word(&text, "els"));
}

/// The repository root, read at run time.
fn root() -> PathBuf {
    PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("run through cargo")).join("../..")
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name == "node_modules" || name == "build" || name == ".docusaurus" || name == "target" {
            continue;
        }
        if path.is_dir() {
            walk(&path, out);
        } else {
            out.push(path);
        }
    }
}

/// The acceptance's third item, run: every whole-word `els` hit is one of the scoped-out kinds.
#[test]
fn every_remaining_els_hit_is_a_scoped_out_kind() {
    let root = root();
    let mut files = vec![root.join("README.md"), root.join("AGENTS.md")];
    for dir in ["docs", "website", "crates"] {
        walk(&root.join(dir), &mut files);
    }
    let allowed_file = |relative: &str| {
        relative.starts_with("docs/design/")
            || relative.starts_with("website/static/showcase/2026-10-04/")
            || relative == "crates/canon-engineering/tests/crate_names.rs"
            || relative == "crates/canon-engineering/tests/adversary_rename.rs"
            || relative == "crates/canon-engineering-docs/tests/adversary_rename.rs"
    };
    let allowed_line = |line: &str| {
        [
            "els-fixture/",
            "story:els-",
            "product: 'els'",
            "--repo els",
            "els/incident.response@1",
            "`crates/els/src/vocabulary.rs` at `13d180f`",
        ]
        .iter()
        .any(|kind| line.contains(kind))
    };
    let mut unexplained = Vec::new();
    for file in files {
        let relative = file
            .strip_prefix(&root)
            .expect("under root")
            .to_string_lossy()
            .replace('\\', "/");
        if allowed_file(&relative) {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&file) else {
            continue;
        };
        for (number, line) in text.lines().enumerate() {
            if has_word(line, "els") && !allowed_line(line) {
                unexplained.push(format!("{relative}:{}: {line}", number + 1));
            }
        }
    }
    assert!(
        unexplained.is_empty(),
        "whole-word `els` hits that name the project:\n{}",
        unexplained.join("\n")
    );
}

/// Whether `line` names the repository's old GitHub path, `beyond10x/els`, as a whole path segment.
fn names_old_repository(line: &str) -> bool {
    let needle = "beyond10x/els";
    let mut from = 0;
    while let Some(found) = line[from..].find(needle) {
        let end = from + found + needle.len();
        let after = line[end..].chars().next();
        if !after.is_some_and(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
            return true;
        }
        from = end;
    }
    false
}

/// Whether `line` names the site's old route, `/els/`. The one historic source path this repository
/// cites by commit (`crates/els/src/vocabulary.rs` at `13d180f`) is not a route.
fn names_old_route(line: &str) -> bool {
    line.contains("/els/") && !line.contains("`crates/els/src/vocabulary.rs` at `13d180f`")
}

#[test]
fn the_old_repository_and_route_matchers_match() {
    assert!(names_old_repository("https://github.com/beyond10x/els"));
    assert!(names_old_repository(
        "https://github.com/beyond10x/els/blob/main/x"
    ));
    assert!(names_old_repository("github.repository == 'beyond10x/els'"));
    assert!(!names_old_repository(
        "https://github.com/beyond10x/engineering-protocols"
    ));
    assert!(!names_old_repository("beyond10x/elsewhere"));
    assert!(!names_old_repository("beyond10x/els-docs"));
    assert!(names_old_route("  baseUrl: '/els/',"));
    assert!(names_old_route("src=\"/els/showcase/2026-10-04/\""));
    assert!(!names_old_route("baseUrl: '/engineering-protocols/'"));
    assert!(!names_old_route(
        "/// The 35 terms of `crates/els/src/vocabulary.rs` at `13d180f`, in its order:"
    ));
}

/// No file this repository ships names the old GitHub repository or the old site route. Unlike the
/// whole-word scan above this also reads `Cargo.toml`, the Taskfile, the workflows and `protocols/`;
/// it skips only history: the design proposal, the dated showcase snapshot, the changelog and this
/// suite's own files.
#[test]
fn no_shipped_file_names_the_old_repository_or_route() {
    let root = root();
    let mut files = vec![
        root.join("README.md"),
        root.join("AGENTS.md"),
        root.join("Cargo.toml"),
        root.join("Taskfile.yml"),
    ];
    for dir in [".github", "docs", "website", "crates", "protocols"] {
        walk(&root.join(dir), &mut files);
    }
    let history = |relative: &str| {
        relative.starts_with("docs/design/")
            || relative.starts_with("website/static/showcase/2026-10-04/")
            || relative == "crates/canon-engineering/tests/adversary_rename.rs"
            || relative == "crates/canon-engineering-docs/tests/adversary_rename.rs"
    };
    let mut stale = Vec::new();
    for file in files {
        let relative = file
            .strip_prefix(&root)
            .expect("under root")
            .to_string_lossy()
            .replace('\\', "/");
        if history(&relative) {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&file) else {
            continue;
        };
        for (number, line) in text.lines().enumerate() {
            if names_old_repository(line) || names_old_route(line) {
                stale.push(format!("{relative}:{}: {line}", number + 1));
            }
        }
    }
    assert!(
        stale.is_empty(),
        "lines still naming beyond10x/els or the /els/ route:\n{}",
        stale.join("\n")
    );
}

/// The places that bind the repository's identity name the new one: the package metadata, the site
/// publisher's repository guard, project id and route, the Docusaurus site, and the release path.
#[test]
fn the_repository_identity_names_engineering_protocols() {
    let root = root();
    let read = |relative: &str| {
        std::fs::read_to_string(root.join(relative))
            .unwrap_or_else(|error| panic!("{relative}: {error}"))
    };
    let expected = [
        (
            "Cargo.toml",
            "repository = \"https://github.com/beyond10x/engineering-protocols\"",
        ),
        (
            ".github/workflows/b10x-docs-site.yml",
            "github.repository == 'beyond10x/engineering-protocols'",
        ),
        (
            ".github/workflows/b10x-docs-site.yml",
            "repository: engineering-protocols\n",
        ),
        (
            ".github/workflows/b10x-docs-site.yml",
            "route_base: /engineering-protocols/\n",
        ),
        (
            "website/docusaurus.config.ts",
            "baseUrl: '/engineering-protocols/',",
        ),
        (
            "website/docusaurus.config.ts",
            "projectName: 'engineering-protocols',",
        ),
        (
            "website/docusaurus.config.ts",
            "activeBaseRegex: '^/engineering-protocols/docs/$'",
        ),
        (
            "website/docusaurus.config.ts",
            "https://github.com/beyond10x/engineering-protocols/tree/main/website/",
        ),
        (
            "website/docs/showcase.mdx",
            "src=\"/engineering-protocols/showcase/2026-10-04/\"",
        ),
        ("AGENTS.md", "# AGENTS.md — engineering-protocols\n"),
        (
            "AGENTS.md",
            "/repos/beyond10x/engineering-protocols/releases",
        ),
    ];
    let missing: Vec<String> = expected
        .iter()
        .filter(|(file, text)| !read(file).contains(text))
        .map(|(file, text)| format!("{file}: {text:?}"))
        .collect();
    assert!(
        missing.is_empty(),
        "the repository identity is not engineering-protocols in:\n{}",
        missing.join("\n")
    );
}
