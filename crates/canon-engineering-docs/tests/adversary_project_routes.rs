//! Adversary, pass 1, `story:project-routes`: cases the implementor's suite does not state.
//!
//! Acceptance 2 says `anchors` are the page's *rendered* `id` attribute values, and acceptance 3 says
//! only `404.html`, a client redirect and a marked trailing-slash copy produce no route. The scanner
//! in `src/routes.rs` matches substrings (` id=`, `http-equiv="refresh"`) anywhere in the file, so
//! text inside a `<script>` or a `<code>` element is read as markup. The minified Docusaurus build
//! emits `"` raw in text nodes (`website/build/docs/guides/assertions.html`:
//! `token string" style=color:#b9d9ee>"login"<`), which is what the fixtures below reproduce.
//!
//! Each case builds its own fixture under `CARGO_TARGET_TMPDIR`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::Value;

const COMMIT: &str = "fef049470cf8fe4ca34ad0bf3694e6c25cdc9a74";

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

fn fresh_site(case: &str) -> PathBuf {
    let site = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "adversary-project-routes-{}-{case}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&site);
    fs::create_dir_all(&site).expect("mkdir site");
    site
}

fn put(site: &Path, relative: &str, content: &str) {
    let path = site.join(relative);
    fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
    fs::write(path, content).expect("write fixture file");
}

/// The anchors the inventory lists for `path`, or `None` when the route is absent.
fn anchors(site: &Path, path: &str) -> Option<Vec<String>> {
    let text = fs::read_to_string(site.join(".well-known/b10x-routes.json")).expect("inventory");
    let inventory: Value = serde_json::from_str(&text).expect("json");
    inventory["routes"]
        .as_array()
        .expect("routes")
        .iter()
        .find(|route| route["path"] == path)
        .map(|route| {
            route["anchors"]
                .as_array()
                .expect("anchors")
                .iter()
                .map(|a| a.as_str().expect("anchor").to_owned())
                .collect()
        })
}

/// The real landing page (`website/build/index.html`, built at `5d28957`) carries Docusaurus's
/// base-URL banner script. Its two `id="…"` occurrences are characters inside a JavaScript string
/// literal that is assigned to `innerHTML` only when the site loaded under the wrong base URL; on the
/// served site no such element exists. They are not rendered `id` attribute values (acceptance 2),
/// so `/engineering-protocols/` must not list them.
#[test]
fn adversary_landing_anchors_exclude_ids_inside_a_script() {
    let site = fresh_site("script-ids");
    // The script element is copied verbatim from website/build/index.html.
    put(
        &site,
        "index.html",
        r#"<!doctype html><html lang=en dir=ltr><head><script data-rh=true>document.addEventListener("DOMContentLoaded",function(){void 0===window.docusaurus&&insertBanner()});function insertBanner(){var n=document.createElement("div");n.id="__docusaurus-base-url-issue-banner-container",n.innerHTML='\n<div id="__docusaurus-base-url-issue-banner" style="border: thick solid red; background-color: rgb(255, 230, 179); margin: 20px; padding: 20px; font-size: 20px;">\n   <p style="font-weight: bold; font-size: 30px;">Your Docusaurus site did not load properly.</p>\n   <p>A very common reason is a wrong site <a href="https://docusaurus.io/docs/docusaurus.config.js/#baseUrl" style="font-weight: bold;">baseUrl configuration</a>.</p>\n   <p>Current configured baseUrl = <span style="font-weight: bold; color: red;">/engineering-protocols/</span> </p>\n   <p>We suggest trying baseUrl = <span id="__docusaurus-base-url-issue-banner-suggestion-container" style="font-weight: bold; color: green;"></span></p>\n</div>\n',document.body.prepend(n);var e=document.getElementById("__docusaurus-base-url-issue-banner-suggestion-container"),o=window.location.pathname;e.textContent="/"===o.substr(-1)?o:o+"/"}</script></head><body><div id=__docusaurus><main id=__docusaurus_skipToContent_fallback><h1 id=b10x-hero-title>Engineering protocols</h1></main></div></body></html>"#,
    );
    let out = site_manifest(&site);
    assert_eq!(out.status.code(), Some(0), "site-manifest: {out:?}");
    assert_eq!(
        anchors(&site, "/engineering-protocols/").expect("landing route"),
        [
            "__docusaurus",
            "__docusaurus_skipToContent_fallback",
            "b10x-hero-title"
        ],
        "only rendered id attributes are anchors; the banner script's string literal is not markup"
    );
    let _ = fs::remove_dir_all(&site);
}

