use anyhow::{Result, ensure};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
};

/// Hash names, entry kinds and bytes, including dirty and untracked source. Build/cache and
/// retained evidence directories are excluded and must not be used as source inputs.
pub fn identity(root: &Path, output: Option<&Path>) -> Result<String> {
    identity_with_exclusions(root, output, true)
}
pub fn full_identity(root: &Path) -> Result<String> {
    identity_with_exclusions(root, None, false)
}
fn identity_with_exclusions(
    root: &Path,
    output: Option<&Path>,
    exclude_caches: bool,
) -> Result<String> {
    let mut hash = Sha256::new();
    let mut pending = vec![PathBuf::new()];
    let mut files = 0usize;
    let mut bytes = 0u64;
    while let Some(relative) = pending.pop() {
        ensure!(
            relative.components().count() <= 64,
            "source snapshot depth exceeds 64"
        );
        let path = if relative.as_os_str().is_empty() {
            root.to_owned()
        } else {
            root.join(&relative)
        };
        if output == Some(path.as_path()) {
            continue;
        }
        if exclude_caches
            && (relative.components().any(|c| {
                matches!(
                    c.as_os_str().to_str(),
                    Some(".git" | "target" | "node_modules" | ".docusaurus")
                )
            }) || relative.starts_with(".engineering/assertions")
                || relative.starts_with(".engineering/drafts")
                || relative.starts_with("website/build"))
        {
            continue;
        }
        files += 1;
        ensure!(files <= 100_000, "source snapshot exceeds 100000 entries");
        let name = relative
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("non-UTF8 source path"))?;
        hash.update((name.len() as u64).to_be_bytes());
        hash.update(name.as_bytes());
        let meta = fs::symlink_metadata(&path)?;
        if meta.file_type().is_symlink() {
            hash.update(b"symlink");
            let target = fs::read_link(&path)?;
            let target = target
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("non-UTF8 symlink"))?;
            hash.update((target.len() as u64).to_be_bytes());
            hash.update(target.as_bytes());
        } else if meta.is_dir() {
            hash.update(b"directory");
            let mut children = fs::read_dir(&path)?
                .map(|e| e.map(|e| relative.join(e.file_name())))
                .collect::<std::io::Result<Vec<_>>>()?;
            children.sort();
            pending.extend(children.into_iter().rev());
        } else if meta.is_file() {
            hash.update(b"file");
            hash.update(meta.len().to_be_bytes());
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                hash.update(meta.permissions().mode().to_be_bytes());
            }
            bytes = bytes
                .checked_add(meta.len())
                .ok_or_else(|| anyhow::anyhow!("snapshot size overflow"))?;
            ensure!(bytes <= 1024 * 1024 * 1024, "source snapshot exceeds 1 GiB");
            let mut file = fs::File::open(&path)?;
            let mut buffer = [0u8; 65536];
            let mut read = 0u64;
            loop {
                let n = file.read(&mut buffer)?;
                if n == 0 {
                    break;
                }
                read += n as u64;
                ensure!(read <= meta.len(), "source grew during snapshot");
                hash.update(&buffer[..n]);
            }
            ensure!(read == meta.len(), "source shrank during snapshot");
        } else {
            anyhow::bail!("special source file {}", relative.display());
        }
    }
    Ok(format!("{:x}", hash.finalize()))
}
