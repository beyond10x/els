#![forbid(unsafe_code)]

//! `canon-engineering`: the engineering protocols command line. `canon-engineering protocols list`
//! names the built-in protocols and `canon-engineering protocols show <name>@<major>` prints one as
//! released. A malformed argument is a usage error (exit 2); a well-formed one that names no
//! built-in is a refusal (exit 1).

mod builtin_name;

use std::io::Write as _;
use std::process::ExitCode;

use canon_engineering::registry;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "canon-engineering",
    about = "The engineering protocols, built on Canon"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// The built-in protocols.
    #[command(subcommand)]
    Protocols(Protocols),
}

#[derive(Subcommand)]
enum Protocols {
    /// Print each built-in protocol as `<name>@<major>`, one per line.
    List,
    /// Print a built-in protocol's YAML exactly as released.
    Show {
        /// The protocol, as `<name>@<major>`, for example `software-change@1`.
        #[arg(value_parser = protocol_reference)]
        protocol: Reference,
    },
}

/// A well-formed `<name>@<major>`, which may still name no built-in.
#[derive(Clone)]
struct Reference {
    name: String,
    major: u32,
}

/// Parses `<name>@<major>` by the rule `build.rs` embeds files by; anything else is a usage error.
fn protocol_reference(text: &str) -> Result<Reference, String> {
    let (name, major) = builtin_name::reference(text)?;
    Ok(Reference {
        name: name.to_owned(),
        major,
    })
}

fn main() -> ExitCode {
    match Cli::parse().command {
        Command::Protocols(Protocols::List) => {
            let mut out = String::new();
            for (name, major) in registry::list() {
                out.push_str(&format!("{name}@{major}\n"));
            }
            print(out.as_bytes())
        }
        Command::Protocols(Protocols::Show { protocol }) => {
            match registry::get(&protocol.name, protocol.major) {
                Ok(builtin) => print(builtin.yaml.as_bytes()),
                Err(error) => {
                    eprintln!("canon-engineering: {error}");
                    ExitCode::FAILURE
                }
            }
        }
    }
}

/// Writes `bytes` to standard output unchanged.
fn print(bytes: &[u8]) -> ExitCode {
    let mut stdout = std::io::stdout().lock();
    match stdout.write_all(bytes).and_then(|()| stdout.flush()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("canon-engineering: writing standard output: {error}");
            ExitCode::FAILURE
        }
    }
}
