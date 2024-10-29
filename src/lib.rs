use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
#[cfg(target_family = "unix")]
use std::os::unix;
#[cfg(target_family = "windows")]
use std::os::windows;
use std::path::Path;

/// A simple implementation of `% cat path`
pub fn read_file(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

/// A simple implementation of `% echo s > path`
pub fn write_file(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;
    f.write_all(s.as_bytes())
}

/// A simple implementation of `% touch path` (ignores existing files)
pub fn touch_file(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

/// Recursively create a directory, returns `io::Result<()>`
/// like `mkdir -p a/c/d`
pub fn create_dir(path: &Path) -> io::Result<()>{
    match fs::create_dir_all(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

/// Remove a file, returns `io::Result<()>`
pub fn remove_file(path: &Path) -> io::Result<()> {
    match fs::remove_file(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

/// Remove an empty directory, returns `io::Result<()>`
/// Or pass not_empty [bool] as true to remove non empty ones
pub fn remove_dir (path: &Path, not_empty: bool) -> io::Result<()> {
    if not_empty {
        match fs::remove_dir_all(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    } else {
        match fs::remove_dir(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

/// Create a symbolic link, returns `io::Result<()>`
pub fn create_link (src_path: &Path, dst_path: &Path) -> io::Result<()> {
    #[cfg(target_family = "unix")] {
        match unix::fs::symlink(src_path, dst_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
    #[cfg(target_family = "windows")] {
        match windows::fs::symlink_file(src_path, dst_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

