//! A compiled protocol as a `b10x-protocol-graph/1` document, the input of the docs-system
//! `ProtocolGraph` component.
//!
//! Nodes are ordered by kind (action, evidence, claim, outcome, obligation), then by name, and are
//! identified as `<kind>:<name>`. Edges: `produces` (action → evidence kind it may produce),
//! `establishes` (evidence kind → claim whose `true_when` matches it, qualified by the result),
//! `supports` (claim → claim whose `true_when` tests it), `requires` (claim → outcome whose
//! `requires` tests it) and `gates` (claim → action whose precondition tests it). A claim edge
//! carries the tested value as its qualifier when that value is not `true`. An outcome or a
//! precondition that matches evidence directly keeps that match in its predicate; the format has no
//! edge kind for it.

use b10x_canon::ir::Ir;
use b10x_canon::model::{Predicate, Truth};
use serde_json::{Map, Value, json};

/// The format this module emits.
pub const FORMAT: &str = "b10x-protocol-graph/1";

/// A predicate in the `canon-ir/1` shape: a claim test omits `is` when it is `true`, an evidence
/// match omits `result` when it has none.
pub fn predicate(predicate: &Predicate) -> Value {
    match predicate {
        Predicate::All(members) => {
            json!({"all": members.iter().map(self::predicate).collect::<Vec<_>>()})
        }
        Predicate::Any(members) => {
            json!({"any": members.iter().map(self::predicate).collect::<Vec<_>>()})
        }
        Predicate::Not(inner) => json!({"not": self::predicate(inner)}),
        Predicate::Claim(test) => {
            let mut claim = Map::new();
            claim.insert("id".into(), json!(test.claim.as_str()));
            if test.is != Truth::True {
                claim.insert("is".into(), json!(test.is.to_string()));
            }
            json!({"claim": claim})
        }
        Predicate::Evidence(matching) => {
            let mut evidence = Map::new();
            evidence.insert("kind".into(), json!(matching.kind.as_str()));
            if let Some(result) = &matching.result {
                evidence.insert("result".into(), json!(result));
            }
            json!({"evidence": evidence})
        }
    }
}

fn node(kind: &str, name: &str, description: &Option<String>) -> Map<String, Value> {
    let mut node = Map::new();
    node.insert("id".into(), json!(format!("{kind}:{name}")));
    node.insert("kind".into(), json!(kind));
    node.insert("name".into(), json!(name));
    if let Some(text) = description
        .as_deref()
        .map(str::trim)
        .filter(|text| !text.is_empty())
    {
        node.insert("description".into(), json!(text));
    }
    node
}

fn is_trivial(predicate: &Predicate) -> bool {
    matches!(predicate, Predicate::All(members) if members.is_empty())
}

#[derive(PartialEq)]
struct Edge {
    from: String,
    to: String,
    kind: &'static str,
    qualifier: Option<String>,
}

impl Edge {
    fn value(&self) -> Value {
        let mut edge = Map::new();
        edge.insert("from".into(), json!(self.from));
        edge.insert("to".into(), json!(self.to));
        edge.insert("kind".into(), json!(self.kind));
        if let Some(qualifier) = &self.qualifier {
            edge.insert("qualifier".into(), json!(qualifier));
        }
        Value::Object(edge)
    }
}

fn claim_qualifier(is: Truth) -> Option<String> {
    (is != Truth::True).then(|| is.to_string())
}

