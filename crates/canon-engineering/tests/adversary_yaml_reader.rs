//! Adversary cases for `story:vocabulary-yaml-source`: what `Vocabulary::from_yaml` accepts that
//! the released vocabulary's own rules (`tests/vocabulary.rs`, the module doc of
//! `src/vocabulary.rs`) say a vocabulary cannot hold.

use canon_engineering::vocabulary::{Category, Marking, Vocabulary};

fn document(entries: &[(&str, &str, &str, &str)]) -> String {
    let mut text = String::from("format: vocabulary/1\nterms:\n");
    for (id, category, marking, meaning) in entries {
        text.push_str(&format!(
            "  - id: {id}\n    category: {category}\n    marking: {marking}\n    meaning: {meaning}\n"
        ));
    }
    text
}

/// The module doc spells claim and action ids `<subject>.<predicate>` and every other id as one
/// snake_case token, and `tests/vocabulary.rs` holds the released file to it. The reader is the
/// public way to read "a vocabulary document given as text", so a document breaking that rule
/// must be refused, not read into a `Term` whose `claim_id()` hands Canon an id that is no
/// claim id.
#[test]
fn adversary_yaml_reader_refuses_ids_the_vocabulary_cannot_spell() {
    for (id, category) in [
        ("servicehealthy", "claim_id"),
        ("Service.Healthy", "claim_id"),
        ("logs.search.all", "action_id"),
        ("test.result", "evidence_kind"),
        ("Test_Result", "evidence_kind"),
        ("\"two words\"", "artifact_kind"),
        ("\"\"", "outcome_id"),
    ] {
        let text = document(&[(id, category, "core", "a term spelled against the rule")]);
        let read = Vocabulary::from_yaml(&text);
        assert!(
            read.is_err(),
            "`{id}` as {category} was read: {:?}",
            read.map(|v| v
                .terms()
                .iter()
                .map(|t| (t.id.to_owned(), t.claim_id().map(|c| c.as_str().to_owned())))
                .collect::<Vec<_>>())
        );
    }
}

/// Two entries whose ids differ only by surrounding whitespace are one name to a reader of the
/// file and two to `lookup`; the duplicate check must not let them both in.
#[test]
fn adversary_yaml_reader_refuses_an_id_padded_into_a_second_entry() {
    let text = document(&[
        ("intent", "artifact_kind", "core", "the first"),
        ("\"intent \"", "artifact_kind", "core", "the second"),
    ]);
    let read = Vocabulary::from_yaml(&text);
    assert!(
        read.is_err(),
        "both read: {:?}",
        read.map(|v| v
            .terms()
            .iter()
            .map(|t| t.id.to_owned())
            .collect::<Vec<_>>())
    );
}

/// `tests/vocabulary.rs` refuses a released term without a meaning; the reader must not turn an
/// empty or null `meaning` into a term.
#[test]
fn adversary_yaml_reader_refuses_a_term_without_a_meaning() {
    for meaning in ["\"\"", "", "~", "null"] {
        let text = document(&[("plan", "artifact_kind", "core", meaning)]);
        let read = Vocabulary::from_yaml(&text);
        assert!(
            read.is_err(),
            "`meaning: {meaning}` was read as {:?}",
            read.map(|v| v
                .terms()
                .iter()
                .map(|t| t.meaning.to_owned())
                .collect::<Vec<_>>())
        );
    }
}

/// Probe: meanings carrying YAML syntax come back exactly as the document spells them.
#[test]
fn adversary_yaml_reader_reads_yaml_special_meanings_verbatim() {
    let text = "format: vocabulary/1
terms:
  - id: a
    category: evidence_kind
    marking: core
    meaning: \"a: b # not a comment\"
  - id: b
    category: evidence_kind
    marking: core
    meaning: 'it''s \"quoted\"'
  - id: c
    category: evidence_kind
    marking: core
    meaning: >-
      folded over
      two lines
  - id: d
    category: evidence_kind
    marking: core
    meaning: |
      literal
      block
  - id: e
    category: evidence_kind
    marking: software.change
    meaning: Übergabe — 引き継ぎ ✓
  - id: f
    category: evidence_kind
    marking: core
    meaning: plain text # a trailing comment
";
    let read = Vocabulary::from_yaml(text).expect("the document reads");
    let meanings: Vec<&str> = read.terms().iter().map(|t| t.meaning).collect();
    assert_eq!(
        meanings,
        [
            "a: b # not a comment",
            "it's \"quoted\"",
            "folded over two lines",
            "literal\nblock\n",
            "Übergabe — 引き継ぎ ✓",
            "plain text",
        ]
    );
    assert_eq!(read.lookup("e").unwrap().marking, Marking::SoftwareChange);
    assert_eq!(read.lookup("a").unwrap().category, Category::EvidenceKind);
}

/// Probe: category and marking are matched exactly, not case-folded; a second YAML document and
/// a missing `terms` list are refused.
#[test]
fn adversary_yaml_reader_refuses_case_variants_and_stray_documents() {
    for (category, marking) in [
        ("Evidence_Kind", "core"),
        ("EVIDENCE_KIND", "core"),
        ("EvidenceKind", "core"),
        ("evidence-kind", "core"),
        ("evidence_kind", "Core"),
        ("evidence_kind", "Software.Change"),
        ("evidence_kind", "software_change"),
        ("evidence_kind", "SoftwareChange"),
    ] {
        let text = document(&[("x", category, marking, "m")]);
        assert!(
            Vocabulary::from_yaml(&text).is_err(),
            "{category}/{marking} was read"
        );
    }
    let one = document(&[("x", "evidence_kind", "core", "m")]);
    assert!(Vocabulary::from_yaml(&format!("{one}---\n{one}")).is_err());
    assert!(Vocabulary::from_yaml("format: vocabulary/1\n").is_err());
}
