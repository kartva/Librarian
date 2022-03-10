/*
Currently, test responses take around 6-7 seconds, which is far longer than the recommended 500 ms.
Since this is a long-running calculation, consider using this advice:
https://docs.microsoft.com/en-us/azure/architecture/patterns/async-request-reply
*/

use actix_files::Files;
use actix_web::{
    error::BlockingError, middleware, post, web, App, HttpResponse, HttpServer, Responder,
};
use base64::{self, STANDARD};
use fastq2comp::BaseComp;
use log::{self, warn};
use serde_json::Value;
use simple_logger::SimpleLogger;
use std::{
    env,
};

use librarian_server::plot_comp;

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
async fn plot(comp: web::Json<Vec<BaseComp>>) -> impl Responder {
    match web::block(move || plot_comp(comp.into_inner())).await {
        Ok(o) => {
            let out_arr = o.into_iter().map(|f| Value::String(base64::encode_config(&f, STANDARD))).collect();
            HttpResponse::Ok().content_type("application/json").body(Value::Array(out_arr))
        },
        Err(blocking_e) => match blocking_e {
            BlockingError::Error(s) => HttpResponse::InternalServerError().body(s.to_string()),
            BlockingError::Canceled => panic!("Error blocking threadpool?"),
        },
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    eprintln!("Starting application");

    HttpServer::new(|| {
        SimpleLogger::new()
            .with_level(log::LevelFilter::Trace)
            .init()
            .unwrap();

        App::new()
            .wrap(middleware::Logger::default())
            .service(plot)
            .service(
                Files::new(
                    "/",
                    env::var("LIBRARIAN_INDEX_PATH").ok().unwrap_or_else(|| {
                        warn!(
                            "LIBRARIAN_INDEX_PATH not found, using default path (../frontend/dist)"
                        );
                        "../frontend/dist".to_string()
                    }),
                )
                .index_file("index.html"),
            )
    })
    .bind(("0.0.0.0", {
        env::var("LIBRARIAN_PORT")
            .ok()
            .and_then(|port| port.parse().ok())
            .unwrap_or_else(|| {
                warn!("LIBRARIAN_PORT not found, using default port 8186.");
                8186
            })
    }))?
    .run()
    .await
}
