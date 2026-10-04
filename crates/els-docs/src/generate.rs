//! Every derived page of the site: one page per protocol document, the protocols index and the
//! vocabulary, written into the Docusaurus docs tree and checked there for drift.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result, anyhow, bail, ensure};
use b10x_canon::ir::compile;
use b10x_canon::model::{one_line, parse};
use b10x_els::vocabulary::{Category, Marking, Vocabulary};

use crate::markdown::{HEADER, code, text, yaml_string};
use crate::protocol::{self, Source};

/// The Docusaurus docs tree, relative to the repository root.
pub const DOCS: &str = "website/docs";

/// What generation reads from the repository.
pub struct Inputs {
    pub vocabulary: String,
    pub protocols: Vec<Source>,
}

fn is_name(name: &str) -> bool {
    name.starts_with(|c: char| c.is_ascii_lowercase())
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

impl Inputs {
    /// Reads `protocols/vocabulary.yaml` and every `protocols/<name>/<major>.yaml` under `root`.
    pub fn read(root: &Path) -> Result<Self> {
        let dir = root.join("protocols");
        let vocabulary = fs::read_to_string(dir.join("vocabulary.yaml"))
            .with_context(|| format!("reading {}", dir.join("vocabulary.yaml").display()))?;
        let mut protocols = Vec::new();
        for entry in fs::read_dir(&dir).with_context(|| format!("reading {}", dir.display()))? {
            let entry = entry?;
            if !entry.file_type()?.is_dir() {
                continue;
            }
            let name = entry
                .file_name()
                .into_string()
                .map_err(|name| anyhow!("protocol directory {name:?} is not UTF-8"))?;
            ensure!(
                is_name(&name),
                "protocol directory `{}` must be lowercase letters, digits and hyphens",
                one_line(&name)
            );
            for file in fs::read_dir(entry.path())? {
                let file = file?;
                let file_name = file
                    .file_name()
                    .into_string()
                    .map_err(|name| anyhow!("protocol file {name:?} is not UTF-8"))?;
                let Some(stem) = file_name.strip_suffix(".yaml") else {
                    continue;
                };
                let path = format!("protocols/{name}/{file_name}");
                let major = stem
                    .parse::<u64>()
                    .ok()
                    .filter(|major| *major > 0 && major.to_string() == stem)
                    .with_context(|| {
                        format!("{}: a protocol file is named <major>.yaml", one_line(&path))
                    })?;
                let text =
                    fs::read_to_string(file.path()).with_context(|| format!("reading {path}"))?;
                protocols.push(Source {
                    name: name.clone(),
                    major,
                    path,
                    text,
                });
            }
        }
        protocols.sort_by(|a, b| (&a.name, a.major).cmp(&(&b.name, b.major)));
        Ok(Self {
            vocabulary,
            protocols,
        })
    }
}

fn category_title(category: Category) -> &'static str {
    match category {
        Category::ArtifactKind => "Artifact kinds",
        Category::EvidenceKind => "Evidence kinds",
        Category::ClaimId => "Claims",
        Category::ActionId => "Actions",
        Category::ObligationId => "Obligations",
        Category::OutcomeId => "Outcomes",
    }
}

fn vocabulary_page(vocabulary: &Vocabulary) -> String {
    let terms = vocabulary.terms();
    let mut out = format!(
        "---\n{HEADER}\ntitle: \"Engineering vocabulary\"\nsidebar_position: 3\ndescription: \"The names ELS protocols use, each with its category, marking and meaning.\"\ncustom_edit_url: null\n---\n\nThe {} names ELS protocols use, each declared once in [`protocols/vocabulary.yaml`](https://github.com/beyond10x/els/blob/main/protocols/vocabulary.yaml) with its category, its marking and its meaning. The vocabulary holds names and what they mean; what a claim, a piece of evidence or an obligation is, and how one is decided, belongs to Canon.\n\n- **`core`**: not specific to Git, pull requests or code; usable by every engineering protocol.\n- **`software.change`**: only makes sense for Git, pull requests or code.\n",
        terms.len()
    );
    for category in Category::ALL {
        let _ = write!(
            out,
            "\n## {}\n\nCategory {}.\n\n",
            category_title(category),
            code(category.as_str())
        );
        let in_category: Vec<_> = terms
            .iter()
            .filter(|term| term.category == category)
            .collect();
        if in_category.is_empty() {
            out.push_str("*No term in this category.*\n");
            continue;
        }
        out.push_str("| Term | Marking | Meaning |\n|---|---|---|\n");
        for term in in_category {
            let marking = match term.marking {
                Marking::Core => "core",
                Marking::SoftwareChange => "software.change",
            };
            let _ = writeln!(
                out,
                "| {} | {} | {} |",
                code(term.id),
                code(marking),
                text(term.meaning)
            );
        }
    }
    out
}

