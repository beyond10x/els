//! The built site's route inventory, `b10x-project-routes/v1` at `.well-known/b10x-routes.json`:
//! every page route under [`BASE_URL`] with its rendered element IDs, stamped with the commit the
//! site manifest carries. Website reads it to list the project and check
//! links into it.
//!
//! The site is Docusaurus with `trailingSlash: false` on `@beyond10x/docs-system`. Docusaurus writes
//! a page at `x.html` and a directory index at `x/index.html`; docs-system writes a copy of every
//! `x.html` at `x/index.html` marked `<!-- b10x-trailing-slash-copy -->`, and the client-redirects
//! plugin writes `http-equiv="refresh"` pages. Every route is listed in its `/x/` form. A redirect,
//! a trailing-slash copy and `404.html` are no page; two real pages answering one route are refused,
//! and so is a site with no landing route. Both the anchors and the redirect signals are read from
//! start tags and comments only, never from script, style, attribute values or text.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail, ensure};
use serde_json::json;

use crate::manifest::BASE_URL;

/// The comment docs-system writes into its trailing-slash copy of a page.
const TRAILING_SLASH_COPY: &str = "b10x-trailing-slash-copy";

/// Elements whose content the tokenizer reads as text, or (`template`) parses into an inert
/// fragment: nothing in them is a rendered element.
const OPAQUE: [&str; 10] = [
    "script", "style", "template", "title", "textarea", "xmp", "iframe", "noembed", "noframes",
    "noscript",
];

/// What one built file's markup says. Only start tags and comments are markup: nothing inside an
/// [`OPAQUE`] element, a comment, a bogus comment, an end tag, an attribute value or text is read.
#[derive(Default)]
struct Markup {
    /// The `id` attribute values of its start tags.
    ids: BTreeSet<String>,
    /// It is no page of its own: a `<meta http-equiv=refresh>` start tag (quoted or not, any case),
    /// or docs-system's trailing-slash copy comment.
    redirect: bool,
}

/// A tag: its lower-case name, its attributes (lower-case names, raw values) and its length.
struct Tag<'a> {
    name: String,
    attributes: Vec<(String, &'a str)>,
    len: usize,
}

impl Tag<'_> {
    /// The first value of the attribute `name`, as HTML reads a repeated attribute.
    fn attribute(&self, name: &str) -> Option<&str> {
        self.attributes
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| *value)
    }
}

/// The tag `html` opens with after an opener of `open` bytes (`<` for a start tag, `</` for an end
/// tag), or `None` when no letter follows the opener.
fn tag(html: &str, open: usize) -> Option<Tag<'_>> {
    let bytes = html.as_bytes();
    if !bytes.get(open)?.is_ascii_alphabetic() {
        return None;
    }
    let ends_name = |b: u8| b.is_ascii_whitespace() || b == b'/' || b == b'>' || b == b'=';
    let mut at = open;
    while at < bytes.len() && !ends_name(bytes[at]) {
        at += 1;
    }
    let name = html[open..at].to_ascii_lowercase();
    let mut attributes = Vec::new();
    loop {
        while at < bytes.len() && (bytes[at].is_ascii_whitespace() || bytes[at] == b'/') {
            at += 1;
        }
        match bytes.get(at) {
            None => break,
            Some(b'>') => {
                at += 1;
                break;
            }
            Some(_) => {}
        }
        let start = at;
        at += 1;
        while at < bytes.len() && !ends_name(bytes[at]) {
            at += 1;
        }
        let key = html[start..at].to_ascii_lowercase();
        while at < bytes.len() && bytes[at].is_ascii_whitespace() {
            at += 1;
        }
        let mut value = "";
        if bytes.get(at) == Some(&b'=') {
            at += 1;
            while at < bytes.len() && bytes[at].is_ascii_whitespace() {
                at += 1;
            }
            if let Some(&quote @ (b'"' | b'\'')) = bytes.get(at) {
                let from = at + 1;
                let end = html[from..]
                    .find(char::from(quote))
                    .map_or(bytes.len(), |n| from + n);
                value = &html[from..end];
                at = (end + 1).min(bytes.len());
            } else {
                let from = at;
                while at < bytes.len() && !bytes[at].is_ascii_whitespace() && bytes[at] != b'>' {
                    at += 1;
                }
                value = &html[from..at];
            }
        }
        attributes.push((key, value));
    }
    Some(Tag {
        name,
        attributes,
        len: at,
    })
}

/// A comment's text and the bytes it takes after `<!--`: it ends at `-->` or `--!>`, and `<!-->`
/// and `<!--->` are complete empty comments.
fn comment(body: &str) -> (&str, usize) {
    if body.starts_with('>') {
        return ("", 1);
    }
    if body.starts_with("->") {
        return ("", 2);
    }
    [("-->", 3), ("--!>", 4)]
        .into_iter()
        .filter_map(|(close, len)| body.find(close).map(|end| (end, end + len)))
        .min()
        .map_or((body, body.len()), |(end, len)| (&body[..end], len))
}

