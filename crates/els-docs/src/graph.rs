//! The dependency graph of one compiled protocol, as a Mermaid flowchart: actions → evidence kinds
//! → claims → outcomes.
//!
//! An edge means "is used by": an action may produce an evidence kind, a claim's `true_when` names
//! an evidence kind or another claim, an outcome's `requires` names a claim or an evidence kind.
//! Action preconditions are not drawn; the actions table shows them.

use std::fmt::Write as _;

use b10x_canon::ir::Ir;
use b10x_canon::model::Predicate;

use crate::markdown::mermaid_label;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Column {
    Action,
    Evidence,
    Claim,
    Outcome,
}

impl Column {
    const ALL: [Column; 4] = [
        Column::Action,
        Column::Evidence,
        Column::Claim,
        Column::Outcome,
    ];

    fn prefix(self) -> &'static str {
        match self {
            Column::Action => "a",
            Column::Evidence => "e",
            Column::Claim => "c",
            Column::Outcome => "o",
        }
    }

    fn title(self) -> &'static str {
        match self {
            Column::Action => "Actions",
            Column::Evidence => "Evidence kinds",
            Column::Claim => "Claims",
            Column::Outcome => "Outcomes",
        }
    }

    fn subgraph(self) -> &'static str {
        match self {
            Column::Action => "actions",
            Column::Evidence => "evidence_kinds",
            Column::Claim => "claims",
            Column::Outcome => "outcomes",
        }
    }
}

/// One edge, from a node to a node that uses it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Edge {
    pub from: (Column, String),
    pub to: (Column, String),
}

/// The nodes of each column, in the IR's canonical order.
pub fn nodes(ir: &Ir) -> [Vec<String>; 4] {
    [
        ir.actions.keys().map(ToString::to_string).collect(),
        ir.evidence_kinds.keys().map(ToString::to_string).collect(),
        ir.claims.keys().map(ToString::to_string).collect(),
        ir.outcomes.keys().map(ToString::to_string).collect(),
    ]
}

/// Every edge, each once.
pub fn edges(ir: &Ir) -> Vec<Edge> {
    let mut edges = Vec::new();
    let mut add = |from: (Column, String), to: (Column, String)| {
        let edge = Edge { from, to };
        if !edges.contains(&edge) {
            edges.push(edge);
        }
    };
    for (action, declared) in &ir.actions {
        for kind in &declared.may_produce {
            add(
                (Column::Action, action.to_string()),
                (Column::Evidence, kind.to_string()),
            );
        }
    }
    let mut uses = |predicate: &Predicate, to: (Column, String)| {
        predicate.visit(&mut |inner| match inner {
            Predicate::Evidence(matching) => {
                add((Column::Evidence, matching.kind.to_string()), to.clone())
            }
            Predicate::Claim(test) => add((Column::Claim, test.claim.to_string()), to.clone()),
            Predicate::All(_) | Predicate::Any(_) | Predicate::Not(_) => {}
        });
    };
    for (claim, declared) in &ir.claims {
        uses(&declared.true_when, (Column::Claim, claim.to_string()));
    }
    for (outcome, declared) in &ir.outcomes {
        uses(&declared.requires, (Column::Outcome, outcome.to_string()));
    }
    edges
}

/// The graph as the body of a ```` ```mermaid ```` block.
pub fn mermaid(ir: &Ir) -> String {
    let columns = nodes(ir);
    let node_id = |column: Column, id: &str| -> Option<String> {
        let index = columns[column as usize]
            .iter()
            .position(|candidate| candidate == id)?;
        Some(format!("{}{index}", column.prefix()))
    };
    let mut out = String::from("flowchart LR\n");
    if columns.iter().all(Vec::is_empty) {
        out.push_str("  none[\"no declarations\"]\n");
        return out;
    }
    for column in Column::ALL {
        let ids = &columns[column as usize];
        if ids.is_empty() {
            continue;
        }
        let _ = writeln!(
            out,
            "  subgraph {}[\"{}\"]",
            column.subgraph(),
            column.title()
        );
        for (index, id) in ids.iter().enumerate() {
            let _ = writeln!(
                out,
                "    {}{index}[\"{}\"]",
                column.prefix(),
                mermaid_label(id)
            );
        }
        out.push_str("  end\n");
    }
    for edge in edges(ir) {
        let (Some(from), Some(to)) = (
            node_id(edge.from.0, &edge.from.1),
            node_id(edge.to.0, &edge.to.1),
        ) else {
            continue;
        };
        let arrow = if edge.from.0 == edge.to.0 {
            "-.->"
        } else {
            "-->"
        };
        let _ = writeln!(out, "  {from} {arrow} {to}");
    }
    out
}
