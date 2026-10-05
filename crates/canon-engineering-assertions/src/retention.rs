//! Retain evidence through directory descriptors; never follow a provider-created symlink.
use anyhow::{Result, ensure};
use std::{fs::File, io::Write, path::Path};

#[cfg(unix)]
pub fn write(root: &Path, relative: &Path, bytes: &[u8]) -> Result<()> {
    use rustix::fs::{AtFlags, Mode, OFlags, mkdirat, open, openat, renameat, unlinkat};
    let flags = OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC;
    let mut directory = open(root.canonicalize()?, flags, Mode::empty())?;
    let components = relative.components().collect::<Vec<_>>();
    ensure!(!components.is_empty(), "empty evidence destination");
    for component in &components[..components.len() - 1] {
        match mkdirat(&directory, component.as_os_str(), Mode::RWXU) {
            Ok(()) => {}
            Err(rustix::io::Errno::EXIST) => {}
            Err(error) => return Err(error.into()),
        }
        directory = openat(&directory, component.as_os_str(), flags, Mode::empty())?;
    }
    let name = components.last().unwrap().as_os_str();
    let mut temporary = name.to_os_string();
    temporary.push(".pending");
    let fd = openat(
        &directory,
        &temporary,
        OFlags::WRONLY | OFlags::CREATE | OFlags::EXCL | OFlags::NOFOLLOW | OFlags::CLOEXEC,
        Mode::RUSR | Mode::WUSR,
    )?;
    let mut file = File::from(fd);
    let result = (|| -> Result<()> {
        file.write_all(bytes)?;
        file.sync_all()?;
        renameat(&directory, &temporary, &directory, name)?;
        Ok(())
    })();
    if result.is_err() {
        let _ = unlinkat(&directory, &temporary, AtFlags::empty());
    }
    result
}
#[cfg(not(unix))]
pub fn write(_root: &Path, _relative: &Path, _bytes: &[u8]) -> Result<()> {
    anyhow::bail!("safe evidence publication requires Unix in v1")
}
