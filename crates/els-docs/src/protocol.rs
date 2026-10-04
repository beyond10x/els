//! One protocol page, rendered from Canon's compiled form (`canon-ir/1`), never from the YAML text,
//! so the page shows what Canon understood (ELS `story:protocol-docs-render`).

use std::fmt::Write as _;

use b10x_canon::ir::Ir;
use b10x_canon::model::{Predicate, Truth};

use crate::graph::{self, Column};
use crate::markdown::{HEADER, code, text, yaml_string};

/// A protocol document found under `protocols/<name>/<major>.yaml`.
pub struct Source {
    pub name: String,
    pub major: u64,
    pub path: String,
    pub text: String,
}

impl Source {
    /// The generated file, relative to `website/docs/`.
    pub fn file(&self) -> String {
        format!("protocols/{}/{}.md", self.name, self.major)
    }

    /// The page's route below the site base.
    pub fn slug(&self) -> String {
        format!("/protocols/{}/{}", self.name, self.major)
    }
}

fn truth(value: Truth) -> &'static str {
    match value {
        Truth::True => "**TRUE**",
        Truth::False => "**FALSE**",
        Truth::Unknown => "**UNKNOWN**",
    }
}

/// A predicate as one readable line, nested with parentheses.
pub fn predicate(predicate: &Predicate) -> String {
    let list = |members: &[Predicate]| -> String {
        members
            .iter()
            .map(self::predicate)
            .collect::<Vec<_>>()
            .join("; ")
    };
    match predicate {
        Predicate::Evidence(matching) => match &matching.result {
            None => format!("evidence {}", code(matching.kind.as_str())),
            Some(result) => format!(
                "evidence {} with result {}",
                code(matching.kind.as_str()),
                code(result)
            ),
        },
        Predicate::Claim(test) => format!("{} is {}", code(test.claim.as_str()), truth(test.is)),
        Predicate::All(members) if members.is_empty() => "always true".to_owned(),
        Predicate::Any(members) if members.is_empty() => "never true".to_owned(),
        Predicate::All(members) => format!("all of ({})", list(members)),
        Predicate::Any(members) => format!("any of ({})", list(members)),
        Predicate::Not(inner) => format!("not ({})", self::predicate(inner)),
    }
}

fn description(value: &Option<String>) -> String {
    value
        .as_deref()
        .map(|value| text(value.trim()))
        .unwrap_or_default()
}

fn joined(items: Vec<String>, empty: &str) -> String {
    if items.is_empty() {
        format!("*{empty}*")
    } else {
        items.join(", ")
    }
}

fn table(out: &mut String, title: &str, lede: &str, head: &[&str], rows: Vec<Vec<String>>) {
    let _ = write!(out, "\n## {title}\n\n{lede}\n\n");
    if rows.is_empty() {
        let _ = writeln!(out, "*This protocol declares no {}.*", title.to_lowercase());
        return;
    }
    let _ = writeln!(out, "| {} |", head.join(" | "));
    let _ = writeln!(out, "|{}", "---|".repeat(head.len()));
    for row in rows {
        let _ = writeln!(out, "| {} |", row.join(" | "));
    }
}

