//! Adversary pass 1 on `story:incident-response-protocol`: the generated protocol page and graph
//! compared with `protocols/incident-response/1.yaml`.
//!
//! The YAML says `restore_service` is discharged when `service.healthy` is TRUE, and
//! `emergency.leave`'s description says it is admissible "once restore_service is discharged".
//! The Canon pin bump in this unit (33540b1 -> 8fc260a) added `discharged_when` to the compiled
//! obligation; the page and the graph are generated from that IR and still render an obligation
//! as its description only.

#[allow(dead_code)] // uses part of the harness; fixture_harness.rs uses all of it
mod support;

fn read(path: &str) -> String {
    std::fs::read_to_string(support::repo_root().join(path))
        .unwrap_or_else(|error| panic!("{path}: {error}"))
}

#[test]
fn protocol_page_and_graph_say_what_discharges_restore_service() {
    let mut missing = Vec::new();

    let page = read("website/docs/protocols/incident-response/1.mdx");
    let start = page
        .find("## Obligations")
        .expect("the page has an Obligations section");
    let rest = &page[start + 2..];
    let section = &page[start..start + 2 + rest.find("\n## ").unwrap_or(rest.len())];
    if !section.contains("service.healthy") {
        missing.push(format!(
            "the page's Obligations section does not name `service.healthy`:\n{section}"
        ));
    }

    let graph: serde_yaml_ng::Value = serde_yaml_ng::from_str(&read(
        "website/data/protocol-graphs/incident-response-1.json",
    ))
    .expect("the graph is JSON, which YAML reads");
    let edges = graph["edges"].as_sequence().expect("the graph has edges");
    let touching: Vec<&serde_yaml_ng::Value> = edges
        .iter()
        .filter(|edge| {
            edge["from"] == "obligation:restore_service"
                || edge["to"] == "obligation:restore_service"
        })
        .collect();
    if touching.is_empty() {
        missing.push("the graph has no edge at `obligation:restore_service`".to_owned());
    }

    assert!(missing.is_empty(), "{}", missing.join("\n"));
}
