//! Independent review of `story:incident-response-protocol` (wave 2026-10-04-w8), at 0d4eccc.
//!
//! Invariants under check, for every committed `website/data/protocol-graphs/*.json`:
//!
//! 1. It conforms to the pinned docs-system schema, `schema/b10x.protocol-graph.v1.schema.json`
//!    at `929f965` (the commit `website/package.json` pins), transcribed below: closed key sets,
//!    required keys, the node and edge kind enums, non-empty strings and the predicate shapes.
//!    Nothing else in this repository checks a generated graph against that schema; the unit
//!    tests compare one reference rendering byte for byte, which has no obligation in it.
//! 2. It says what the compiled protocol says: one node per declared action, evidence kind,
//!    claim, outcome and obligation, ordered by kind then name; each action's effect and
//!    capabilities are its compiled `effect` and `requires`; and every claim an obligation's
//!    discharge or an action's precondition tests has an edge into it.

use std::collections::BTreeSet;
use std::path::PathBuf;

use b10x_canon::ir::{Ir, compile};
use b10x_canon::model::{Predicate, parse};
use serde_json::{Map, Value};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn closed(
    object: &Map<String, Value>,
    allowed: &[&str],
    required: &[&str],
    at: &str,
) -> Vec<String> {
    let mut problems = Vec::new();
    for key in object.keys() {
        if !allowed.contains(&key.as_str()) {
            problems.push(format!("{at}: key `{key}` is not in the schema"));
        }
    }
    for key in required {
        if !object.contains_key(*key) {
            problems.push(format!("{at}: required key `{key}` is missing"));
        }
    }
    problems
}

fn nonempty(value: &Value, at: &str) -> Vec<String> {
    match value.as_str() {
        Some(text) if !text.is_empty() => Vec::new(),
        _ => vec![format!("{at}: not a non-empty string: {value}")],
    }
}

fn predicate(value: &Value, at: &str) -> Vec<String> {
    let Some(object) = value.as_object() else {
        return vec![format!("{at}: predicate is not an object")];
    };
    if object.len() != 1 {
        return vec![format!(
            "{at}: predicate has {} keys, expected one",
            object.len()
        )];
    }
    let (key, inner) = object.iter().next().expect("one key");
    match key.as_str() {
        "all" | "any" => match inner.as_array() {
            Some(members) => members
                .iter()
                .enumerate()
                .flat_map(|(index, member)| predicate(member, &format!("{at}.{key}[{index}]")))
                .collect(),
            None => vec![format!("{at}.{key}: not an array")],
        },
        "not" => predicate(inner, &format!("{at}.not")),
        "claim" => match inner.as_object() {
            Some(claim) => {
                let mut problems = closed(claim, &["id", "is"], &["id"], &format!("{at}.claim"));
                problems.extend(nonempty(&claim["id"], &format!("{at}.claim.id")));
                if let Some(is) = claim.get("is")
                    && !["true", "false", "unknown"].contains(&is.as_str().unwrap_or(""))
                {
                    problems.push(format!("{at}.claim.is: {is} is not true, false or unknown"));
                }
                problems
            }
            None => vec![format!("{at}.claim: not an object")],
        },
        "evidence" => match inner.as_object() {
            Some(evidence) => {
                let mut problems = closed(
                    evidence,
                    &["kind", "result"],
                    &["kind"],
                    &format!("{at}.evidence"),
                );
                problems.extend(nonempty(&evidence["kind"], &format!("{at}.evidence.kind")));
                if let Some(result) = evidence.get("result")
                    && !result.is_null()
                {
                    problems.extend(nonempty(result, &format!("{at}.evidence.result")));
                }
                problems
            }
            None => vec![format!("{at}.evidence: not an object")],
        },
        other => vec![format!("{at}: `{other}` is not a predicate form")],
    }
}

