use log::{debug, log_enabled, trace};
use std::ops::Deref;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub struct TempDir {
    path: PathBuf,
    cleanup: bool,
}

impl TempDir {
    /// Create a new TempDir.
    /// Will delete the directory on drop.
    pub fn new() -> Self {
        let output_str = String::from_utf8_lossy(
            &Command::new("mktemp")
                .arg("-d")
                .output()
                .expect("Temporary file creation failed.")
                .stdout, // removes the \n which mktemp appends
        ).to_string();

        let tmpdir = output_str.split('\n').next().unwrap();

        let tmpdir = PathBuf::from(tmpdir);

        debug!("Tempdir: {:?}", tmpdir);

        Self {
            path: tmpdir,
            cleanup: true,
        }
    }

    /// Create a new TempDir from an existing path.
    /// Will not delete the directory on drop.
    pub fn from_dir(path: &Path) -> Self {
        let path = path.to_path_buf();
        Self {
            path,
            cleanup: false,
        }
    }
}

impl Deref for TempDir {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        if !self.cleanup {
            return;
        }

        if log_enabled!(log::Level::Trace) {
            trace!("Leaving tempdir files at {:?} undeleted since trace is enabled.", self.path);
            return;
        }

        debug!("Deleting files.");
        std::fs::remove_dir_all(&self.path).expect("Error deleting tmpfile.");
    }
}
