//! Async (if possible) FS operations with a fallback to sync one.

use std::ffi::OsString;
use std::marker::Unpin;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[cfg(target_os = "windows")]
use std::os::windows::io::RawHandle;

use heim_common::prelude::*;

use crate::shims;

/// A reference to an open file in filesystem.
#[derive(Debug)]
pub struct File(shims::fs::File);

impl File {
    /// Attempt to open file in read-only mode.
    pub fn open<T>(path: T) -> impl Future<Output = Result<File>>
    where
        T: AsRef<Path> + Send + Unpin + 'static,
    {
        shims::fs::File::open(path).map_ok(File)
    }

    /// Returns the raw Windows handle from file.
    #[cfg(target_os = "windows")]
    pub fn as_raw_handle(&self) -> RawHandle {
        self.0.as_raw_handle()
    }
}

/// An entry returned by `read_dir` function.
#[derive(Debug)]
pub struct DirEntry(shims::fs::DirEntry);

impl DirEntry {
    /// Returns the full path to the file that this entry represents.
    pub fn path(&self) -> PathBuf {
        self.0.path()
    }

    /// Returns the bare file name of this directory entry without any other leading path component.
    pub fn file_name(&self) -> OsString {
        self.0.file_name()
    }
}

/// Returns future which checks if path `path` points to some file.
pub fn path_exists<T>(path: T) -> impl Future<Output = bool>
where
    T: AsRef<Path> + Send + Unpin + 'static,
{
    shims::fs::path_exists(path)
}

/// Read `path` file asynchronously and convert it contents into a string.
pub fn read_to_string<T>(path: T) -> impl Future<Output = Result<String>>
where
    T: AsRef<Path> + Send + Unpin + 'static,
{
    shims::fs::read_to_string(path)
}

/// Read `path` file and try to parse it into a `R` type via `std::str::FromStr`.
pub fn read_into<T, R, E>(path: T) -> impl Future<Output = Result<R>>
where
    T: AsRef<Path> + Send + Unpin + 'static,
    R: FromStr<Err = E>,
    Error: From<E>,
{
    shims::fs::read_into(path)
}

/// Returns stream of lines yielded from file with `path` path.
pub fn read_lines<T>(path: T) -> impl TryStream<Ok = String, Error = Error>
where
    T: AsRef<Path> + Send + Unpin + 'static,
{
    shims::fs::read_lines(path)
}

/// Returns stream which reads lines from file and tries to parse them with help of `FromStr` trait.
pub fn read_lines_into<T, R, E>(path: T) -> impl TryStream<Ok = R, Error = Error>
where
    T: AsRef<Path> + Send + Unpin + 'static,
    R: FromStr<Err = E>,
    Error: From<E>,
{
    shims::fs::read_lines_into(path)
}

/// Returns future which tries to read the first line from file.
pub fn read_first_line<T>(path: T) -> impl TryFuture<Ok = String, Error = Error>
where
    T: AsRef<Path> + Send + Unpin + 'static,
{
    shims::fs::read_first_line(path)
}

/// Returns stream of files and directories contained in the `path` directory.
pub fn read_dir<T>(path: T) -> impl TryStream<Ok = DirEntry, Error = Error>
where
    T: AsRef<Path> + Send + Unpin + 'static,
{
    shims::fs::read_dir(path).map_ok(DirEntry)
}

/// Returns future which tries read the symlink.
pub fn read_link<T>(path: T) -> impl TryFuture<Ok = PathBuf, Error = Error>
where
    T: AsRef<Path> + Send + Unpin + 'static,
{
    shims::fs::read_link(path)
}