struct Shipped {
    file: String,
    title: String,
    revision: u64,
    description: Option<String>,
    counts: String,
}

fn index_page(shipped: &[Shipped]) -> String {
    let mut out = format!(
        "---\n{HEADER}\ntitle: \"Protocols\"\ndescription: \"The protocols ELS ships, each rendered from its compiled Canon protocol/1 document.\"\ncustom_edit_url: null\n---\n\nEvery protocol ELS ships is a Canon `protocol/1` document at `protocols/<name>/<major>.yaml`. Each page here is generated from the document's compiled form, `canon-ir/1`, so it shows what Canon understood.\n\n"
    );
    if shipped.is_empty() {
        out.push_str("**No protocol is shipped yet.** This page lists each protocol as soon as its document lands in `protocols/`. The engineering [vocabulary](../vocabulary.md) ships today.\n");
    } else {
        out.push_str("| Protocol | Revision | Description | Declares |\n|---|---|---|---|\n");
        for protocol in shipped {
            let _ = writeln!(
                out,
                "| [{}](./{}) | {} | {} | {} |",
                code(&protocol.title),
                protocol
                    .file
                    .strip_prefix("protocols/")
                    .unwrap_or(&protocol.file),
                protocol.revision,
                protocol
                    .description
                    .as_deref()
                    .map(|d| text(d.trim()))
                    .unwrap_or_default(),
                protocol.counts
            );
        }
    }
    out
}

/// Every generated file, by path below `website/docs/`.
pub fn render(inputs: &Inputs) -> Result<BTreeMap<String, String>> {
    let vocabulary = Vocabulary::from_yaml(&inputs.vocabulary)
        .map_err(|error| anyhow!("protocols/vocabulary.yaml: {error}"))?;
    let mut files = BTreeMap::new();
    let mut shipped = Vec::new();
    for source in &inputs.protocols {
        let document = parse(&source.text).map_err(|error| anyhow!("{}: {error}", source.path))?;
        let ir = compile(&document).map_err(|problems| {
            let problems: Vec<String> = problems.iter().map(ToString::to_string).collect();
            anyhow!("{}: {}", source.path, problems.join("; "))
        })?;
        files.insert(source.file(), protocol::page(&ir, source));
        files
            .entry(format!("protocols/{}/_category_.yml", source.name))
            .or_insert_with(|| {
                format!(
                    "{HEADER}\nlabel: {}\n",
                    yaml_string(ir.protocol.id.as_str())
                )
            });
        shipped.push(Shipped {
            file: source.file(),
            title: format!("{}/{}", ir.protocol.id, source.major),
            revision: ir.protocol.revision,
            description: ir.protocol.description.clone(),
            counts: format!(
                "{} claims, {} actions, {} outcomes",
                ir.claims.len(),
                ir.actions.len(),
                ir.outcomes.len()
            ),
        });
    }
    files.insert("protocols/index.md".to_owned(), index_page(&shipped));
    files.insert(
        "protocols/_category_.yml".to_owned(),
        format!("{HEADER}\nlabel: Protocols\nposition: 2\n"),
    );
    files.insert("vocabulary.md".to_owned(), vocabulary_page(&vocabulary));
    Ok(files)
}

