//! `story:project-routes`: `site-manifest` also writes `.well-known/b10x-routes.json`, the
//! `b10x-project-routes/v1` inventory of the built site, from the commit it writes into
//! `.well-known/b10x-site.json`.
//!
//! Each case builds its own fixture site under `CARGO_TARGET_TMPDIR`, shaped like a Docusaurus build
//! with `trailingSlash: false` on the pinned `@beyond10x/docs-system`: a landing page, a page
//! `x.html` with the marked copy `x/index.html` docs-system writes beside it, a directory index,
//! client redirect pages (quoted and unquoted `http-equiv`), `404.html` and non-page assets.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::{Value, json};

const COMMIT: &str = "fef049470cf8fe4ca34ad0bf3694e6c25cdc9a74";
const BASE: &str = "/engineering-protocols/";

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

/// A fresh, empty site directory for one case.
fn fresh_site(case: &str) -> PathBuf {
    let site = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("project-routes-{}-{case}", std::process::id()));
    let _ = fs::remove_dir_all(&site);
    fs::create_dir_all(&site).expect("mkdir site");
    site
}

fn put(site: &Path, relative: &str, content: &str) {
    let path = site.join(relative);
    fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
    fs::write(path, content).expect("write fixture file");
}

/// The fixture acceptance item 5 names.
fn fixture_site(case: &str) -> PathBuf {
    let site = fresh_site(case);
    put(
        &site,
        "index.html",
        r##"<!doctype html><html><body><main id="__docusaurus_skipToContent_fallback"><h2 id="b">B</h2><h2 id="a">A</h2><a href="#a" id="a">again</a><p id='single'>s</p><div id=bare class=x>x</div></main></body></html>"##,
    );
    put(
        &site,
        "docs/status.html",
        r#"<!doctype html><html><body><h2 id="shipped">S</h2><h2 id="known-limitations">K</h2></body></html>"#,
    );
    put(
        &site,
        "docs/status/index.html",
        r#"<!doctype html><html><head><!-- b10x-trailing-slash-copy --></head><body><h2 id="shipped">S</h2><h2 id="known-limitations">K</h2></body></html>"#,
    );
    put(
        &site,
        "docs/concepts/index.html",
        r#"<!doctype html><html><body><h1 id="concepts">C</h1></body></html>"#,
    );
    put(
        &site,
        "vocabulary.html",
        r#"<!doctype html><html><head><meta http-equiv="refresh" content="0; url=/engineering-protocols/docs/vocabulary"></head><body id="redirect"></body></html>"#,
    );
    put(
        &site,
        "protocols/index.html",
        r#"<!doctype html><html><head><meta http-equiv=refresh content="0; url=/engineering-protocols/docs/protocols"></head><body id=redirect></body></html>"#,
    );
    put(
        &site,
        "404.html",
        r#"<!doctype html><html><body><main id="lost">Not found</main></body></html>"#,
    );
    put(
        &site,
        "assets/js/main.js",
        "document.getElementById('x');\n",
    );
    put(&site, "sitemap.xml", "<urlset></urlset>\n");
    site
}

fn read_json(path: &Path) -> (String, Value) {
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let value = serde_json::from_str(&text).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    (text, value)
}

/// Acceptance 1–3: the inventory sits beside the site manifest, carries exactly the five keys and
/// the manifest's commit, and lists one route per real page: `x.html` and its marked copy give one
/// `/x/` route, a directory index gives its directory, and redirects and `404.html` give none.
#[test]
fn site_manifest_writes_the_route_inventory_beside_the_site_manifest() {
    let site = fixture_site("inventory");
    let out = site_manifest(&site);
    assert_eq!(out.status.code(), Some(0), "site-manifest: {out:?}");

    let (_, declared) = read_json(&site.join(".well-known/b10x-site.json"));
    let routes_path = site.join(".well-known/b10x-routes.json");
    let (text, inventory) = read_json(&routes_path);

    let keys: BTreeSet<&str> = inventory
        .as_object()
        .expect("inventory is an object")
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(
        keys,
        BTreeSet::from(["baseUrl", "commit", "repository", "routes", "schema"])
    );
    assert_eq!(inventory["schema"], "b10x-project-routes/v1");
    assert_eq!(inventory["repository"], "engineering-protocols");
    assert_eq!(inventory["baseUrl"], BASE);
    assert_eq!(inventory["commit"], declared["commit"]);
    assert_eq!(inventory["commit"], COMMIT);

    let routes = inventory["routes"].as_array().expect("routes is an array");
    let paths: Vec<&str> = routes
        .iter()
        .map(|route| route["path"].as_str().expect("path is a string"))
        .collect();
    let mut sorted = paths.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(paths, sorted, "paths are sorted and unique");
    assert!(paths.contains(&BASE), "the landing route is present");
    for route in routes {
        let object = route.as_object().expect("route is an object");
        assert_eq!(
            object.keys().map(String::as_str).collect::<BTreeSet<_>>(),
            BTreeSet::from(["anchors", "path"])
        );
        let path = route["path"].as_str().expect("path");
        assert!(path.starts_with(BASE) && path.ends_with('/'), "{path}");
        let anchors: Vec<&str> = route["anchors"]
            .as_array()
            .expect("anchors is an array")
            .iter()
            .map(|anchor| anchor.as_str().expect("anchor is a string"))
            .collect();
        let mut sorted = anchors.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(anchors, sorted, "anchors of {path} are sorted and unique");
    }

    assert_eq!(
        inventory,
        json!({
            "schema": "b10x-project-routes/v1",
            "repository": "engineering-protocols",
            "commit": COMMIT,
            "baseUrl": BASE,
            "routes": [
                {
                    "path": "/engineering-protocols/",
                    "anchors": ["__docusaurus_skipToContent_fallback", "a", "b", "bare", "single"],
                },
                {
                    "path": "/engineering-protocols/docs/concepts/",
                    "anchors": ["concepts"],
                },
                {
                    "path": "/engineering-protocols/docs/status/",
                    "anchors": ["known-limitations", "shipped"],
                },
            ],
        })
    );
    assert_eq!(
        text,
        serde_json::to_string_pretty(&inventory).expect("render") + "\n",
        "pretty-printed with a trailing newline"
    );

    // A second run over the same site, now holding `.well-known/`, writes the same bytes.
    let again = site_manifest(&site);
    assert_eq!(
        again.status.code(),
        Some(0),
        "second site-manifest: {again:?}"
    );
    assert_eq!(
        fs::read_to_string(&routes_path).expect("routes"),
        text,
        "deterministic"
    );
    let _ = fs::remove_dir_all(&site);
}

