use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::time::Duration;
use base64::decode;
use colored::Colorize;
use futures::future::join;
use futures::{stream, StreamExt};

use fastq2comp::extract_comp::{FASTQReader, run, SampleArgs};
use std::io::{BufReader, Write};
use log::error;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "plot FASTQ base composition", about = "Extracts base composition of FASTQ file and plots it.")]
struct Cli {
    #[structopt(flatten)]
    pub sample_args: SampleArgs,

    /// Input files
    #[structopt(parse(from_os_str), required_unless("stdin"))]
    pub input: Vec<PathBuf>,
}

#[tokio::main]
async fn main() {
    let args = Cli::from_args();
	simple_logger::init_with_level(log::Level::Warn).unwrap();

	let client = reqwest::Client::builder().timeout(Duration::from_secs(60)).build().unwrap();
	let mut requests = Vec::new();
	requests.reserve(args.input.len());

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
	
		requests.push((p, client.post("http://127.0.0.1:8186/api/plot_comp").json(&comp).send()));
	}

	let bodies = stream::iter(requests.into_iter())
		.map(|f| join(futures::future::ok::<PathBuf, ()>(f.0), f.1))
		.buffer_unordered(3)
		.filter_map(|(p, req)| {
			async {
				let p = p.unwrap();
				let res = req.ok()?;
				Some((p, res.json::<Vec<String>>().await.ok()?))
			}
		});

	bodies.for_each(|(p, res)| async move {
		for (i, r) in res.into_iter().enumerate() {
			let r = decode(r).expect("Server response was malformed.");

			let mut p = p.clone().into_os_string();
			p.push(i.to_string() + ".png");
			let p = PathBuf::from(p);
			let mut f = OpenOptions::new().create(true).write(true).open(&p).unwrap();
			f.write_all(&r).unwrap();
			println!("{} {p:?}", "Created ".green());
		}
	}).await
}