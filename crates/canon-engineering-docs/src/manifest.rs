//! The built site's own declaration of what it is, which the project-site publisher refuses on
//! mismatch: `.well-known/b10x-site.json`, written with the route inventory (`crate::routes`).

use std::fs;
use std::path::Path;

use anyhow::{Context, Result, ensure};
use serde_json::json;

/// Where the site is served.
pub const BASE_URL: &str = "/engineering-protocols/";

/// The manifest for a site built from `commit`.
pub fn manifest(commit: &str) -> Result<String> {
    ensure!(
        commit.len() == 40
            && commit
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            && commit != "0".repeat(40),
        "commit must be a nonzero full lowercase Git revision"
    );
    let manifest = json!({
        "schema": "b10x-project-site/v1",
        "repository": "engineering-protocols",
        "commit": commit,
        "baseUrl": BASE_URL,
    });
    Ok(serde_json::to_string_pretty(&manifest)? + "\n")
}

/// Writes the route inventory from the same commit, `.nojekyll` and, last, the manifest into a built
/// site, and returns the number of routes. A `.well-known` that is not a directory of the site is
/// refused before anything is touched. An earlier run's outputs go first (a link is removed, never
/// followed), so a site the inventory refuses, or whose writes fail part-way, has no manifest.
pub fn write(site: &Path, commit: &str) -> Result<usize> {
    let manifest = manifest(commit)?;
    ensure!(
        site.join("index.html").is_file(),
        "{} holds no built site (no index.html)",
        site.display()
    );
    let well_known = site.join(".well-known");
    if let Ok(meta) = fs::symlink_metadata(&well_known) {
        ensure!(
            meta.is_dir(),
            "{} is not a directory of the site",
            well_known.display()
        );
    }
    for earlier in [
        ".well-known/b10x-site.json",
        ".well-known/b10x-routes.json",
        ".nojekyll",
    ] {
        let path = site.join(earlier);
        if fs::symlink_metadata(&path).is_ok_and(|meta| !meta.is_dir()) {
            fs::remove_file(&path).with_context(|| format!("removing the earlier {earlier}"))?;
        }
    }
    let (routes, count) = crate::routes::inventory(site, commit)?;
    fs::create_dir_all(site.join(".well-known"))?;
    fs::write(site.join(".well-known/b10x-routes.json"), routes)
        .context("writing .well-known/b10x-routes.json")?;
    fs::write(site.join(".nojekyll"), "").context("writing .nojekyll")?;
    fs::write(site.join(".well-known/b10x-site.json"), manifest)
        .context("writing .well-known/b10x-site.json")?;
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_manifest_binds_the_commit_and_base() {
        let commit = "fef049470cf8fe4ca34ad0bf3694e6c25cdc9a74";
        let value: serde_json::Value =
            serde_json::from_str(&manifest(commit).expect("manifest")).expect("json");
        assert_eq!(
            value,
            json!({"schema":"b10x-project-site/v1","repository":"engineering-protocols","commit":commit,"baseUrl":"/engineering-protocols/"})
        );
        assert!(manifest(&"0".repeat(40)).is_err());
        assert!(manifest("fef0494").is_err());
        assert!(manifest(&commit.to_uppercase()).is_err());
    }
}