fn assert_refused(site: &Path, out: &Output, reason: &str) {
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_ne!(
        out.status.code(),
        Some(0),
        "site-manifest accepted: {out:?}"
    );
    assert!(
        stderr.contains(reason),
        "stderr names {reason:?}:\n{stderr}"
    );
    assert!(
        !site.join(".well-known/b10x-routes.json").exists(),
        "a refused site gets no route inventory"
    );
    assert!(
        !site.join(".well-known/b10x-site.json").exists(),
        "a refused site gets no site manifest either"
    );
}

/// Acceptance 4: an `index.html` that is only a client redirect leaves the site with no landing
/// route, and `site-manifest` refuses it.
#[test]
fn site_manifest_refuses_a_site_without_a_landing_route() {
    let site = fixture_site("no-landing");
    put(
        &site,
        "index.html",
        r#"<!doctype html><html><head><meta http-equiv="refresh" content="0; url=/engineering-protocols/docs"></head></html>"#,
    );
    let out = site_manifest(&site);
    assert_refused(&site, &out, "no landing route");
    let _ = fs::remove_dir_all(&site);
}

/// Acceptance 4: `x.html` and an unmarked `x/index.html` are two real pages answering one route,
/// and `site-manifest` refuses the site, naming the route and both files.
#[test]
fn site_manifest_refuses_two_built_files_answering_one_route() {
    let site = fixture_site("duplicate");
    put(
        &site,
        "docs/status/index.html",
        r#"<!doctype html><html><body><h2 id="other">O</h2></body></html>"#,
    );
    let out = site_manifest(&site);
    assert_refused(
        &site,
        &out,
        "two built files answer /engineering-protocols/docs/status/",
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("docs/status.html") && stderr.contains("docs/status/index.html"),
        "stderr names both files:\n{stderr}"
    );
    let _ = fs::remove_dir_all(&site);
}

/// A site declared by an earlier run and refused by this one is left undeclared: the earlier
/// `b10x-site.json` and `b10x-routes.json` go, so nothing stands that this run did not write.
#[test]
fn site_manifest_refusal_removes_an_earlier_site_manifest() {
    let site = fixture_site("stale");
    let first = site_manifest(&site);
    assert_eq!(first.status.code(), Some(0), "site-manifest: {first:?}");
    put(
        &site,
        "docs/status/index.html",
        r#"<!doctype html><html><body><h2 id="other">O</h2></body></html>"#,
    );
    let out = site_manifest(&site);
    assert_refused(
        &site,
        &out,
        "two built files answer /engineering-protocols/docs/status/",
    );
    let _ = fs::remove_dir_all(&site);
}

