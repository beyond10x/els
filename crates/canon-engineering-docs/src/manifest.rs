//! The built site's own declaration of what it is, which the project-site publisher refuses on
//! mismatch: `.well-known/b10x-site.json`.

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

/// Writes the manifest and `.nojekyll` into a built site.
pub fn write(site: &Path, commit: &str) -> Result<()> {
    let manifest = manifest(commit)?;
    ensure!(
        site.join("index.html").is_file(),
        "{} holds no built site (no index.html)",
        site.display()
    );
    fs::create_dir_all(site.join(".well-known"))?;
    fs::write(site.join(".well-known/b10x-site.json"), manifest)
        .context("writing .well-known/b10x-site.json")?;
    fs::write(site.join(".nojekyll"), "").context("writing .nojekyll")?;
    Ok(())
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
