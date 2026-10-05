use crate::{Arguments, Collected, ProviderContext, ProviderError, arg, known};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    io::{self, Read},
    path::{Component, Path, PathBuf},
};

/// Resolve paths below the root, rejecting traversal and symlink escapes, including absent leaves.
pub(crate) fn resolve(root: &Path, source: &str) -> Result<PathBuf, ProviderError> {
    let path = Path::new(source);
    if path.is_absolute()
        || path
            .components()
            .any(|c| matches!(c, Component::ParentDir | Component::Prefix(_)))
    {
        return Err(ProviderError::Invalid(
            "path must remain below repository root".into(),
        ));
    }
    let root = fs::canonicalize(root)?;
    let target = root.join(path);
    let mut ancestor = target.as_path();
    loop {
        match fs::canonicalize(ancestor) {
            Ok(found) => {
                if !found.starts_with(&root) {
                    return Err(ProviderError::Invalid(
                        "path escapes repository root".into(),
                    ));
                }
                break;
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                ancestor = ancestor
                    .parent()
                    .ok_or_else(|| ProviderError::Invalid("unresolvable path".into()))?
            }
            Err(e) => return Err(e.into()),
        }
    }
    Ok(target)
}
pub(crate) fn read(context: &ProviderContext, path: &str) -> Result<Vec<u8>, ProviderError> {
    let path = resolve(&context.root, path)?;
    if !fs::metadata(&path)?.is_file() {
        return Err(ProviderError::Invalid("expected regular file".into()));
    }
    let file = fs::File::open(path)?;
    let mut bytes = Vec::new();
    file.take(context.max_output_bytes as u64 + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() > context.max_output_bytes {
        return Err(ProviderError::Invalid(
            "file exceeds observation byte limit".into(),
        ));
    }
    Ok(bytes)
}
pub(crate) fn document(
    context: &ProviderContext,
    path: &str,
    format: &str,
) -> Result<Value, ProviderError> {
    let bytes = read(context, path)?;
    match format {
        "json" => Ok(serde_json::from_slice(&bytes)?),
        "yaml" => {
            let value: serde_yaml_ng::Value = serde_yaml_ng::from_slice(&bytes)
                .map_err(|e| ProviderError::Invalid(e.to_string()))?;
            reject_yaml_floats(&value)?;
            Ok(serde_json::to_value(value)?)
        }
        _ => Err(ProviderError::Invalid(
            "document format must be json or yaml".into(),
        )),
    }
}
fn reject_yaml_floats(value: &serde_yaml_ng::Value) -> Result<(), ProviderError> {
    use serde_yaml_ng::Value as Y;
    match value {
        Y::Number(number) if number.is_f64() => return Err(ProviderError::Invalid(
            "YAML floating-point values are not admitted; use JSON for exact decimal observations"
                .into(),
        )),
        Y::Sequence(values) => {
            for value in values {
                reject_yaml_floats(value)?;
            }
        }
        Y::Mapping(values) => {
            for (key, value) in values {
                reject_yaml_floats(key)?;
                reject_yaml_floats(value)?;
            }
        }
        Y::Tagged(_) => {
            return Err(ProviderError::Invalid(
                "YAML tags are not admitted as JSON document values".into(),
            ));
        }
        _ => {}
    }
    Ok(())
}
pub(super) fn collect(
    operation: &str,
    args: &Arguments,
    context: &ProviderContext,
) -> Result<Collected, ProviderError> {
    match operation {
        "fs.file.exists" | "fs.dir.exists" => {
            let path = resolve(&context.root, arg(args, "path")?)?;
            match fs::metadata(path) {
                Ok(m) => known(if operation == "fs.file.exists" {
                    m.is_file()
                } else {
                    m.is_dir()
                }),
                Err(e) if e.kind() == io::ErrorKind::NotFound => known(false),
                Err(e) => Ok(Collected::Unavailable(e.to_string())),
            }
        }
        "text.contains" => {
            let bytes = read(context, arg(args, "path")?)?;
            let text =
                std::str::from_utf8(&bytes).map_err(|e| ProviderError::Invalid(e.to_string()))?;
            known(text.contains(arg(args, "needle")?))
        }
        "json.read" | "yaml.read" => known(document(
            context,
            arg(args, "path")?,
            if operation == "json.read" {
                "json"
            } else {
                "yaml"
            },
        )?),
        "document.get" => {
            let value = document(
                context,
                arg(args, "path")?,
                args.get("format").and_then(Value::as_str).unwrap_or("json"),
            )?;
            let pointer = arg(args, "pointer")?;
            if !pointer.is_empty() && !pointer.starts_with('/') {
                return Err(ProviderError::Invalid(
                    "pointer must be an RFC 6901 JSON pointer".into(),
                ));
            }
            known(value.pointer(pointer).cloned().unwrap_or(Value::Null))
        }
        "document.validates" => {
            let value = document(context, arg(args, "path")?, "json")?;
            let schema = document(context, arg(args, "schema")?, "json")?;
            reject_remote_refs(&schema)?;
            let validator = jsonschema::validator_for(&schema)
                .map_err(|e| ProviderError::Invalid(e.to_string()))?;
            known(validator.is_valid(&value))
        }
        "generated.matches" => {
            let a = tree(context, arg(args, "expected")?)?;
            let b = tree(context, arg(args, "actual")?)?;
            known(a == b)
        }
        _ => Err(ProviderError::Invalid("unknown file operation".into())),
    }
}
fn reject_remote_refs(value: &Value) -> Result<(), ProviderError> {
    match value {
        Value::Object(m) => {
            for (k, v) in m {
                if matches!(k.as_str(), "$ref" | "$dynamicRef")
                    && v.as_str().is_some_and(|s| !s.starts_with('#'))
                {
                    return Err(ProviderError::Invalid(
                        "only document-local schema references are admitted".into(),
                    ));
                }
                reject_remote_refs(v)?;
            }
        }
        Value::Array(a) => {
            for v in a {
                reject_remote_refs(v)?;
            }
        }
        _ => {}
    }
    Ok(())
}
fn tree(
    context: &ProviderContext,
    source: &str,
) -> Result<BTreeMap<PathBuf, Value>, ProviderError> {
    let base = resolve(&context.root, source)?;
    if !base.is_dir() {
        return Err(ProviderError::Invalid(
            "generated comparison requires directories".into(),
        ));
    }
    let mut todo = vec![base.clone()];
    let mut entries = BTreeMap::new();
    let mut budget = context.max_output_bytes;
    while let Some(path) = todo.pop() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let kind = entry.file_type()?;
            if entries.len() + todo.len() > 10000 {
                return Err(ProviderError::Invalid(
                    "generated comparison entry budget exceeded".into(),
                ));
            }
            if kind.is_symlink() {
                return Err(ProviderError::Invalid(
                    "generated comparison refuses symlinks".into(),
                ));
            }
            let path = entry.path();
            let relative = path
                .strip_prefix(&base)
                .map_err(|e| ProviderError::Invalid(e.to_string()))?
                .to_path_buf();
            if kind.is_dir() {
                entries.insert(relative, json!({"directory":true}));
                todo.push(path);
            } else if kind.is_file() {
                let file = fs::File::open(&path)?;
                let mut bytes = Vec::new();
                file.take(budget as u64 + 1).read_to_end(&mut bytes)?;
                if bytes.len() > budget {
                    return Err(ProviderError::Invalid(
                        "generated comparison byte budget exceeded".into(),
                    ));
                }
                budget -= bytes.len();
                use sha2::{Digest, Sha256};
                entries.insert(relative, json!(format!("{:x}", Sha256::digest(bytes))));
            } else {
                return Err(ProviderError::Invalid(
                    "generated comparison requires regular files".into(),
                ));
            }
        }
    }
    Ok(entries)
}
