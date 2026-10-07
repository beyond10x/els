//! Adversary, pass 2, `story:project-routes`: the start-tag and comment scanner (`scan` in
//! `src/routes.rs`) and the removal of an earlier manifest (`manifest::write`).
//!
//! The scanner's contract (`src/routes.rs:11-12`, `:26-27`): anchors are the `id` values of start
//! tags, and nothing inside `<script>`, `<style>`, a comment, an attribute value or text is read.
//! Acceptance 2: anchors are the page's *rendered* `id` attribute values. Each case below is markup
//! where an HTML parser creates no element with the id the scanner reports, or the reverse. The
//! expected anchors are what the WHATWG tokenizer yields for the same bytes.
//!
//! Each case builds its own fixture under `CARGO_TARGET_TMPDIR`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::Value;

const COMMIT: &str = "fef049470cf8fe4ca34ad0bf3694e6c25cdc9a74";
const LANDING: &str = "/engineering-protocols/";

fn site_manifest(site: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_canon-engineering-docs"))
        .args([
            "site-manifest",
            "--site",
            site.to_str().expect("UTF-8 path"),
            "--commit",
            COMMIT,
        ])
        .output()
        .expect("run canon-engineering-docs site-manifest")
}

fn scratch(case: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("adversary-p2-{}-{case}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("mkdir scratch");
    dir
}

/// A site holding one landing page with `body`, run through `site-manifest`; the landing anchors.
fn landing_anchors(case: &str, body: &str) -> Vec<String> {
    let site = scratch(case);
    fs::write(
        site.join("index.html"),
        format!("<!doctype html><html><head><title>t</title></head><body>{body}</body></html>"),
    )
    .expect("write index.html");
    let out = site_manifest(&site);
    assert_eq!(out.status.code(), Some(0), "site-manifest: {out:?}");
    let text = fs::read_to_string(site.join(".well-known/b10x-routes.json")).expect("inventory");
    let inventory: Value = serde_json::from_str(&text).expect("json");
    let anchors = inventory["routes"]
        .as_array()
        .expect("routes")
        .iter()
        .find(|route| route["path"] == LANDING)
        .expect("landing route")["anchors"]
        .as_array()
        .expect("anchors")
        .iter()
        .map(|a| a.as_str().expect("anchor").to_owned())
        .collect();
    let _ = fs::remove_dir_all(&site);
    anchors
}

/// A script ends only at `</script` followed by whitespace, `/` or `>` (WHATWG "script data end
/// tag name state": an appropriate end tag). `</scripty>` inside a string does not end it, so the
/// `<b id="leak">` after it is still script text. `scan` ends the skip at the first `</script`.
#[test]
fn adversary_p2_script_does_not_end_at_a_longer_end_tag_name() {
    assert_eq!(
        landing_anchors(
            "script-end",
            r#"<script>var a="</scripty>";var b='<b id="leak">';</script><h2 id=real>R</h2>"#,
        ),
        ["real"],
        "nothing inside <script> is read"
    );
}

/// `<!-->` is a complete, empty comment (WHATWG "abrupt-closing-of-empty-comment"); the heading
/// after it is rendered. `scan` searches for `-->` after `<!--` and swallows the heading up to the
/// next comment's end.
#[test]
fn adversary_p2_an_abruptly_closed_empty_comment_hides_nothing() {
    assert_eq!(
        landing_anchors("empty-comment", "<!--><h2 id=real>R</h2><!-- later -->"),
        ["real"],
        "the element after <!--> is rendered"
    );
}

/// Outside SVG and MathML, `<![CDATA[` opens a bogus comment that ends at the first `>` (WHATWG
/// "cdata-in-html-content"), so `<a id="cdata">` inside it creates no element. `scan` steps over
/// `<!` one byte and reads the `<a …>` as a start tag.
#[test]
fn adversary_p2_markup_inside_a_bogus_comment_is_no_element() {
    assert_eq!(
        landing_anchors("cdata", r#"<![CDATA[<a id="cdata">]]><h2 id=real>R</h2>"#),
        ["real"],
        "a bogus comment is a comment"
    );
}

/// `<template>` content is an inert document fragment: its elements are not in the document, so
/// `#inert` neither scrolls nor matches `getElementById`. It is no rendered id (acceptance 2).
#[test]
fn adversary_p2_ids_inside_a_template_are_not_rendered() {
    assert_eq!(
        landing_anchors(
            "template",
            r#"<template><h2 id="inert">I</h2></template><h2 id=real>R</h2>"#
        ),
        ["real"],
        "template content is not rendered"
    );
}

/// Website's `IndependentSites::validate` refuses the whole independent site when any anchor holds
/// a control character (`tools/website/src/routes.rs:168-172`). `site-manifest` either refuses the
/// page here or leaves the anchor out; it does not publish an inventory its consumer refuses.
#[test]
fn adversary_p2_no_anchor_carries_a_control_character() {
    let site = scratch("control");
    fs::write(
        site.join("index.html"),
        "<!doctype html><html><body><h2 id=\"tab\there\">T</h2><h2 id=real>R</h2></body></html>",
    )
    .expect("write index.html");
    let out = site_manifest(&site);
    if out.status.code() == Some(0) {
        let text =
            fs::read_to_string(site.join(".well-known/b10x-routes.json")).expect("inventory");
        let inventory: Value = serde_json::from_str(&text).expect("json");
        for route in inventory["routes"].as_array().expect("routes") {
            for anchor in route["anchors"].as_array().expect("anchors") {
                let anchor = anchor.as_str().expect("anchor");
                assert!(
                    !anchor.chars().any(char::is_control),
                    "anchor {anchor:?} of {} is refused by Website",
                    route["path"]
                );
            }
        }
    }
    let _ = fs::remove_dir_all(&site);
}

/// The earlier-manifest removal reaches through a symlinked `.well-known`: on a site the inventory
/// refuses, it deletes `b10x-site.json` in the directory the link points at, outside the site.
#[cfg(unix)]
#[test]
fn adversary_p2_removing_an_earlier_manifest_deletes_nothing_outside_the_site() {
    let root = scratch("symlink");
    let outside = root.join("outside");
    fs::create_dir_all(&outside).expect("mkdir outside");
    fs::write(outside.join("b10x-site.json"), "not this site's\n").expect("write outside file");
    let site = root.join("site");
    fs::create_dir_all(&site).expect("mkdir site");
    fs::write(
        site.join("index.html"),
        r#"<!doctype html><html><head><meta http-equiv="refresh" content="0; url=/engineering-protocols/docs"></head></html>"#,
    )
    .expect("write index.html");
    std::os::unix::fs::symlink(&outside, site.join(".well-known")).expect("symlink .well-known");
    let out = site_manifest(&site);
    assert_ne!(out.status.code(), Some(0), "no landing route: {out:?}");
    assert_eq!(
        fs::read_to_string(outside.join("b10x-site.json"))
            .ok()
            .as_deref(),
        Some("not this site's\n"),
        "a file outside the site is untouched"
    );
    let _ = fs::remove_dir_all(&root);
}