/// Problems with `graph` against the pinned schema.
fn schema_problems(graph: &Value) -> Vec<String> {
    let Some(top) = graph.as_object() else {
        return vec!["the document is not an object".to_owned()];
    };
    let mut problems = closed(
        top,
        &["format", "protocol", "nodes", "edges"],
        &["format", "protocol", "nodes", "edges"],
        "$",
    );
    if top.get("format") != Some(&Value::from("b10x-protocol-graph/1")) {
        problems.push(format!("$.format is {:?}", top.get("format")));
    }
    match top.get("protocol").and_then(Value::as_object) {
        Some(protocol) => {
            problems.extend(closed(
                protocol,
                &["id", "revision", "description", "source"],
                &["id", "revision"],
                "$.protocol",
            ));
            problems.extend(nonempty(&protocol["id"], "$.protocol.id"));
            if protocol["revision"]
                .as_u64()
                .is_none_or(|revision| revision < 1)
            {
                problems.push(format!("$.protocol.revision: {}", protocol["revision"]));
            }
            for key in ["description", "source"] {
                if let Some(value) = protocol.get(key) {
                    problems.extend(nonempty(value, &format!("$.protocol.{key}")));
                }
            }
        }
        None => problems.push("$.protocol is not an object".to_owned()),
    }
    let kinds = ["action", "evidence", "claim", "outcome", "obligation"];
    for (index, node) in top["nodes"].as_array().into_iter().flatten().enumerate() {
        let at = format!("$.nodes[{index}]");
        let Some(node) = node.as_object() else {
            problems.push(format!("{at}: not an object"));
            continue;
        };
        problems.extend(closed(
            node,
            &[
                "id",
                "kind",
                "name",
                "description",
                "predicate",
                "effect",
                "capabilities",
            ],
            &["id", "kind", "name"],
            &at,
        ));
        problems.extend(nonempty(&node["id"], &format!("{at}.id")));
        problems.extend(nonempty(&node["name"], &format!("{at}.name")));
        if !kinds.contains(&node["kind"].as_str().unwrap_or("")) {
            problems.push(format!("{at}.kind: {}", node["kind"]));
        }
        for key in ["description", "effect"] {
            if let Some(value) = node.get(key) {
                problems.extend(nonempty(value, &format!("{at}.{key}")));
            }
        }
        if let Some(value) = node.get("predicate") {
            problems.extend(predicate(value, &format!("{at}.predicate")));
        }
        if let Some(capabilities) = node.get("capabilities") {
            match capabilities.as_array() {
                Some(list) => {
                    for (position, capability) in list.iter().enumerate() {
                        problems.extend(nonempty(
                            capability,
                            &format!("{at}.capabilities[{position}]"),
                        ));
                    }
                }
                None => problems.push(format!("{at}.capabilities: not an array")),
            }
        }
    }
    let edge_kinds = ["produces", "establishes", "supports", "requires", "gates"];
    for (index, edge) in top["edges"].as_array().into_iter().flatten().enumerate() {
        let at = format!("$.edges[{index}]");
        let Some(edge) = edge.as_object() else {
            problems.push(format!("{at}: not an object"));
            continue;
        };
        problems.extend(closed(
            edge,
            &["from", "to", "kind", "qualifier"],
            &["from", "to", "kind"],
            &at,
        ));
        problems.extend(nonempty(&edge["from"], &format!("{at}.from")));
        problems.extend(nonempty(&edge["to"], &format!("{at}.to")));
        if !edge_kinds.contains(&edge["kind"].as_str().unwrap_or("")) {
            problems.push(format!("{at}.kind: {}", edge["kind"]));
        }
        if let Some(qualifier) = edge.get("qualifier") {
            problems.extend(nonempty(qualifier, &format!("{at}.qualifier")));
        }
    }
    if top.get("nodes").and_then(Value::as_array).is_none() {
        problems.push("$.nodes is not an array".to_owned());
    }
    if top.get("edges").and_then(Value::as_array).is_none() {
        problems.push("$.edges is not an array".to_owned());
    }
    problems
}

/// Every claim `predicate` tests, through `all`, `any` and `not`.
fn tested(predicate: &Predicate) -> BTreeSet<String> {
    let mut claims = BTreeSet::new();
    predicate.visit(&mut |node| {
        if let Predicate::Claim(test) = node {
            claims.insert(test.claim.as_str().to_owned());
        }
    });
    claims
}