/// Where the content of the [`OPAQUE`] element `name` that starts at `from` ends: at `</name`
/// followed by whitespace, `/` or `>`, in any case; at the end of `lower` when there is none.
fn content_end(lower: &str, from: usize, name: &str) -> usize {
    let close = format!("</{name}");
    let mut at = from;
    while let Some(found) = lower[at..].find(&close) {
        let end = at + found;
        at = end + close.len();
        if lower
            .as_bytes()
            .get(at)
            .is_some_and(|&b| b.is_ascii_whitespace() || b == b'/' || b == b'>')
        {
            return end;
        }
    }
    lower.len()
}

/// Reads the markup of one built file.
fn scan(html: &str) -> Markup {
    let lower = html.to_ascii_lowercase();
    let mut markup = Markup::default();
    let mut at = 0;
    while let Some(found) = html[at..].find('<') {
        at += found;
        let rest = &html[at..];
        if let Some(body) = rest.strip_prefix("<!--") {
            let (text, len) = comment(body);
            if text.trim() == TRAILING_SLASH_COPY {
                markup.redirect = true;
            }
            at += 4 + len;
            continue;
        }
        let bytes = rest.as_bytes();
        let end_tag = bytes.get(1) == Some(&b'/');
        let bogus = match bytes.get(1) {
            Some(b'!' | b'?') => true,
            Some(b'/') => bytes
                .get(2)
                .is_some_and(|&b| !b.is_ascii_alphabetic() && b != b'>'),
            _ => false,
        };
        if bogus {
            at = rest.find('>').map_or(html.len(), |end| at + end + 1);
            continue;
        }
        let Some(tag) = tag(rest, if end_tag { 2 } else { 1 }) else {
            at += 1;
            continue;
        };
        at += tag.len;
        if end_tag {
            continue;
        }
        if let Some(id) = tag.attribute("id").filter(|id| !id.is_empty()) {
            markup.ids.insert(id.to_owned());
        }
        if tag.name == "meta"
            && tag
                .attribute("http-equiv")
                .is_some_and(|value| value.trim().eq_ignore_ascii_case("refresh"))
        {
            markup.redirect = true;
        }
        if OPAQUE.contains(&tag.name.as_str()) {
            at = content_end(&lower, at, &tag.name);
        }
    }
    markup
}

/// The route a built file answers, or `None` for a file that is no page.
fn route(relative: &str) -> Option<String> {
    if relative == "404.html" {
        return None;
    }
    if relative == "index.html" {
        return Some(BASE_URL.to_owned());
    }
    if let Some(dir) = relative.strip_suffix("/index.html") {
        return Some(format!("{BASE_URL}{dir}/"));
    }
    relative
        .strip_suffix(".html")
        .map(|page| format!("{BASE_URL}{page}/"))
}

/// A real page: the file that answers a route, and its anchors.
struct Page {
    file: String,
    anchors: BTreeSet<String>,
}

fn pages(site: &Path, dir: &Path, out: &mut BTreeMap<String, Page>) -> Result<()> {
    let mut entries = fs::read_dir(dir)
        .with_context(|| format!("reading {}", dir.display()))?
        .collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(fs::DirEntry::file_name);
    for entry in entries {
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            pages(site, &path, out)?;
            continue;
        }
        let relative = path
            .strip_prefix(site)?
            .to_str()
            .context("non-UTF-8 path in the built site")?
            .replace('\\', "/");
        let Some(route) = route(&relative) else {
            continue;
        };
        let html =
            fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
        let markup = scan(&html);
        if markup.redirect {
            continue;
        }
        if let Some(anchor) = markup
            .ids
            .iter()
            .find(|id| id.chars().any(char::is_control))
        {
            bail!("{route} ({relative}) has an anchor holding a control character: {anchor:?}");
        }
        if let Some(first) = out.get(&route) {
            bail!(
                "two built files answer {route}: {} and {relative}",
                first.file
            );
        }
        out.insert(
            route,
            Page {
                file: relative,
                anchors: markup.ids,
            },
        );
    }
    Ok(())
}

/// The inventory of the built site at `site`, built from `commit`, as written to
/// `.well-known/b10x-routes.json`; the caller validates `commit`.
pub fn inventory(site: &Path, commit: &str) -> Result<(String, usize)> {
    let mut found = BTreeMap::new();
    pages(site, site, &mut found)?;
    ensure!(
        found.contains_key(BASE_URL),
        "{} has no landing route {BASE_URL}",
        site.display()
    );
    let count = found.len();
    let routes: Vec<_> = found
        .into_iter()
        .map(|(path, page)| json!({"path": path, "anchors": page.anchors}))
        .collect();
    let document = json!({
        "schema": "b10x-project-routes/v1",
        "repository": "engineering-protocols",
        "commit": commit,
        "baseUrl": BASE_URL,
        "routes": routes,
    });
    Ok((serde_json::to_string_pretty(&document)? + "\n", count))
}
