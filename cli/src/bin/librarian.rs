use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::time::Duration;
use base64::decode;
use colored::Colorize;

use fastq2comp::extract_comp::{FASTQReader, run, SampleArgs};
use std::io::{BufReader, Write};
use log::{error, info};

use time::format_description::parse;
use time::{OffsetDateTime};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
	name = "Librarian CLI",
	about = "Utility CLI to query the Librarian server for plotting base compositions."
)]
struct Cli {
    /// List of input files
    #[structopt(required = true, parse(from_os_str))]
    pub input: Vec<PathBuf>,

	/// Output path
	#[structopt(short = "o", long = "output", parse(from_os_str))]
	pub outdir: Option<PathBuf>,
}

#[tokio::main]
async fn main() {
    let args = Cli::from_args();
	simple_logger::init_with_level(log::Level::Warn).unwrap();

	let client = reqwest::Client::builder().timeout(Duration::from_secs(60)).build().unwrap();
	let mut comps = Vec::with_capacity(args.input.len());

	println!("{}", "Requests may take up to 60 seconds to process.".green());

	for p in args.input {
		let p = p.canonicalize().unwrap();
		let f = File::open(&p);
		if let Err(e) = f {
			error!("Couldn't open {:?} for reading due to error {}", p, e);
			continue;
		}
		let f = f.unwrap();
		let comp = run(FASTQReader::new(SampleArgs::default(), BufReader::new(f)));
		comps.push(comp);
	}
	let req = client.post("https://www.bioinformatics.babraham.ac.uk/librarian/api/plot_comp").json(&comps).send().await;

	let res = req.expect("Request to server failed.");
	let res = res.json::<Vec<String>>().await.expect("Unable to extract JSON from server response. Server may be down.");

	let plot_names = ["tile-probability-map", "tile-probability-barchart", "reference-map"];
	assert_eq!(res.len(), plot_names.len());

	for (res, name) in res.into_iter().zip(plot_names) {
		let r = decode(res).expect("Server response was malformed.");

		let d =
			name.to_string() +
			"-" + &OffsetDateTime::now_local()
				.unwrap_or_else(|_| {info!("Couldn't get local time. Using UTC time instead."); OffsetDateTime::now_utc()})
				.format(&parse("[ year ]-[ month ]-[ day ]-[ hour ]-[ minute ]").unwrap())
				.unwrap()
			+ ".png";

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
		println!("{} {:?}", "Created ".green(), p);
	};
}