/// Problems with `graph` against what `ir` declares.
fn fidelity_problems(graph: &Value, ir: &Ir) -> Vec<String> {
    let mut problems = Vec::new();
    let mut expected: Vec<String> = Vec::new();
    expected.extend(ir.actions.keys().map(|id| format!("action:{id}")));
    expected.extend(ir.evidence_kinds.keys().map(|id| format!("evidence:{id}")));
    expected.extend(ir.claims.keys().map(|id| format!("claim:{id}")));
    expected.extend(ir.outcomes.keys().map(|id| format!("outcome:{id}")));
    expected.extend(ir.obligations.keys().map(|id| format!("obligation:{id}")));
    let nodes: Vec<&Value> = graph["nodes"].as_array().into_iter().flatten().collect();
    let found: Vec<String> = nodes
        .iter()
        .map(|node| node["id"].as_str().unwrap_or("").to_owned())
        .collect();
    if found != expected {
        problems.push(format!(
            "nodes are {found:?}, the compiled protocol declares {expected:?}"
        ));
    }
    let node = |id: &str| nodes.iter().find(|node| node["id"] == id);
    for (id, action) in &ir.actions {
        let Some(node) = node(&format!("action:{id}")) else {
            continue;
        };
        let effect = action
            .effect
            .as_ref()
            .map(|effect| Value::from(effect.as_str()));
        if node.get("effect").cloned() != effect {
            problems.push(format!(
                "action `{id}`: effect {:?}, compiled {effect:?}",
                node.get("effect")
            ));
        }
        let mut required: Vec<String> = action
            .requires
            .iter()
            .map(|c| c.as_str().to_owned())
            .collect();
        required.sort();
        required.dedup();
        let mut drawn: Vec<String> = node
            .get("capabilities")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|c| c.as_str().map(str::to_owned))
            .collect();
        drawn.sort();
        if drawn != required {
            problems.push(format!(
                "action `{id}`: capabilities {drawn:?}, compiled requires {required:?}"
            ));
        }
    }
    let edges: Vec<(String, String, String)> = graph["edges"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|edge| {
            let text = |key: &str| edge[key].as_str().unwrap_or("").to_owned();
            (text("from"), text("to"), text("kind"))
        })
        .collect();
    let has = |from: &str, to: &str, kind: &str| {
        edges
            .iter()
            .any(|(f, t, k)| f == from && t == to && k == kind)
    };
    for (id, obligation) in &ir.obligations {
        for claim in tested(&obligation.discharged_when) {
            if !has(
                &format!("claim:{claim}"),
                &format!("obligation:{id}"),
                "requires",
            ) {
                problems.push(format!(
                    "obligation `{id}`: no `requires` edge from claim `{claim}`"
                ));
            }
        }
    }
    for (id, action) in &ir.actions {
        for claim in tested(&action.precondition) {
            if !has(&format!("claim:{claim}"), &format!("action:{id}"), "gates") {
                problems.push(format!(
                    "action `{id}`: no `gates` edge from claim `{claim}`"
                ));
            }
        }
    }
    problems
}

#[test]
fn every_committed_protocol_graph_conforms_to_the_pinned_schema_and_its_protocol() {
    let graphs = root().join("website/data/protocol-graphs");
    let mut checked = 0;
    let mut problems = Vec::new();
    for entry in std::fs::read_dir(&graphs).expect("the graph directory reads") {
        let path = entry.expect("an entry").path();
        let Some(stem) = path
            .file_name()
            .and_then(|name| name.to_str())
            .and_then(|name| name.strip_suffix(".json"))
        else {
            continue;
        };
        let graph: Value = serde_json::from_str(&std::fs::read_to_string(&path).expect("reads"))
            .unwrap_or_else(|error| panic!("{stem}: {error}"));
        for problem in schema_problems(&graph) {
            problems.push(format!("{stem}: {problem}"));
        }
        let source = graph["protocol"]["source"]
            .as_str()
            .expect("the graph names its source");
        let text = std::fs::read_to_string(root().join(source)).expect("the source protocol reads");
        let ir = compile(&parse(&text).expect("parses")).expect("compiles");
        for problem in fidelity_problems(&graph, &ir) {
            problems.push(format!("{stem}: {problem}"));
        }
        checked += 1;
    }
    assert!(checked > 0, "no graph was checked");
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}
