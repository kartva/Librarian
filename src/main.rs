/*
Currently, test responses take around 6-7 seconds, which is far longer than the recommended 500 ms.
Since this is a long-running calculation, consider using this advice:
https://docs.microsoft.com/en-us/azure/architecture/patterns/async-request-reply
*/

use actix_web::{App, HttpResponse, HttpServer, Responder, middleware, web, post};
use fastq2comp::BaseComp;
use std::{env, fs::File, io::{Read, Write}, process::{Command, Stdio}};
use simple_logger::SimpleLogger;
use log::{self, debug};
use actix_files::Files;

#[post("/api/plot_comp")]
async fn plot_comp(comp: web::Json<BaseComp>) -> impl Responder {
    let mut input = comp.into_inner().lib.into_iter().flat_map(|b| b.bases.iter()).
        fold(String::new(), |acc, curr| acc + &curr.to_string() + "\t");
    input.pop(); // remove trailing ',' to make it valid tsv

    let output = web::block(move || {
        let tmpfile = String::from_utf8_lossy(
                &Command::new("mktemp")
                    .output()
                    .expect("Temporary file creation failed.")
                .stdout
                // removes the \n which mktemp appends
            ).to_string().split_ascii_whitespace().next().unwrap().to_owned();
        debug!("Tempfile: {:?}", tmpfile);

        let mut child = Command::new("Rscript")
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .arg("scripts/placeholder_code_for_graph_210726.R")
            .arg("--args")
            .arg(&tmpfile)
            .spawn()
            .expect("Failed to spawn child process");

        debug!("Writing to Rscript stdin: {:?}", input);

        let mut stdin = child.stdin.take().expect("Failed to open stdin");

        std::thread::spawn (move ||
            stdin.write_all(input.as_bytes()).expect("Failed to write to stdin")
        );

        let res = match child.wait() {
            Ok(e) => {
                if !e.success() {return Err(format!("{:?}", e))};
                Ok({
                    let mut f = File::open(&tmpfile).expect("Unable to open tempfile");
                    let mut o = Vec::new();
                    if f.read_to_end(&mut o).expect("Unable to read tempfile") == 0 {
                        panic!("Temp file is empty.")
                    }
                    o
                })
            },
            Err(e) => Err(format!("{:?}", e))
        };

        std::fs::remove_file(&tmpfile).expect("Error deleting tmpfile.");

        res
    }).await;

    match output {
        Ok(o) => HttpResponse::Ok()
            .content_type("application/pdf")
            .body(o),
        Err(_e) => HttpResponse::InternalServerError().finish()
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
            .service(Files::new("/", "frontend/dist/").index_file("index.html"))
    })
    .bind(("0.0.0.0", {
        env::var("PORT")
            .ok()
            .and_then(|port| port.parse().ok())
            .unwrap_or_else(|| 8186)
        }
    ))?
    .run()
    .await
}