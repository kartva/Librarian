use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::time::Duration;
use base64::decode;
use colored::Colorize;

use futures::{stream, StreamExt};

use fastq2comp::extract_comp::{FASTQReader, run, SampleArgs};
use std::io::{BufReader, Write};
use log::error;

use time::format_description::well_known::Rfc3339;
use time::{OffsetDateTime};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "plot FASTQ base composition", about = "Extracts base composition of FASTQ file and plots it.")]
struct Cli {
    #[structopt(flatten)]
    pub sample_args: SampleArgs,

    /// Input files
    #[structopt(parse(from_os_str))]
    pub input: Vec<PathBuf>,

	/// Output path
	#[structopt(short = "o", long = "output", parse(from_os_str))]
	pub outdir: Option<PathBuf>
}

#[tokio::main]
async fn main() {
    let args = Cli::from_args();
	simple_logger::init_with_level(log::Level::Warn).unwrap();

	let client = reqwest::Client::builder().timeout(Duration::from_secs(60)).build().unwrap();
	let mut requests = Vec::with_capacity(args.input.len());

	println!("{}", "Requests may take up to 60 seconds to process.".green());

	for p in args.input {
		let p = p.canonicalize().unwrap();
		let f = File::open(&p);
		if let Err(e) = f {
			error!("Couldn't open {p:?} for reading due to error {e}");
			continue;
		}
		let f = f.unwrap();
		let comp = run(FASTQReader::new(args.sample_args, BufReader::new(f)));
	
		requests.push(client.post("http://127.0.0.1:8186/api/plot_comp").json(&comp).send());
	}

	let bodies = stream::iter(requests.into_iter())
		.buffer_unordered(3)
		.filter_map(|req| {
			async {
				let res = req.ok()?;
				res.json::<Vec<String>>().await.ok()
			}
		});

	bodies.for_each(|res| async {
		for (i, r) in res.into_iter().enumerate() {
			let r = decode(r).expect("Server response was malformed.");

			let d = "plot-".to_string() + &(i.to_string() + "-") + &OffsetDateTime::now_utc().format(&Rfc3339).unwrap() + ".png";
			let p = match &args.outdir {
				None => PathBuf::from(d),
				Some(ref o) => {
					let mut p = PathBuf::from(o);
					p.push(d);
					p
				}
			};
			let mut f = OpenOptions::new().create(true).write(true).open(&p).unwrap();
			f.write_all(&r).unwrap();
			println!("{} {p:?}", "Created ".green());
		}
	}).await
}