/// The graph document for one compiled protocol; `source` names the document it came from.
pub fn document(ir: &Ir, source: &str) -> Value {
    let mut nodes = Vec::new();
    for (id, action) in &ir.actions {
        let mut value = node("action", id.as_str(), &action.description);
        if !is_trivial(&action.precondition) {
            value.insert("predicate".into(), predicate(&action.precondition));
        }
        if let Some(effect) = &action.effect {
            value.insert("effect".into(), json!(effect.as_str()));
        }
        if !action.requires.is_empty() {
            let capabilities: Vec<_> = action.requires.iter().map(|c| c.as_str()).collect();
            value.insert("capabilities".into(), json!(capabilities));
        }
        nodes.push(Value::Object(value));
    }
    for (id, kind) in &ir.evidence_kinds {
        nodes.push(Value::Object(node(
            "evidence",
            id.as_str(),
            &kind.description,
        )));
    }
    for (id, claim) in &ir.claims {
        let mut value = node("claim", id.as_str(), &claim.description);
        value.insert("predicate".into(), predicate(&claim.true_when));
        nodes.push(Value::Object(value));
    }
    for (id, outcome) in &ir.outcomes {
        let mut value = node("outcome", id.as_str(), &outcome.description);
        value.insert("predicate".into(), predicate(&outcome.requires));
        nodes.push(Value::Object(value));
    }
    for (id, obligation) in &ir.obligations {
        nodes.push(Value::Object(node(
            "obligation",
            id.as_str(),
            &obligation.description,
        )));
    }

    let mut edges: Vec<Edge> = Vec::new();
    let mut add = |edge: Edge| {
        if !edges.contains(&edge) {
            edges.push(edge);
        }
    };
    for (id, action) in &ir.actions {
        for kind in &action.may_produce {
            add(Edge {
                from: format!("action:{id}"),
                to: format!("evidence:{kind}"),
                kind: "produces",
                qualifier: None,
            });
        }
    }
    for (id, claim) in &ir.claims {
        claim.true_when.visit(&mut |inner| match inner {
            Predicate::Evidence(matching) => add(Edge {
                from: format!("evidence:{}", matching.kind),
                to: format!("claim:{id}"),
                kind: "establishes",
                qualifier: matching.result.clone(),
            }),
            Predicate::Claim(test) => add(Edge {
                from: format!("claim:{}", test.claim),
                to: format!("claim:{id}"),
                kind: "supports",
                qualifier: claim_qualifier(test.is),
            }),
            Predicate::All(_) | Predicate::Any(_) | Predicate::Not(_) => {}
        });
    }
    for (id, outcome) in &ir.outcomes {
        outcome.requires.visit(&mut |inner| {
            if let Predicate::Claim(test) = inner {
                add(Edge {
                    from: format!("claim:{}", test.claim),
                    to: format!("outcome:{id}"),
                    kind: "requires",
                    qualifier: claim_qualifier(test.is),
                });
            }
        });
    }
    for (id, action) in &ir.actions {
        action.precondition.visit(&mut |inner| {
            if let Predicate::Claim(test) = inner {
                add(Edge {
                    from: format!("claim:{}", test.claim),
                    to: format!("action:{id}"),
                    kind: "gates",
                    qualifier: claim_qualifier(test.is),
                });
            }
        });
    }

    let mut protocol = Map::new();
    protocol.insert("id".into(), json!(ir.protocol.id.as_str()));
    protocol.insert("revision".into(), json!(ir.protocol.revision));
    if let Some(text) = ir
        .protocol
        .description
        .as_deref()
        .map(str::trim)
        .filter(|t| !t.is_empty())
    {
        protocol.insert("description".into(), json!(text));
    }
    protocol.insert("source".into(), json!(source));
    json!({
        "format": FORMAT,
        "protocol": protocol,
        "nodes": nodes,
        "edges": edges.iter().map(Edge::value).collect::<Vec<_>>(),
    })
}

/// Every node name of one kind, in document order.
#[cfg(test)]
pub fn names(document: &Value, kind: &str) -> Vec<String> {
    document["nodes"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|node| node["kind"] == kind)
        .filter_map(|node| node["name"].as_str().map(str::to_owned))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use b10x_canon::ir::compile;
    use b10x_canon::model::parse;

    const INVESTIGATION: &str = include_str!("../tests/fixtures/investigation.yaml");
    /// docs-system `examples/product-site/data/investigation.protocol-graph.json` at `929f965`,
    /// copied byte for byte: the reference rendering of the same fixture.
    const REFERENCE: &str = include_str!("../tests/fixtures/investigation.protocol-graph.json");

    fn sorted(value: &Value) -> Vec<String> {
        let mut items: Vec<String> = value
            .as_array()
            .expect("array")
            .iter()
            .map(Value::to_string)
            .collect();
        items.sort();
        items
    }

    #[test]
    fn the_investigation_fixture_matches_the_reference_graph() {
        let ir = compile(&parse(INVESTIGATION).expect("parses")).expect("compiles");
        let ours = document(&ir, "protocols/investigation/1.yaml");
        let reference: Value = serde_json::from_str(REFERENCE).expect("reference");
        assert_eq!(ours["format"], reference["format"]);
        assert_eq!(ours["nodes"], reference["nodes"]);
        assert_eq!(sorted(&ours["edges"]), sorted(&reference["edges"]));
        assert_eq!(ours["protocol"]["id"], reference["protocol"]["id"]);
        assert_eq!(
            ours["protocol"]["revision"],
            reference["protocol"]["revision"]
        );
        assert_eq!(
            ours["protocol"]["description"],
            reference["protocol"]["description"]
        );
    }

    #[test]
    fn preconditions_gate_and_negative_tests_are_qualified() {
        let source = "format: protocol/1\nprotocol: {id: p, revision: 1}\nevidence_kinds: {e: {}}\nclaims:\n  a:\n    true_when: {evidence: {kind: e}}\n  b:\n    true_when: {not: {claim: a, is: unknown}}\nactions:\n  go:\n    precondition: {claim: b}\n    requires: [{capability: c}]\n    effect: write\noutcomes:\n  done:\n    requires: {claim: b, is: false}\n";
        let ir = compile(&parse(source).expect("parses")).expect("compiles");
        let graph = document(&ir, "x");
        let edges = sorted(&graph["edges"]);
        assert!(
            edges.contains(
                &r#"{"from":"claim:a","kind":"supports","qualifier":"unknown","to":"claim:b"}"#
                    .to_owned()
            ),
            "{edges:?}"
        );
        assert!(
            edges.contains(&r#"{"from":"claim:b","kind":"gates","to":"action:go"}"#.to_owned()),
            "{edges:?}"
        );
        assert!(
            edges.contains(
                &r#"{"from":"claim:b","kind":"requires","qualifier":"false","to":"outcome:done"}"#
                    .to_owned()
            ),
            "{edges:?}"
        );
        let action = &graph["nodes"][0];
        assert_eq!(action["capabilities"], json!(["c"]));
        assert_eq!(action["effect"], json!("write"));
        assert_eq!(action["predicate"], json!({"claim": {"id": "b"}}));
    }
}