/// A page that shows markup in inline code, as the minified build renders it (`<` escaped, `"`
/// raw in text), has rendered no element with that id.
#[test]
fn adversary_an_id_shown_in_inline_code_is_no_anchor() {
    let site = fresh_site("inline-code-id");
    put(
        &site,
        "index.html",
        r#"<!doctype html><html><body><main id=main>Home</main></body></html>"#,
    );
    put(
        &site,
        "docs/anchors.html",
        r#"<!doctype html><html><body><h2 id=real>Real</h2><p>Write <code>&lt;h2 id="shown"&gt;</code> to give a heading an anchor.</p></body></html>"#,
    );
    let out = site_manifest(&site);
    assert_eq!(out.status.code(), Some(0), "site-manifest: {out:?}");
    assert_eq!(
        anchors(&site, "/engineering-protocols/docs/anchors/").expect("route listed"),
        ["real"],
        "text inside <code> is not an id attribute"
    );
    let _ = fs::remove_dir_all(&site);
}

/// A real page whose text mentions a refresh redirect in inline code is still a page (acceptance 3
/// names only `404.html`, a client redirect page and a marked copy as producing no route). Today
/// it vanishes from the inventory without an error.
#[test]
fn adversary_a_page_that_mentions_a_refresh_redirect_is_still_a_route() {
    let site = fresh_site("mentions-refresh");
    put(
        &site,
        "index.html",
        r#"<!doctype html><html><body><main id=main>Home</main></body></html>"#,
    );
    put(
        &site,
        "docs/redirects.html",
        r#"<!doctype html><html><head><title>Redirects</title></head><body><h1 id=redirects>Redirects</h1><p>The client-redirects plugin writes a page holding <code>&lt;meta http-equiv="refresh"&gt;</code> for every moved route.</p></body></html>"#,
    );
    put(
        &site,
        "docs/redirects/index.html",
        r#"<!doctype html><html><head><!-- b10x-trailing-slash-copy --><title>Redirects</title></head><body><h1 id=redirects>Redirects</h1><p>The client-redirects plugin writes a page holding <code>&lt;meta http-equiv="refresh"&gt;</code> for every moved route.</p></body></html>"#,
    );
    let out = site_manifest(&site);
    assert_eq!(out.status.code(), Some(0), "site-manifest: {out:?}");
    assert_eq!(
        anchors(&site, "/engineering-protocols/docs/redirects/"),
        Some(vec!["redirects".to_owned()]),
        "a real page that only mentions http-equiv=\"refresh\" in its text is listed"
    );
    let _ = fs::remove_dir_all(&site);
}

/// `manifest::write` writes `b10x-site.json` before `b10x-routes.json`. When the second write fails,
/// the site is left declared (the file the deploy step checks) with no inventory beside it. The
/// failure is forced by a directory standing where the inventory goes.
#[test]
fn adversary_a_failed_inventory_write_leaves_no_site_manifest() {
    let site = fresh_site("half-written");
    put(
        &site,
        "index.html",
        r#"<!doctype html><html><body><main id=main>Home</main></body></html>"#,
    );
    fs::create_dir_all(site.join(".well-known/b10x-routes.json")).expect("mkdir blocker");
    let out = site_manifest(&site);
    assert_ne!(
        out.status.code(),
        Some(0),
        "the write cannot succeed: {out:?}"
    );
    assert!(
        !site.join(".well-known/b10x-site.json").exists(),
        "a site-manifest that wrote no inventory leaves no site manifest either"
    );
    let _ = fs::remove_dir_all(&site);
}
