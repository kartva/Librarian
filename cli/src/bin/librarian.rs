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

    /// Prefix to append to output files (eg. `output_dir/` or `name`)
    #[structopt(short, long, default_value = "librarian")]
    pub prefix: String,

    /// Suppresses all output except errors
    #[structopt(short, long)]
    pub quiet: bool,
}

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

fn main() {
    let mut args = Cli::from_args();
    args.prefix += "_"; // so the prefix "librarian" produces files like "librarian_compositions_map"

    SimpleLogger::new()
        .with_level (if args.quiet {log::LevelFilter::Error} else {log::LevelFilter::Info})
        .env()
        .with_colors(true)
        .without_timestamps()
        .init()
        .expect("Couldn't initialize logger");

    query(args);
}

fn query(args: Cli) {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(60 * 5))
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();
    let mut comps = Vec::with_capacity(args.input.len());

    for p in args.input {
        info!("Processing {p:?}");

        let f = File::open(&p);
        if let Err(e) = f {
            error!("Couldn't open {:?} (canonicalized: {:?}) for reading due to error {}", p, p.canonicalize(), e);
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
            warn!("Fewer valid reads ({l}) in sample {p:?} than recommended (100,000) (this may be due to reads being filtered out due to being shorter than 50 bases)")
        }

        comps.push(comp);
    }

    debug!("Compositions: {:#?}", comps);

    let url = var("LIBRARIAN_API_URL").unwrap_or_else(|e| {
        trace!("LIBRARIAN_API_URL {e}, using default");
        "https://www.bioinformatics.babraham.ac.uk/librarian/api/plot_comp".to_string()
    });

    info!("Sending data to server at https://www.bioinformatics.babraham.ac.uk");
    info!(
        "{}",
        "Requests may take up to 5 minutes to process.".green()
    );

    let req = client.post(&url).json(&comps).send();

    let res = req.map_err(|e| {error!("{}\n", "Request to server failed".to_string().red()); panic!("{}", e)}).unwrap();
    if !res.status().is_success() {
        error!(
            "non-success response {} received, terminating",
            res.status().to_string().red()
        );
        error!("error body: {}", res.text().unwrap());
        panic!();
    }

    let res = res
        .json::<Vec<Plot>>()
        .expect("unable to extract JSON from server response. server may be down");

    for mut res in res.into_iter() {
        let r = res.plot;

        let p = {
            res.filename.insert_str(0, &args.prefix);
            PathBuf::from(res.filename)
        };
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&p)
            .unwrap();
        f.write_all(r.as_bytes()).unwrap();
        info!("{} {:?}", "Created".green(), p);
    }
}
