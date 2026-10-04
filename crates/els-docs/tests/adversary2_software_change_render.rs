//! Adversary pass 2 on `story:software-change-protocol`: the generated page and graph of
//! `software.change/1` do not depend on the order its YAML is written in.
//!
//! The shipped protocol is generated with the `els-docs` binary twice, into two roots under
//! `CARGO_TARGET_TMPDIR`: once as written, once with every mapping and every sequence at every
//! depth in reverse order. Every generated file must be byte for byte the same. Nothing in the
//! repository is written.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("cargo sets CARGO_MANIFEST_DIR"))
        .join("../..")
}

/// One line and the lines indented below it.
struct Block {
    line: String,
    children: Vec<Block>,
}

fn indent(line: &str) -> usize {
    line.len() - line.trim_start().len()
}

/// The lines from `at` on, indented at least `depth`, as sibling blocks.
fn blocks(lines: &[&str], at: &mut usize, depth: usize) -> Vec<Block> {
    let mut siblings = Vec::new();
    while *at < lines.len() && indent(lines[*at]) >= depth {
        let line = lines[*at];
        let own = indent(line);
        *at += 1;
        // A sequence item's own keys sit two columns right of its dash.
        let below = if line.trim_start().starts_with("- ") {
            own + 2
        } else {
            own + 1
        };
        let children = blocks(lines, at, below);
        siblings.push(Block {
            line: line.to_owned(),
            children,
        });
    }
    siblings
}

/// Reverses every list of siblings, except the lines of a block scalar (`>-`, `|`).
fn reverse(blocks: &mut Vec<Block>) {
    blocks.reverse();
    for block in blocks {
        let scalar = block.line.ends_with(">-") || block.line.ends_with('|');
        if !scalar {
            reverse(&mut block.children);
        }
    }
}

fn write(blocks: &[Block], out: &mut String) {
    for block in blocks {
        out.push_str(&block.line);
        out.push('\n');
        write(&block.children, out);
    }
}

/// `text` with every mapping and sequence at every depth in reverse order. Blank lines are dropped;
/// the protocol has none inside a block scalar. A sequence item's first key stays on the dash line.
fn permuted(text: &str) -> String {
    let lines: Vec<&str> = text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    let mut at = 0;
    let mut tree = blocks(&lines, &mut at, 0);
    assert_eq!(at, lines.len(), "every line read");
    reverse(&mut tree);
    let mut out = String::new();
    write(&tree, &mut out);
    out
}

/// Every file under `dir`, keyed by its path relative to `base`.
fn files(base: &Path, dir: &Path, into: &mut BTreeMap<String, Vec<u8>>) {
    for entry in std::fs::read_dir(dir).expect("read_dir") {
        let path = entry.expect("entry").path();
        if path.is_dir() {
            files(base, &path, into);
        } else {
            let relative = path.strip_prefix(base).unwrap().display().to_string();
            into.insert(relative, std::fs::read(&path).expect("read"));
        }
    }
}

/// Generates `protocol` as `protocols/software-change/1.yaml` into a fresh root and returns every
/// file written under `website/`.
fn generated(name: &str, protocol: &str) -> BTreeMap<String, Vec<u8>> {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "adversary2-software-change-render-{}-{name}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("protocols/software-change")).expect("scratch root");
    std::fs::copy(
        repo_root().join("protocols/vocabulary.yaml"),
        root.join("protocols/vocabulary.yaml"),
    )
    .expect("copy the vocabulary");
    std::fs::write(root.join("protocols/software-change/1.yaml"), protocol).expect("protocol");
    let output = Command::new(env!("CARGO_BIN_EXE_els-docs"))
        .args(["generate", "--root"])
        .arg(&root)
        .output()
        .expect("els-docs runs");
    assert!(
        output.status.success(),
        "els-docs generate ({name}): {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut found = BTreeMap::new();
    files(&root.join("website"), &root.join("website"), &mut found);
    let _ = std::fs::remove_dir_all(&root);
    found
}

#[test]
fn the_page_and_graph_do_not_depend_on_the_order_the_yaml_is_written_in() {
    let text = std::fs::read_to_string(repo_root().join("protocols/software-change/1.yaml"))
        .expect("the shipped protocol");
    let reordered = permuted(&text);
    assert_ne!(reordered, text, "the permutation moves something");
    let original_lines: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();
    assert_ne!(
        reordered.lines().next(),
        original_lines.first().copied(),
        "the first top-level key moved"
    );

    let as_written = generated("as-written", &text);
    let reversed = generated("reversed", &reordered);
    assert!(
        as_written
            .keys()
            .any(|path| path.ends_with("protocols/software-change/1.mdx")),
        "the page is generated: {:?}",
        as_written.keys().collect::<Vec<_>>()
    );
    assert!(
        as_written
            .keys()
            .any(|path| path.ends_with("protocol-graphs/software-change-1.json")),
        "the graph is generated"
    );
    let differing: std::collections::BTreeSet<&String> = as_written
        .keys()
        .chain(reversed.keys())
        .filter(|path| as_written.get(*path) != reversed.get(*path))
        .collect();
    assert_eq!(differing, std::collections::BTreeSet::new());
}
