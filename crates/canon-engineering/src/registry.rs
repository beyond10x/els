//! The built-in engineering protocols, embedded in the crate when it is built.
//!
//! A built-in is every `protocols/<name>/<major>.yaml` in the repository (`build.rs`); adding a
//! protocol file needs no Rust edit, and the registry holds no protocol definition of its own.
//! [`list`] names the built-ins and [`get`] returns one exactly as released, together with Canon's
//! validated `protocol/1` model of it.

use std::fmt;

use b10x_canon::model::{ParseError, Protocol};
use b10x_canon::validate::Problem;

/// Every built-in as (name, major, YAML as released), sorted by name and major.
static BUILTINS: &[(&str, u32, &str)] = include!(concat!(env!("OUT_DIR"), "/builtins.rs"));

/// A built-in protocol as released, with Canon's validated model of it.
#[derive(Debug, Clone)]
pub struct Builtin {
    /// The protocol directory name, `<name>` in `protocols/<name>/<major>.yaml`.
    pub name: &'static str,
    /// The major version, `<major>` in `protocols/<name>/<major>.yaml`.
    pub major: u32,
    /// The document byte for byte as released.
    pub yaml: &'static str,
    /// Canon's model of [`Builtin::yaml`], which Canon has validated.
    pub model: Protocol,
}

/// Why [`get`] refused.
#[derive(Debug)]
pub enum Error {
    /// No built-in has this name.
    UnknownName { name: String, major: u32 },
    /// A built-in has this name, but not at this major.
    UnknownMajor { name: String, major: u32 },
    /// Canon cannot parse the embedded document.
    Parse {
        name: &'static str,
        major: u32,
        error: ParseError,
    },
    /// Canon parses the embedded document and finds it invalid.
    Invalid {
        name: &'static str,
        major: u32,
        problems: Vec<Problem>,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnknownName { name, major } => {
                write!(f, "{name}@{major}: no built-in protocol is named `{name}`")
            }
            Error::UnknownMajor { name, major } => {
                write!(
                    f,
                    "{name}@{major}: built-in `{name}` has no major version {major}"
                )
            }
            Error::Parse { name, major, error } => {
                write!(f, "{name}@{major}: Canon cannot parse it: {error}")
            }
            Error::Invalid {
                name,
                major,
                problems,
            } => {
                write!(f, "{name}@{major}: Canon finds it invalid:")?;
                for problem in problems {
                    write!(f, "\n  {problem}")?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for Error {}

/// Every built-in, as (name, major), sorted by name and major.
pub fn list() -> Vec<(&'static str, u32)> {
    BUILTINS
        .iter()
        .map(|(name, major, _)| (*name, *major))
        .collect()
}

/// The built-in `name@major`, as released and validated by Canon. Refuses an unknown name or major,
/// and a document Canon cannot parse or finds invalid.
pub fn get(name: &str, major: u32) -> Result<Builtin, Error> {
    let Some((name, major, yaml)) = BUILTINS
        .iter()
        .find(|(known, version, _)| *known == name && *version == major)
        .copied()
    else {
        let name = name.to_owned();
        return Err(if BUILTINS.iter().any(|(known, _, _)| *known == name) {
            Error::UnknownMajor { name, major }
        } else {
            Error::UnknownName { name, major }
        });
    };
    let model =
        b10x_canon::model::parse(yaml).map_err(|error| Error::Parse { name, major, error })?;
    b10x_canon::validate::validate(&model).map_err(|problems| Error::Invalid {
        name,
        major,
        problems,
    })?;
    Ok(Builtin {
        name,
        major,
        yaml,
        model,
    })
}
