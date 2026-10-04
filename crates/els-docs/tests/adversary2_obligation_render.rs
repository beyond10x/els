//! Adversary pass 2 on `story:incident-response-protocol`: the obligation rendering this unit
//! added to `els-docs` (the page's "Discharged when" column and the graph's claim → obligation
//! `requires` edges), driven through the `els-docs generate` binary against protocols other than
//! `incident.response/1`: several obligations, and discharges nested with `all`, `any` and `not`.
//!
//! Each run generates into its own root under `CARGO_TARGET_TMPDIR`, with the repository's
//! vocabulary copied in; nothing in the repository is written.

use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;

/// Three claims and three obligations: `decide` is discharged once `c` is no longer UNKNOWN (the
/// shape Canon's own obligations tests use), `refute` once `a` is not TRUE, `settle` by a nested
/// `all`/`any` that tests `a` twice.
const PROTOCOL: &str = "format: protocol/1
protocol: {id: nested.discharge, revision: 1}
evidence_kinds: {e: {}}
claims:
  a: {true_when: {evidence: {kind: e, result: ok}}}
  b: {true_when: {evidence: {kind: e, result: done}}}
  c: {true_when: {evidence: {kind: e, result: found}}}
obligations:
  settle:
    discharged_when: {all: [{claim: a}, {any: [{claim: b}, {claim: a}]}]}
  refute:
    discharged_when: {not: {claim: a}}
  decide:
    discharged_when: {not: {claim: c, is: unknown}}
";

fn manifest_dir() -> PathBuf {
    PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("cargo sets CARGO_MANIFEST_DIR"))
}

/// A fresh root holding the repository's vocabulary and `PROTOCOL` as
/// `protocols/nested-discharge/1.yaml`, generated with the `els-docs` binary.
fn generated(name: &str) -> PathBuf {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "adversary2-obligation-render-{}-{name}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("protocols/nested-discharge")).expect("scratch root");
    std::fs::copy(
        manifest_dir().join("../../protocols/vocabulary.yaml"),
        root.join("protocols/vocabulary.yaml"),
    )
    .expect("copy the vocabulary");
    std::fs::write(root.join("protocols/nested-discharge/1.yaml"), PROTOCOL).expect("protocol");
    let output = Command::new(env!("CARGO_BIN_EXE_els-docs"))
        .args(["generate", "--root"])
        .arg(&root)
        .output()
        .expect("els-docs runs");
    assert!(
        output.status.success(),
        "els-docs generate: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    root
}

fn read(root: &Path, path: &str) -> String {
    std::fs::read_to_string(root.join(path)).unwrap_or_else(|error| panic!("{path}: {error}"))
}

fn graph(root: &Path) -> Value {
    serde_json::from_str(&read(
        root,
        "website/data/protocol-graphs/nested-discharge-1.json",
    ))
    .expect("the graph is JSON")
}

/// The edges into `obligation:<id>`, each as its JSON text, in document order.
fn edges_into(graph: &Value, obligation: &str) -> Vec<String> {
    let target = format!("obligation:{obligation}");
    graph["edges"]
        .as_array()
        .expect("edges")
        .iter()
        .filter(|edge| edge["to"] == target.as_str())
        .map(Value::to_string)
        .collect()
}

/// Probe, green: the page's Obligations table reads each nested discharge faithfully, in id
/// order, and the graph has one edge per claim an obligation tests, no duplicates, grouped by
/// obligation in id order; two runs give the same bytes.
#[test]
fn several_nested_obligations_render_faithfully_and_deterministically() {
    let root = generated("faithful");
    let page = read(&root, "website/docs/protocols/nested-discharge/1.mdx");
    let start = page.find("## Obligations").expect("an Obligations section");
    let rest = &page[start + 2..];
    let section = &page[start..start + 2 + rest.find("\n## ").unwrap_or(rest.len())];
    let rows: Vec<&str> = section
        .lines()
        .filter(|line| line.starts_with("| `"))
        .collect();
    assert_eq!(
        rows,
        [
            "| `decide` |  | not (`c` is **UNKNOWN**) |",
            "| `refute` |  | not (`a` is **TRUE**) |",
            // Canon's compiled order: `any` sorts before `claim`.
            "| `settle` |  | all of (any of (`a` is **TRUE**; `b` is **TRUE**); `a` is **TRUE**) |",
        ],
        "{section}"
    );

    let graph = graph(&root);
    let into_settle = edges_into(&graph, "settle");
    assert_eq!(
        into_settle,
        [
            r#"{"from":"claim:a","kind":"requires","to":"obligation:settle"}"#,
            r#"{"from":"claim:b","kind":"requires","to":"obligation:settle"}"#,
        ]
    );
    let order: Vec<String> = graph["edges"]
        .as_array()
        .expect("edges")
        .iter()
        .filter_map(|edge| edge["to"].as_str())
        .filter(|to| to.starts_with("obligation:"))
        .map(str::to_owned)
        .collect();
    assert_eq!(
        order,
        [
            "obligation:decide",
            "obligation:refute",
            "obligation:settle",
            "obligation:settle"
        ]
    );

    let again = generated("faithful-again");
    for path in [
        "website/docs/protocols/nested-discharge/1.mdx",
        "website/data/protocol-graphs/nested-discharge-1.json",
        "website/docs/protocols/index.md",
    ] {
        assert_eq!(read(&root, path), read(&again, path), "{path}");
    }
}

/// `graph.rs` module docs: "A claim edge carries the tested value as its qualifier when that value
/// is not `true`." Under `not`, the value the obligation needs is the opposite of the one the
/// claim test names: `refute` needs `a` not TRUE, and `decide` needs `c` not UNKNOWN. The edges
/// drop the `not`: `refute`'s edge is the edge `{claim: a}` would give ("requires a"), and
/// `decide`'s is qualified `unknown`, the one value that leaves `decide` open.
#[test]
fn a_negated_discharge_is_not_drawn_as_its_opposite() {
    let root = generated("negated");
    let graph = graph(&root);
    let mut wrong = Vec::new();
    let refute = edges_into(&graph, "refute");
    if refute == [r#"{"from":"claim:a","kind":"requires","to":"obligation:refute"}"#] {
        wrong.push(format!(
            "`refute` (discharged when not `a`) is drawn as requiring `a`: {refute:?}"
        ));
    }
    let decide = edges_into(&graph, "decide");
    if decide
        .iter()
        .any(|edge| edge.contains(r#""qualifier":"unknown""#))
    {
        wrong.push(format!(
            "`decide` (discharged when `c` is not UNKNOWN) is drawn as requiring `c` UNKNOWN: \
             {decide:?}"
        ));
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// The protocols index says what each protocol declares. For `incident.response/1` that is three
/// claims, six actions, no outcome and one obligation — the obligation its whole emergency rests
/// on — and the row counts only the first three.
#[test]
fn the_index_counts_the_obligations_a_protocol_declares() {
    let index =
        std::fs::read_to_string(manifest_dir().join("../../website/docs/protocols/index.md"))
            .expect("the protocols index reads");
    let row = index
        .lines()
        .find(|line| line.contains("incident.response/1"))
        .expect("the index lists incident.response/1");
    assert!(row.contains("obligation"), "{row}");
}
