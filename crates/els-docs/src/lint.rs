//! The source lint the docs-system admonition guard enforces at build time, run earlier: a line
//! matching `^:::[a-z]+ +\S` is a raw admonition opener with a space-separated title. Docusaurus
//! renders it as text, so the box silently disappears; the title belongs in brackets,
//! `:::note[Title]`. Lines inside fenced code blocks are examples, not openers.

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

/// Whether `line` matches `^:::[a-z]+ +\S`.
pub fn is_raw_admonition(line: &str) -> bool {
    let Some(rest) = line.strip_prefix(":::") else {
        return false;
    };
    let kind = rest.bytes().take_while(u8::is_ascii_lowercase).count();
    if kind == 0 {
        return false;
    }
    let after = &rest[kind..];
    let spaces = after.bytes().take_while(|byte| *byte == b' ').count();
    spaces > 0
        && after[spaces..]
            .chars()
            .next()
            .is_some_and(|c| !c.is_whitespace())
}

/// Every raw admonition opener in `source`, as one-based line numbers, outside fenced code blocks.
pub fn raw_admonitions(source: &str) -> Vec<usize> {
    let mut fence: Option<&str> = None;
    let mut found = Vec::new();
    for (index, line) in source.lines().enumerate() {
        let trimmed = line.trim_start();
        let marker = ["```", "~~~"]
            .into_iter()
            .find(|marker| trimmed.starts_with(marker));
        match (fence, marker) {
            (None, Some(marker)) => fence = Some(marker),
            (Some(open), Some(marker)) if open == marker => fence = None,
            (None, None) if is_raw_admonition(line) => found.push(index + 1),
            _ => {}
        }
    }
    found
}

/// Every raw admonition opener in the `.md` and `.mdx` files under `docs`, as `path:line`.
pub fn check_tree(docs: &Path) -> Result<Vec<String>> {
    let mut pending = vec![docs.to_path_buf()];
    let mut found = Vec::new();
    while let Some(dir) = pending.pop() {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries {
            let path = entry?.path();
            if path.is_dir() {
                pending.push(path);
                continue;
            }
            if !matches!(
                path.extension().and_then(|e| e.to_str()),
                Some("md" | "mdx")
            ) {
                continue;
            }
            let source =
                fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
            let shown = path
                .strip_prefix(docs)
                .unwrap_or(&path)
                .display()
                .to_string();
            found.extend(
                raw_admonitions(&source)
                    .into_iter()
                    .map(|line| format!("{shown}:{line}")),
            );
        }
    }
    found.sort();
    Ok(found)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_a_spaced_title_after_a_lowercase_kind_matches() {
        for raw in [":::note Title", ":::shipped  Two spaces", ":::tip x"] {
            assert!(is_raw_admonition(raw), "{raw}");
        }
        for fine in [
            ":::note[Title]",
            ":::note",
            ":::",
            ":::Note Title",
            ":::note ",
            ":::note \tTab",
            " :::note Title",
            "text :::note Title",
        ] {
            assert!(!is_raw_admonition(fine), "{fine}");
        }
    }

    #[test]
    fn fenced_examples_are_not_openers() {
        let source = "a\n:::note Title\n```md\n:::note Inside\n```\n~~~\n:::tip Also inside\n~~~\n:::tip Out\n";
        assert_eq!(raw_admonitions(source), [2, 9]);
    }

    #[test]
    fn the_committed_docs_have_no_raw_admonition() {
        let docs = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../website/docs"));
        assert_eq!(check_tree(docs).expect("reads"), Vec::<String>::new());
    }
}
