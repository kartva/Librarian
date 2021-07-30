use actix_web::{web, App, HttpServer, HttpResponse, Responder};

use fastq2comp::BaseComp;
use std::{fs::canonicalize, process::{Command, Stdio}, io::Write};


async fn plot_comp(comp: web::Json<BaseComp>) -> impl Responder {
    let mut input = comp.into_inner().lib.into_iter().flat_map(|b| b.bases.iter()).
        fold(String::new(), |acc, curr| acc + &curr.to_string() + "\t");
    input.pop(); // remove trailing ',' to make it valid tsv

    let mut child = Command::new("sh")
        .current_dir(canonicalize("..").unwrap())
        .arg("Rscript")
        .arg("scripts/placeholder_code_for_graph_210726.R")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");
        
    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin.write_all(input.as_bytes()).expect("Failed to write to stdin");
    });
    
    let output = child.wait_with_output().expect("Failed to read stdout");
    let output = String::from_utf8_lossy(&output.stdout);

    HttpResponse::Ok()
            .content_type("text/csv")
            .body(output.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .route("/plot_comp", web::post().to(plot_comp))
            )            
            .route("/{name}", web::get().to(plot_comp))
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}