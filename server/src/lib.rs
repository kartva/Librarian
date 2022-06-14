use fastq2comp::BaseComp;
use std::{
    fs::{read_dir, File},
    io::{Read, Write},
    process::{Command, Stdio},
};
use log::{self, debug, trace, warn};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlotError {
    #[error("RScript exited unsuccessfully: {0}")]
    RExit(String),
	#[error("Error opening directory")]
	DirErr(#[from] std::io::Error)
}

use serde::{Serialize, Deserialize};
/// Describes a plot; which has a filename and data.
#[derive(Serialize, Deserialize)]
pub struct Plot {
    /// Raw plot data - in svg
    pub plot: Vec<u8>,
    pub filename: String,
}

impl actix_web::error::ResponseError for PlotError {}

pub fn plot_comp(comp: Vec<BaseComp>) -> Result<Vec<Plot>, PlotError> {
    assert!(!comp.is_empty());

    let mut input = String::new();

    for (i, c) in comp.into_iter().enumerate() {
        input += &format!("sample_{:02}\t", i + 1);
        c
            .lib
            .into_iter()
            .flat_map(|b| b.bases.iter())
            .for_each(|curr| input.push_str(&(curr.to_string() + "\t")));
        input.pop(); // remove trailing '\t' to make it valid tsv
        input.push('\n');
    }
    trace!("Input: {:?}", &input);

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

    let mut child = Command::new("Rscript")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg("scripts/librarian_plotting_multi_server_220606.R")
        .arg("--args")
        .arg(&tmpdir)
        .spawn()
        .expect("Failed to spawn child process");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");

    std::thread::spawn(move || {
        stdin
            .write_all(input.as_bytes())
            .expect("Failed to write to stdin")
    });

    let exit_status = child.wait().expect("Error waiting on child to exit.");
    if !exit_status.success() {
        let mut buf = String::new();
        return Err(PlotError::RExit(
            match child.stdout.take().unwrap().read_to_string(&mut buf) {
                Ok(_) => buf,
                Err(e) => e.to_string(),
            },
        ));
    };

    debug!("Child executed successfuly.");

    let out_arr = read_dir(&tmpdir)?
        .filter_map(|e| {
			if e.is_err() {
				warn!("Error iterating over dir {:?}, skipping file.", &tmpdir)
			};
			e.ok()
		})
        .filter_map(|e| {
            let filename = e.file_name().to_string_lossy().to_owned().to_string();
            let f = File::open(e.path());
			if f.is_err() {
				warn!("Error opening file {:?} due to error {:?}", e.path(), &f);
				return None;
			};
            let mut buf = Vec::new();
			if let Err(err) = f.unwrap().read_to_end(&mut buf) {
				warn!("Error reading file {:?} due to error {:?}", e.path(), err);
				return None;
			}
			Some(Plot {plot: buf, filename})
        })
        .collect::<Vec<_>>();

	trace!("Deleting files.");
    std::fs::remove_dir_all(&tmpdir).expect("Error deleting tmpfile.");

    Ok(out_arr)
}