fn is_generated(content: &str) -> bool {
    content.lines().take(2).any(|line| line == HEADER)
}

/// Writes `files` into `docs` and removes generated files that are no longer produced; with
/// `check`, changes nothing and fails when the tree differs. Returns what differed.
pub fn apply(docs: &Path, files: &BTreeMap<String, String>, check: bool) -> Result<Vec<String>> {
    let mut drift = Vec::new();
    for (path, content) in files {
        let target = docs.join(path);
        match fs::read_to_string(&target) {
            Ok(existing) if existing == *content => continue,
            Ok(_) => drift.push(format!("{path} differs from a fresh render")),
            Err(_) => drift.push(format!("{path} is missing")),
        }
        if !check {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&target, content).with_context(|| format!("writing {}", target.display()))?;
        }
    }
    let mut pending = vec!["protocols".to_owned()];
    while let Some(dir) = pending.pop() {
        let Ok(entries) = fs::read_dir(docs.join(&dir)) else {
            continue;
        };
        for entry in entries {
            let entry = entry?;
            let Ok(name) = entry.file_name().into_string() else {
                continue;
            };
            let path = format!("{dir}/{name}");
            if entry.file_type()?.is_dir() {
                pending.push(path);
                continue;
            }
            if files.contains_key(&path) {
                continue;
            }
            if fs::read_to_string(entry.path()).is_ok_and(|content| is_generated(&content)) {
                drift.push(format!("{path} is generated but no longer produced"));
                if !check {
                    fs::remove_file(entry.path())?;
                }
            }
        }
        if !check && dir != "protocols" {
            let _ = fs::remove_dir(docs.join(&dir));
        }
    }
    if check && !drift.is_empty() {
        bail!(
            "generated documentation is stale; run `els-docs generate`:\n  {}",
            drift.join("\n  ")
        );
    }
    Ok(drift)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph;

    const VOCABULARY: &str = include_str!("../../../protocols/vocabulary.yaml");
    /// Canon's `fixtures/investigation/protocol.yaml` at canon `32ae0d0`, copied byte for byte.
    /// It is a test input only and is never shipped under `protocols/`.
    const INVESTIGATION: &str = include_str!("../tests/fixtures/investigation.yaml");

    fn root() -> &'static Path {
        Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."))
    }

    fn with(protocols: Vec<Source>) -> Result<BTreeMap<String, String>> {
        render(&Inputs {
            vocabulary: VOCABULARY.to_owned(),
            protocols,
        })
    }

    fn source(name: &str, text: &str) -> Source {
        Source {
            name: name.to_owned(),
            major: 1,
            path: format!("protocols/{name}/1.yaml"),
            text: text.to_owned(),
        }
    }

    fn scratch(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("els-docs-test-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("scratch directory");
        dir
    }

    #[test]
    fn the_investigation_fixture_page_shows_every_claim_action_and_outcome() {
        let files = with(vec![source("investigation", INVESTIGATION)]).expect("renders");
        let page = &files["protocols/investigation/1.md"];
        let ir = compile(&parse(INVESTIGATION).expect("parses")).expect("compiles");
        let mermaid = page
            .split("```mermaid\n")
            .nth(1)
            .and_then(|tail| tail.split("```").next())
            .expect("a mermaid block");
        let groups: [Vec<String>; 4] = [
            ir.claims.keys().map(ToString::to_string).collect(),
            ir.actions.keys().map(ToString::to_string).collect(),
            ir.outcomes.keys().map(ToString::to_string).collect(),
            ir.evidence_kinds.keys().map(ToString::to_string).collect(),
        ];
        assert_eq!(groups.each_ref().map(Vec::len), [1, 2, 1, 2]);
        for id in groups.iter().flatten() {
            assert!(
                page.contains(&format!("\n| `{id}` |")),
                "{id} has no table row"
            );
            assert!(
                mermaid.contains(&format!("[\"{id}\"]")),
                "{id} has no graph node"
            );
        }
        assert!(page.starts_with(&format!(
            "---\n{HEADER}\nid: \"1\"\ntitle: \"investigation/1\"\n"
        )));
        assert!(page.contains("slug: /protocols/investigation/1\n"));
        assert!(page.contains("all of (evidence `falsification_attempt` with result `survived`; evidence `supporting_observation`)"));
        assert_eq!(graph::edges(&ir).len(), 5);
        assert_eq!(mermaid.matches(" --> ").count(), 5);
        assert!(mermaid.contains("a0 --> e0") && mermaid.contains("a1 --> e1"));
        assert!(page.contains("*This protocol declares no obligations.*"));
        let index = &files["protocols/index.md"];
        assert!(index.contains("[`investigation/1`](./investigation/1.md)"));
        assert!(!index.contains("No protocol is shipped yet"));
    }

    #[test]
    fn with_no_protocol_the_index_says_none_is_shipped() {
        let files = with(Vec::new()).expect("renders");
        assert!(files["protocols/index.md"].contains("**No protocol is shipped yet.**"));
        assert_eq!(
            files.keys().map(String::as_str).collect::<Vec<_>>(),
            [
                "protocols/_category_.yml",
                "protocols/index.md",
                "vocabulary.md"
            ]
        );
        assert!(files.values().all(|content| is_generated(content)));
    }

    #[test]
    fn the_vocabulary_page_lists_every_term() {
        let files = with(Vec::new()).expect("renders");
        let page = &files["vocabulary.md"];
        let vocabulary = Vocabulary::from_yaml(VOCABULARY).expect("vocabulary");
        assert_eq!(vocabulary.terms().len(), 35);
        for term in vocabulary.terms() {
            assert!(
                page.contains(&format!("\n| `{}` |", term.id)),
                "{}",
                term.id
            );
        }
    }

    #[test]
    fn an_invalid_protocol_is_refused_with_its_path() {
        let broken = INVESTIGATION.replace("evidence: supporting_observation", "evidence: hearsay");
        let error = with(vec![source("investigation", &broken)])
            .expect_err("refused")
            .to_string();
        assert!(
            error.starts_with("protocols/investigation/1.yaml:"),
            "{error}"
        );
        assert!(error.contains("hearsay"), "{error}");
    }

    #[test]
    fn document_text_cannot_inject_markup() {
        let hostile = INVESTIGATION.replace(
            "description: Try to refute the explanation.",
            "description: \"<script>x</script> | [link](/elsewhere)\"",
        );
        let files = with(vec![source("investigation", &hostile)]).expect("escaped, not refused");
        let page = &files["protocols/investigation/1.md"];
        assert!(page.contains("\\<script\\>x\\</script\\> \\| \\[link\\](/elsewhere)"));
    }

    #[test]
    fn the_committed_docs_are_fresh_renders() {
        let files = render(&Inputs::read(root()).expect("reads protocols/")).expect("renders");
        apply(&root().join(DOCS), &files, true).expect("committed pages match a fresh render");
    }

    #[test]
    fn check_fails_on_a_hand_edit_or_a_stale_page_and_generate_repairs_both() {
        let docs = scratch("drift");
        let files = with(vec![source("investigation", INVESTIGATION)]).expect("renders");
        apply(&docs, &files, false).expect("writes");
        assert!(apply(&docs, &files, true).expect("fresh").is_empty());

        let page = docs.join("protocols/investigation/1.md");
        let edited = fs::read_to_string(&page).expect("page") + "\nhand edit\n";
        fs::write(&page, edited).expect("edit");
        let error = apply(&docs, &files, true).expect_err("drift").to_string();
        assert!(
            error.contains("protocols/investigation/1.md differs"),
            "{error}"
        );

        let without = with(Vec::new()).expect("renders");
        let error = apply(&docs, &without, true).expect_err("stale").to_string();
        assert!(
            error.contains("protocols/investigation/1.md is generated but no longer produced"),
            "{error}"
        );
        apply(&docs, &without, false).expect("repairs");
        assert!(!page.exists());
        assert!(apply(&docs, &without, true).expect("fresh").is_empty());
        let _ = fs::remove_dir_all(&docs);
    }
}
