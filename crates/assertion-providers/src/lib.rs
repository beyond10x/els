//! Bounded, explicitly registered engineering observations.
use serde_json::Value;
use std::{collections::BTreeMap, path::PathBuf};
#[derive(Debug, PartialEq)]
pub enum Collected {
    Known(Value),
    Unavailable(String),
}
#[derive(Debug)]
pub struct ProviderError(pub String);
pub struct ProviderContext {
    pub root: PathBuf,
}
impl ProviderContext {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }
}
pub fn collect(
    _operation: &str,
    _args: &BTreeMap<String, Value>,
    _context: &ProviderContext,
) -> Result<Collected, ProviderError> {
    Ok(Collected::Unavailable("provider is not implemented".into()))
}
