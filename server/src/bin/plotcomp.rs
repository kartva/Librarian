use std::fs::{File, OpenOptions};
use std::path::PathBuf;

use fastq2comp::extract_comp::{FASTQReader, run, SampleArgs};
use librarian_server::plot_comp;
use std::io::{BufReader, Write};
use log::{error};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "extract FASTQ base composition", about = "Extracts base composition of FASTQ file and returns result in JSON.")]
struct Cli {
    #[structopt(flatten)]
    pub sample_args: SampleArgs,

    /// Input files
    #[structopt(parse(from_os_str), required_unless("stdin"))]
    pub input: Vec<PathBuf>,
}

fn main() {
    let args = Cli::from_args();
	simple_logger::init_with_env().unwrap();

	for p in args.input {
		let p = p.canonicalize().unwrap();
		let f = File::open(&p);
		if let Err(e) = f {
			error!("Couldn't open {p:?} for reading due to error {e}");
			continue;
		}
		let f = f.unwrap();
		let comp = run(FASTQReader::new(args.sample_args, BufReader::new(f)));
	
		let res = plot_comp(comp);

		if let Err(e) = res {
			error!("Couldn't plot composition of {p:?} due to error {e}");
			continue;
		}
		let res = res.unwrap();

		for (i, r) in res.into_iter().enumerate() {
			let mut p = p.clone().into_os_string();
			p.push(i.to_string() + ".png");
			let p = PathBuf::from(p);
			let mut f = OpenOptions::new().create(true).write(true).open(p).unwrap();
			f.write_all(&r).unwrap();
		}
	}
}