//! The rule for a built-in protocol's name and major, in one place. `build.rs` includes this file
//! to decide which `protocols/<name>/<major>.yaml` it embeds, and the `canon-engineering` binary
//! includes it to parse the `<name>@<major>` it is given. `canon-engineering-docs` includes it for
//! the names of the protocol directories it renders.

/// A protocol name: a lowercase ASCII letter, then lowercase ASCII letters, digits and hyphens.
pub fn is_name(name: &str) -> bool {
    name.starts_with(|c: char| c.is_ascii_lowercase())
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// A major version: a positive decimal that fits a `u32`, without a sign or leading zeros.
pub fn major(text: &str) -> Option<u32> {
    let major: u32 = text.parse().ok()?;
    (major > 0 && major.to_string() == text).then_some(major)
}

/// `<name>@<major>`, split into its name and major, or why it is not one.
pub fn reference(text: &str) -> Result<(&str, u32), String> {
    let (name, version) = text
        .split_once('@')
        .ok_or_else(|| "a protocol is named <name>@<major>".to_owned())?;
    if !is_name(name) {
        return Err(format!(
            "`{name}` is not a protocol name: lowercase letters, digits and hyphens"
        ));
    }
    let major = major(version).ok_or_else(|| {
        format!("`{version}` is not a major version: a positive decimal without leading zeros")
    })?;
    Ok((name, major))
}
