#![forbid(unsafe_code)]

//! Generates every derived page of the ELS documentation site into the Docusaurus docs tree
//! (`website/docs/`): one page per protocol document, compiled by Canon, the protocols index and
//! the vocabulary. The site build itself is Docusaurus; nothing here touches the network.

mod generate;
mod graph;
mod manifest;
mod markdown;
mod protocol;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about = "Generate the derived pages of the ELS documentation site")]
struct Cli {
    #[command(subcommand)]
    command: Action,
}

#[derive(Subcommand)]
enum Action {
    /// Render protocols and the vocabulary into `<root>/website/docs/`.
    Generate {
        /// The repository root that holds `protocols/` and `website/docs/`.
        #[arg(long, default_value = ".")]
        root: PathBuf,
        /// Change nothing; fail when a generated page differs from a fresh render.
        #[arg(long)]
        check: bool,
    },
    /// Declare what a built site is: write `.well-known/b10x-site.json` (and `.nojekyll`) into it.
    SiteManifest {
        /// The built site, normally `website/build`.
        #[arg(long)]
        site: PathBuf,
        /// The full Git revision the site was built from.
        #[arg(long)]
        commit: String,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Action::Generate { root, check } => {
            let inputs = generate::Inputs::read(&root)?;
            let files = generate::render(&inputs)?;
            let drift = generate::apply(&root.join(generate::DOCS), &files, check)?;
            if check {
                println!(
                    "els-docs: {} generated files fresh ({} protocols)",
                    files.len(),
                    inputs.protocols.len()
                );
            } else {
                println!(
                    "els-docs: {} generated files, {} updated ({} protocols)",
                    files.len(),
                    drift.len(),
                    inputs.protocols.len()
                );
            }
        }
        Action::SiteManifest { site, commit } => {
            manifest::write(&site, &commit)?;
            println!("els-docs: {} declared as els at {commit}", site.display());
        }
    }
    Ok(())
}