/// The page for one compiled protocol.
pub fn page(ir: &Ir, source: &Source) -> String {
    let header = &ir.protocol;
    let edges = graph::edges(ir);
    let linked = |from: Column, to: Column, id: &str, forward: bool| -> Vec<String> {
        edges
            .iter()
            .filter(|edge| edge.from.0 == from && edge.to.0 == to)
            .filter(|edge| {
                if forward {
                    edge.from.1 == id
                } else {
                    edge.to.1 == id
                }
            })
            .map(|edge| code(if forward { &edge.to.1 } else { &edge.from.1 }))
            .collect()
    };
    let title = format!("{}/{}", header.id, source.major);

    let mut out = String::new();
    let summary = header
        .description
        .as_deref()
        .map(str::trim)
        .map(str::to_owned)
        .unwrap_or_else(|| format!("The ELS protocol {title}."));
    let _ = writeln!(out, "---\n{HEADER}");
    let _ = writeln!(out, "id: {}", yaml_string(&source.major.to_string()));
    let _ = writeln!(out, "title: {}", yaml_string(&title));
    let _ = writeln!(out, "sidebar_label: {}", yaml_string(&title));
    let _ = writeln!(out, "description: {}", yaml_string(&summary));
    let _ = writeln!(out, "slug: {}", source.slug());
    out.push_str("custom_edit_url: null\n---\n\n");

    if let Some(description) = &header.description {
        let _ = writeln!(out, "{}\n", text(description.trim()));
    }
    let _ = writeln!(out, "- **Id:** {}", code(header.id.as_str()));
    let _ = writeln!(out, "- **Revision:** {}", header.revision);
    let _ = writeln!(
        out,
        "- **Source:** [{}](https://github.com/beyond10x/els/blob/main/{})",
        code(&source.path),
        source.path
    );

    let _ = write!(
        out,
        "\n## Dependency graph\n\nWhich actions may produce which evidence, which claims that evidence establishes, and which outcomes rest on those claims. A dashed arrow is a claim whose predicate tests another claim. Action preconditions are in the actions table, not drawn.\n\n```mermaid\n{}```\n",
        graph::mermaid(ir)
    );

    table(
        &mut out,
        "Actions",
        "What can be done in a case: what it needs first, which authority it requires, the class of effect it has and the evidence it may produce.",
        &[
            "Action",
            "Description",
            "Precondition",
            "Requires capability",
            "Effect",
            "May produce",
        ],
        ir.actions
            .iter()
            .map(|(id, action)| {
                let precondition = match &action.precondition {
                    Predicate::All(members) if members.is_empty() => "*none*".to_owned(),
                    other => predicate(other),
                };
                vec![
                    code(id.as_str()),
                    description(&action.description),
                    precondition,
                    joined(
                        action.requires.iter().map(|c| code(c.as_str())).collect(),
                        "no authority",
                    ),
                    action
                        .effect
                        .as_ref()
                        .map(|effect| code(effect.as_str()))
                        .unwrap_or_else(|| "*not declared*".to_owned()),
                    joined(
                        action
                            .may_produce
                            .iter()
                            .map(|k| code(k.as_str()))
                            .collect(),
                        "no evidence",
                    ),
                ]
            })
            .collect(),
    );

    table(
        &mut out,
        "Evidence kinds",
        "The kinds of evidence this protocol admits.",
        &[
            "Evidence kind",
            "Description",
            "Produced by",
            "Used by claims",
        ],
        ir.evidence_kinds
            .iter()
            .map(|(id, kind)| {
                vec![
                    code(id.as_str()),
                    description(&kind.description),
                    joined(
                        linked(Column::Action, Column::Evidence, id.as_str(), false),
                        "no action",
                    ),
                    joined(
                        linked(Column::Evidence, Column::Claim, id.as_str(), true),
                        "no claim",
                    ),
                ]
            })
            .collect(),
    );

    table(
        &mut out,
        "Claims",
        "Each claim is TRUE, FALSE or UNKNOWN, established from evidence by its predicate. Missing or stale evidence leaves a claim UNKNOWN, never FALSE.",
        &["Claim", "Description", "True when"],
        ir.claims
            .iter()
            .map(|(id, claim)| {
                vec![
                    code(id.as_str()),
                    description(&claim.description),
                    predicate(&claim.true_when),
                ]
            })
            .collect(),
    );

    table(
        &mut out,
        "Obligations",
        "What must be done before a case can be complete.",
        &["Obligation", "Description"],
        ir.obligations
            .iter()
            .map(|(id, obligation)| vec![code(id.as_str()), description(&obligation.description)])
            .collect(),
    );

    table(
        &mut out,
        "Outcomes",
        "The legitimate terminal interpretations of a case, and what each requires.",
        &["Outcome", "Description", "Requires"],
        ir.outcomes
            .iter()
            .map(|(id, outcome)| {
                vec![
                    code(id.as_str()),
                    description(&outcome.description),
                    predicate(&outcome.requires),
                ]
            })
            .collect(),
    );

    table(
        &mut out,
        "Artifacts",
        "The things whose revisions evidence is bound to.",
        &["Artifact", "Description"],
        ir.artifacts
            .iter()
            .map(|(id, artifact)| vec![code(id.as_str()), description(&artifact.description)])
            .collect(),
    );
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use b10x_canon::model::parse;

    fn claim_predicate(yaml: &str) -> String {
        let source = format!(
            "format: protocol/1\nprotocol: {{id: p, revision: 1}}\nevidence_kinds: {{e: {{}}}}\nclaims:\n  a:\n    true_when: {{evidence: {{kind: e}}}}\n  b:\n    true_when: {yaml}\n"
        );
        let protocol = parse(&source).expect("parses");
        let (_, claim) = protocol.claims.iter().nth(1).expect("claim b");
        predicate(&claim.true_when)
    }

    #[test]
    fn predicates_read_as_one_nested_line() {
        assert_eq!(
            claim_predicate(
                "{any: [{not: {claim: a, is: unknown}}, {all: []}, {evidence: {kind: e, result: ok}}]}"
            ),
            "any of (not (`a` is **UNKNOWN**); always true; evidence `e` with result `ok`)"
        );
        assert_eq!(claim_predicate("{claim: a}"), "`a` is **TRUE**");
        assert_eq!(claim_predicate("{any: []}"), "never true");
    }
}
