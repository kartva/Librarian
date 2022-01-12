/*
Currently, test responses take around 6-7 seconds, which is far longer than the recommended 500 ms.
Since this is a long-running calculation, consider using this advice:
https://docs.microsoft.com/en-us/azure/architecture/patterns/async-request-reply
*/

use actix_web::{App, HttpResponse, HttpServer, Responder, middleware, web, post, error::BlockingError};
use fastq2comp::BaseComp;
use serde_json::Value;
use std::{env, fs::{File, read_dir}, io::{Read, Write}, process::{Command, Stdio}};
use simple_logger::SimpleLogger;
use log::{self, debug, warn, trace};
use actix_files::Files;
use base64::{self, STANDARD};

// Look into https://actix.rs/docs/errors/ to improve error handling
/*
use thiserror::Error;

#[derive(Error, Debug)]
enum ServerError {
    #[error("R script gave a non-zero response. Output:\n`{0}`")]
    RScriptError(String),
}
*/

#[post("/api/plot_comp")]
async fn plot_comp(comp: web::Json<BaseComp>) -> impl Responder {
    let mut input = comp.into_inner().lib.into_iter().flat_map(|b| b.bases.iter()).
        fold(String::new(), |acc, curr| acc + &curr.to_string() + "\t");
    input.pop(); // remove trailing ',' to make it valid tsv

    let output = web::block(move || {
        let tmpdir = String::from_utf8_lossy(
                &Command::new("mktemp")
                    .arg("-d")
                    .output()
                    .expect("Temporary file creation failed.")
                .stdout
                // removes the \n which mktemp appends
            ).to_string().split('\n').next().unwrap().to_owned();
        debug!("Tempdir: {:?}", tmpdir);

        let mut child = Command::new("Rscript")
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .arg("scripts/librarian_plotting_server_220107.R")
            .arg("--args")
            .arg(&tmpdir)
            .spawn()
            .expect("Failed to spawn child process");

        let mut stdin = child.stdin.take().expect("Failed to open stdin");

        std::thread::spawn (move ||
            stdin.write_all(input.as_bytes()).expect("Failed to write to stdin")
        );

        let exit_status = child.wait().expect("Error waiting on child to exit.");
        if !exit_status.success() {return Err("RScript exited unsuccessfully.")};

        let mut buf = Vec::new();

        let out_arr = read_dir(&tmpdir).unwrap()
            .filter_map(|e| {
                if e.is_err() {warn!("Error iterating over dir {:?}, skipping file.", &tmpdir)};
                e.ok()
            })
            .filter_map(|e| {
                let f = File::open(e.path());
                if f.is_err() {warn! ("Error opening file {:?} due to error {:?}", e.path(), f)};
                f.ok()
            })
            .filter_map(|mut f| {
                buf.clear();
                match f.read_to_end(&mut buf) {
                    Ok(n) => if n == 0 {
                        warn! ("File is empty.");
                        return None;
                    },
                    Err(e) => {
                        warn! ("Unable to read tempfile {:?} with err {:?}", f, e);
                        return None;
                    }
                };

                Some(Value::String(base64::encode_config(&buf, STANDARD)))
            })
            .collect::<Vec<_>>();

        trace! ("Deleting files.");
        std::fs::remove_dir_all(&tmpdir).expect("Error deleting tmpfile.");

        Ok(Value::Array(out_arr))
    }).await;

    match output {
        Ok(o) => HttpResponse::Ok()
            .content_type("application/json")
            .body(o),
        Err(blocking_e) => match blocking_e {
            BlockingError::Error(s) => HttpResponse::InternalServerError().body(s),
            BlockingError::Canceled => panic!("Error blocking threadpool?")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    eprintln!("Starting application");

    HttpServer::new(|| {
        SimpleLogger::new().with_level(log::LevelFilter::Trace).init().unwrap();

        App::new()
            .wrap(middleware::Logger::default())
            .service(plot_comp)
            .service (
                Files::new (
                    "/", 
                    env::var("LIBRARIAN_INDEX_PATH").ok()
                        .unwrap_or_else(|| {
                            warn!("LIBRARIAN_INDEX_PATH not found, using default path (../frontend/dist)");
                            "../frontend/dist".to_string()
                        })
                )
                .index_file("index.html"))
    })
    .bind(("0.0.0.0", {
        env::var("LIBRARIAN_PORT")
            .ok()
            .and_then(|port| port.parse().ok())
            .unwrap_or_else (|| {
                warn!("LIBRARIAN_PORT not found, using default port 8186.");
                8186
            })
        }
    ))?
    .run()
    .await
}