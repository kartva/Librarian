use actix_web::{web, App, HttpServer, HttpResponse, Responder};

use fastq2comp::BaseComp;
use std::{fs::{File, canonicalize}, io::{Read}, process::{Command, Stdio}};


async fn plot_comp(comp: web::Json<BaseComp>) -> impl Responder {
    let mut input = comp.into_inner().lib.into_iter().flat_map(|b| b.bases.iter()).
        fold(String::new(), |acc, curr| acc + &curr.to_string() + "\t");
    input.pop(); // remove trailing ',' to make it valid tsv

    let output = web::block(move || {
        let mut wd = dbg!(
            String::from_utf8_lossy(
                &Command::new("mktemp")
                    .arg("-d")
                    .output()
                    .expect("Temporary directory creation failed.")
                .stdout
            ).to_string()
        );
    
        let mut child = Command::new("sh")
            .current_dir(canonicalize("../../").unwrap())
            .arg("Rscript")
            .arg("scripts/placeholder_code_for_graph_210726.R")
            .arg(&wd)
            .stdin(Stdio::piped())
            .spawn()
            .expect("Failed to spawn child process");
            
        if child.wait().unwrap().success() {
            wd.push_str("umap_plot.png");
            
            Ok({
                let mut f = File::open(wd).expect("Error loading plot png file");
                let mut s = String::new();
                f.read_to_string(&mut s).expect("Error reading from file to string");
                s
            })
        } else {
            Err(child.stderr.take().unwrap())
        }
    }).await;

    match output {
        Ok(o) => HttpResponse::Ok()
            .content_type("image/png")
            .body(o),
        Err(_e) => HttpResponse::InternalServerError().finish()
    }

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .route("/plot_comp", web::post().to(plot_comp))
            )
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}