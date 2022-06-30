use colored::Colorize;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::time::Duration;

use fastq2comp::extract_comp::{run, FASTQReader, SampleArgs};
use fastq2comp::io_utils;
use log::{error, trace, debug, info, warn};
use server::Plot;
use simple_logger::SimpleLogger;
use std::env::var;
use std::io::{BufReader, Write};

use time::format_description::parse;
use time::OffsetDateTime;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Librarian CLI",
    about = "A tool to predict the sequencing library type from the base composition of a supplied FastQ file. Uncompresses .gz files when reading.",

)]
struct Cli {
    /// List of input files
    #[structopt(required = true, parse(from_os_str))]
    pub input: Vec<PathBuf>,

    /// Output path
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    pub outdir: Option<PathBuf>,
}

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .env()
        .with_colors(true)
        .without_timestamps()
        .init()
        .unwrap();

    query(Cli::from_args());
}

#[tokio::main(flavor = "current_thread")]
async fn query(args: Cli) {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(60 * 5))
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();
    let mut comps = Vec::with_capacity(args.input.len());

    info!(
        "{}",
        "Requests may take up to 5 minutes to process.".green()
    );

    for p in args.input {
        let p = p.canonicalize().unwrap();
        let f = File::open(&p);
        if let Err(e) = f {
            error!("Couldn't open {:?} for reading due to error {}", p, e);
            continue;
        }
        let f = f.unwrap();
        let reader =
            io_utils::compressed_reader(f, matches!(p.extension(), Some(ext) if ext == "gz"));

        let comp = run(FASTQReader::new(
            SampleArgs::default(),
            BufReader::new(reader),
        ));

        let l = comp.reads_read();
        let target_len = SampleArgs::default().target_read_count;

        if l < target_len {
            warn!("Fewer valid reads ({l}) in sample {p:?} than recommended (this may be due to reads being filtered out due to being shorter than 50 bases)")
        }

        comps.push(comp);
    }

    debug!("Compositions: {:#?}", comps);

    let url = var("LIBRARIAN_API_URL").unwrap_or_else(|e| {
        trace!("LIBRARIAN_API_URL {e}, using default");
        "https://www.bioinformatics.babraham.ac.uk/librarian/api/plot_comp".to_string()
    });

    let req = client.post(&url).json(&comps).send().await;

    let res = req.expect("Request to server failed.");
    if !res.status().is_success() {
        eprintln!(
            "non-success response {} received, terminating",
            res.status().to_string().red()
        );
        eprintln!("error body: {}", res.text().await.unwrap());
        panic!();
    }

    let res = res
        .json::<Vec<Plot>>()
        .await
        .expect("unable to extract JSON from server response. server may be down");

    for res in res.into_iter() {
        let r = res.plot;

		let (root, ext) = res.filename.split_once('.').expect("filename did not have . at all");
        let d = root.to_string() + "-"
            + &OffsetDateTime::now_utc()
                .format(&parse("[ year ]-[ month ]-[ day ]-[ hour ]-[ minute ]").unwrap())
                .unwrap()
            + "." + ext;

        let p = match &args.outdir {
            None => PathBuf::from(d),
            Some(ref o) => {
                let mut p = PathBuf::from(o);
                p.push(d);
                p
            }
        };
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&p)
            .unwrap();
        f.write_all(r.as_bytes()).unwrap();
        println!("{} {:?}", "Created ".green(), p);
    }
}