/// Markup signals come from parsed markup only: `id` attributes of start tags (any attribute-name
/// case) and `<meta http-equiv=refresh>` in any case are read, and nothing inside `<script>`,
/// `<style>`, a comment or another attribute's value is. A real page whose script carries the
/// trailing-slash marker and a refresh tag as strings is still a page.
#[test]
fn markup_signals_come_from_start_tags_outside_script_style_and_comments() {
    let site = fixture_site("markup");
    put(
        &site,
        "docs/markup.html",
        r##"<!doctype html><html><head><script>var s='<!-- b10x-trailing-slash-copy --><meta http-equiv="refresh" content="0"><div id="scripted">';</script><style>/* <p id="styled"> */</style><!-- <div id="commented"> --></head><body><h1 ID="Upper">U</h1><a title='see id="titled"' href=#x data-id=d id=link>l</a><br/><img id=img src=a.png /></body></html>"##,
    );
    put(
        &site,
        "moved.html",
        r#"<!doctype html><html><head><META HTTP-EQUIV=Refresh CONTENT="0; url=/engineering-protocols/docs/markup"></head><body id=moved></body></html>"#,
    );
    let out = site_manifest(&site);
    assert_eq!(out.status.code(), Some(0), "site-manifest: {out:?}");
    let (_, inventory) = read_json(&site.join(".well-known/b10x-routes.json"));
    let routes = inventory["routes"].as_array().expect("routes");
    let paths: Vec<&str> = routes
        .iter()
        .map(|route| route["path"].as_str().expect("path"))
        .collect();
    assert_eq!(
        paths,
        [
            "/engineering-protocols/",
            "/engineering-protocols/docs/concepts/",
            "/engineering-protocols/docs/markup/",
            "/engineering-protocols/docs/status/",
        ],
        "an upper-case META refresh is a redirect; a page whose script holds the marker is a page"
    );
    assert_eq!(routes[2]["anchors"], json!(["Upper", "img", "link"]));
    let _ = fs::remove_dir_all(&site);
}

/// Markup the HTML tokenizer reads as no element gives no anchor: the content of the raw-text and
/// escapable raw-text elements (`title`, `textarea`, `noscript`, `xmp`, `iframe`, `noembed`,
/// `noframes`), a bogus end tag (`</` and no letter, up to `>`) and an end tag's attribute value.
/// A comment closed by `--!>` ends there, so the element after it is rendered.
#[test]
fn markup_the_tokenizer_reads_as_no_element_gives_no_anchor() {
    let site = fixture_site("tokenizer");
    put(
        &site,
        "docs/text.html",
        r#"<!doctype html><html><head><title>A <b id=titled> title</title></head><body><textarea><b id=textarea></textarea><noscript><b id=noscript></noscript><xmp><b id=xmp></xmp><iframe src=x><b id=iframe></iframe><noembed><b id=noembed></noembed><noframes><b id=noframes></noframes></1<b id=bogusend></p title="<i id=inend>"><!--x--!><h2 id=after-bang>B</h2><h2 id=real>R</h2></body></html>"#,
    );
    let out = site_manifest(&site);
    assert_eq!(out.status.code(), Some(0), "site-manifest: {out:?}");
    let (_, inventory) = read_json(&site.join(".well-known/b10x-routes.json"));
    let page = inventory["routes"]
        .as_array()
        .expect("routes")
        .iter()
        .find(|route| route["path"] == "/engineering-protocols/docs/text/")
        .expect("the page is listed");
    assert_eq!(page["anchors"], json!(["after-bang", "real"]));
    let _ = fs::remove_dir_all(&site);
}

/// An anchor holding a control character makes Website refuse the whole site, so `site-manifest`
/// refuses it first, naming the route, the file and the anchor.
#[test]
fn site_manifest_refuses_an_anchor_holding_a_control_character() {
    let site = fixture_site("control");
    put(
        &site,
        "docs/status.html",
        "<!doctype html><html><body><h2 id=\"tab\there\">T</h2></body></html>",
    );
    let out = site_manifest(&site);
    assert_refused(&site, &out, "/engineering-protocols/docs/status/");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("docs/status.html") && stderr.contains(r#""tab\there""#),
        "stderr names the file and the anchor:\n{stderr}"
    );
    let _ = fs::remove_dir_all(&site);
}

/// `site-manifest` writes through no link that points out of the site: an earlier `.nojekyll`,
/// `b10x-site.json` or `b10x-routes.json` that is a symlink is replaced, and its target is left
/// as it was.
#[cfg(unix)]
#[test]
fn site_manifest_writes_through_no_link_out_of_the_site() {
    let site = fixture_site("links");
    let outside = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("project-routes-{}-outside", std::process::id()));
    let _ = fs::remove_dir_all(&outside);
    fs::create_dir_all(&outside).expect("mkdir outside");
    fs::create_dir_all(site.join(".well-known")).expect("mkdir .well-known");
    for (target, link) in [
        ("nojekyll", ".nojekyll"),
        ("b10x-site.json", ".well-known/b10x-site.json"),
        ("b10x-routes.json", ".well-known/b10x-routes.json"),
    ] {
        fs::write(outside.join(target), "keep\n").expect("write outside file");
        std::os::unix::fs::symlink(outside.join(target), site.join(link)).expect("symlink");
    }
    let out = site_manifest(&site);
    assert_eq!(out.status.code(), Some(0), "site-manifest: {out:?}");
    for target in ["nojekyll", "b10x-site.json", "b10x-routes.json"] {
        assert_eq!(
            fs::read_to_string(outside.join(target)).expect("outside file"),
            "keep\n",
            "{target} outside the site is untouched"
        );
    }
    for written in [
        ".nojekyll",
        ".well-known/b10x-site.json",
        ".well-known/b10x-routes.json",
    ] {
        assert!(
            fs::symlink_metadata(site.join(written))
                .expect("written")
                .is_file(),
            "{written} is a file of the site"
        );
    }
    let _ = fs::remove_dir_all(&site);
    let _ = fs::remove_dir_all(&outside);
}
