//! Adversary pass 2 on `story:software-change-protocol`, against the F1 change: every evidence
//! match bound to a subject, Canon at 8d1599e, and `els-docs` rendering subjects.
//!
//! Every case reads the shipped `protocols/software-change/1.yaml` and the committed graph
//! `website/data/protocol-graphs/software-change-1.json`; nothing in the repository is written.

// `accepted_is_blocked_while_the_merge_is_denied_and_r2_never_released` moved to draft story:release-provenance-binding as its acceptance.

#[allow(dead_code)] // uses part of the harness; fixture_harness.rs uses all of it
mod support;

use std::collections::BTreeSet;

use b10x_canon::ir::Ir;
use b10x_canon::model::Predicate;
use serde_yaml_ng::Value;

const PROTOCOL: &str = "protocols/software-change/1.yaml";
const GRAPH: &str = "website/data/protocol-graphs/software-change-1.json";

fn read(path: &str) -> String {
    std::fs::read_to_string(support::repo_root().join(path))
        .unwrap_or_else(|error| panic!("{path}: {error}"))
}

fn shipped() -> Ir {
    let protocol =
        b10x_canon::model::parse(&read(PROTOCOL)).unwrap_or_else(|error| panic!("{error}"));
    b10x_canon::ir::compile(&protocol).unwrap_or_else(|problems| panic!("{problems:?}"))
}

/// Every artifact an evidence match of `predicate` names as its subject.
fn subjects(predicate: &Predicate) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    predicate.visit(&mut |node| {
        if let Predicate::Evidence(matching) = node
            && let Some(subject) = &matching.subject
        {
            found.insert(subject.as_str().to_owned());
        }
    });
    found
}

/// The committed protocol graph is the reader's picture of the compiled protocol
/// (`crates/els-docs/src/graph.rs:28`: "A predicate in the `canon-ir/1` shape"). In `canon-ir/1`
/// an evidence match with no `subject` reads records about any artifact (Canon 8d1599e
/// `EvidenceMatch::subject`), so a graph that drops the subject draws exactly the unbound
/// protocol F1 removed. For every claim, each artifact its evidence matches are bound to must
/// appear in the claim node's predicate or in a qualifier of an edge into the claim (the schema
/// at 929f965 leaves `qualifier` a free string), not only in the description.
#[test]
fn the_committed_graph_shows_the_subject_of_every_evidence_match() {
    let ir = shipped();
    let graph: Value = serde_yaml_ng::from_str(&read(GRAPH)).unwrap();
    let nodes = graph["nodes"].as_sequence().expect("nodes");
    let edges = graph["edges"].as_sequence().expect("edges");
    let mut hidden = Vec::new();
    for (id, declared) in &ir.claims {
        let node_id = format!("claim:{id}");
        let node = nodes
            .iter()
            .find(|node| node["id"].as_str() == Some(node_id.as_str()))
            .unwrap_or_else(|| panic!("the graph has no node `{node_id}`"));
        let mut shown = serde_yaml_ng::to_string(&node["predicate"]).unwrap();
        for edge in edges {
            if edge["to"].as_str() == Some(node_id.as_str()) {
                shown.push_str(&serde_yaml_ng::to_string(&edge["qualifier"]).unwrap());
            }
        }
        for subject in subjects(&declared.true_when) {
            if !shown.contains(&subject) {
                hidden.push(format!("{id}: about `{subject}`"));
            }
        }
    }
    assert_eq!(
        hidden,
        Vec::<String>::new(),
        "the graph draws these bound matches as unbound ones"
    );
}
