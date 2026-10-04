//! Adversary cases (pass 2) for `story:vocabulary-yaml-source`: what `Vocabulary::to_yaml` writes.
//!
//! The module doc promises that `to_yaml` "writes one" — a vocabulary document that
//! `from_yaml` reads — and Atlas ADR 0077 point 3 lets the Rust API *produce* the released data.
//! A produced document that reads back to a different meaning is a vocabulary that changed in
//! transit, with no error anywhere.

use b10x_els::vocabulary::{Category, Marking, Vocabulary};

/// Meanings a person can write in a YAML document and `from_yaml` accepts (each is non-blank),
/// chosen for the characters YAML treats specially.
const MEANINGS: &[&str] = &[
    "a: b # not a comment",
    "  leading spaces",
    "trailing spaces  ",
    "\ttab first",
    "line one\nline two",
    "line one\nline two\n",
    "line one\n\n\nline two\n\n",
    "\nleading newline",
    "  indented first line\nsecond",
    "first\n  indented second",
    "trailing space \nnext line",
    "crlf\r\nline",
    "cr\ronly",
    "nel\u{85}next",
    "ls\u{2028}next",
    "ps\u{2029}next",
    "bom\u{feff}inside",
    "Übergabe — 引き継ぎ ✓ 😀",
    "- dash first",
    "--- document marker",
    "... end marker",
    "#hash first",
    "& anchor",
    "* alias",
    "! tag",
    "% directive",
    "@ at",
    "` tick",
    "| pipe",
    "> fold",
    "{ brace",
    "[ bracket",
    "'single",
    "\"double",
    "null",
    "~",
    "true",
    "123",
    "0x1F",
    ".inf",
    "nul\u{0}char",
    "bell\u{7}",
    "esc\u{1b}[0m",
    "\u{a0}no-break space first",
    "line\n\ttab-indented",
    "a meaning long enough to pass the emitter's preferred line width of eighty columns, with  two  spaces  between  some  of  its  words  near  the  end",
    "a long first line that is longer than eighty columns so that a literal block may be folded by someone\nand a second line",
];

#[test]
fn adversary2_yaml_to_yaml_preserves_every_meaning_byte_for_byte() {
    let mut lost = Vec::new();
    let exotic = [
        "nonchar\u{fffe}\u{ffff}",
        "last plane \u{10ffff}",
        "unit separator\u{1f}",
        "c1 control\u{9b}",
        "surrogate-adjacent \u{d7ff}\u{e000}",
        "a: \"mixed\" 'quotes' and \\ backslash",
        "trailing tab\t",
        "\u{3000}ideographic space first",
    ];
    for (i, meaning) in MEANINGS.iter().chain(exotic.iter()).enumerate() {
        let id = format!(
            "term_{}",
            "abcdefghijklmnopqrstuvwxyz".as_bytes()[i % 26] as char
        )
        .repeat(1 + i / 26);
        let built =
            Vocabulary::default().with_term(&id, Category::EvidenceKind, Marking::Core, meaning);
        let written = built.to_yaml();
        match Vocabulary::from_yaml(&written) {
            Ok(back) => {
                let got = back.lookup(&id).map(|t| t.meaning.to_owned());
                if got.as_deref() != Ok(*meaning) {
                    lost.push(format!(
                        "{meaning:?} came back as {got:?}; written:\n{written}"
                    ));
                } else if back.to_yaml() != written {
                    lost.push(format!("{meaning:?}: to_yaml is not a fixed point"));
                }
            }
            Err(e) => lost.push(format!(
                "{meaning:?} does not read back: {e}; written:\n{written}"
            )),
        }
    }
    assert!(
        lost.is_empty(),
        "{} of {} meanings change through to_yaml then from_yaml:\n{}",
        lost.len(),
        MEANINGS.len() + exotic.len(),
        lost.join("\n")
    );
}

/// `with_term` checks nothing, by its doc; whatever it was given, `to_yaml` writes a document
/// (it does not panic) and `from_yaml` refuses the bad id by name rather than reading another one.
#[test]
fn adversary2_yaml_to_yaml_of_unchecked_ids_is_refused_by_name() {
    for id in [
        "a\nb",
        "a: b",
        "- x",
        "&anchor",
        "*alias",
        "null",
        "~",
        "",
        " padded ",
        "a\r\nb",
        "dotted.\u{85}id",
        "tab\tid",
    ] {
        let built =
            Vocabulary::default().with_term(id, Category::EvidenceKind, Marking::Core, "meaning");
        let written = built.to_yaml();
        match Vocabulary::from_yaml(&written) {
            Ok(back) => {
                let ids: Vec<String> = back.terms().iter().map(|t| t.id.to_owned()).collect();
                assert_eq!(ids, [id], "`{id:?}` read back as another id:\n{written}");
            }
            Err(e) => {
                let quoted = format!("`{id}`");
                assert!(
                    e.to_string().contains(&quoted),
                    "`{id:?}` refused without naming it: {e}\n{written}"
                );
            }
        }
    }
}
