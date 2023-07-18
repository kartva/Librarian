use std::process::Command;
use log::{trace, debug, log_enabled};
use std::ops::Deref;

pub struct TempDir {
    path: String,
}

impl TempDir {
    pub fn new() -> Self {
        let tmpdir = String::from_utf8_lossy(
            &Command::new("mktemp")
                .arg("-d")
                .output()
                .expect("Temporary file creation failed.")
                .stdout, // removes the \n which mktemp appends
        )
        .to_string()
        .split('\n')
        .next()
        .unwrap()
        .to_owned();

        debug!("Tempdir: {:?}", tmpdir);

        Self { path: tmpdir }
    }
}

impl Deref for TempDir {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        trace!("Deleting files.");
        if log_enabled!(log::Level::Debug) {
            return;
        }
        
        std::fs::remove_dir_all(&self.path).expect("Error deleting tmpfile.");
    }
}