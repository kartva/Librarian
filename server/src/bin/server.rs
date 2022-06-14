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
use serde_json::{Value, json};
use simple_logger::SimpleLogger;
use std::{
    env, path::PathBuf, str::FromStr, fmt::Debug
};

use librarian_server::plot_comp;

#[post("/api/plot_comp")]
async fn plot(comp: web::Json<Vec<BaseComp>>) -> impl Responder {
    match web::block(move || plot_comp(comp.into_inner())).await {
        Ok(o) => {
            let out_arr = o.into_iter().map(|p| {
                    let key = p.filename;
                    let val = base64::encode_config(p.plot, STANDARD);
                    json!({key: val})
                }
            ).collect();

            HttpResponse::Ok().content_type("application/json").body(Value::Array(out_arr))
        },
        Err(blocking_e) => match blocking_e {
            BlockingError::Error(s) => HttpResponse::InternalServerError().body(s.to_string()),
            BlockingError::Canceled => panic!("Error blocking threadpool?"),
        },
    }
}

fn get_env_or_default<K, S> (key: K, default: S) -> S
where
    K: AsRef<std::ffi::OsStr> + Debug,
    S: FromStr + Debug
{
    env::var(&key)
            .ok()
            .and_then(|val| val.parse().ok())
            .unwrap_or_else(|| {
                warn!("{:?} not found, using default value {:?}", &key, default);
                default
            })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    eprintln!("Starting application");

    HttpServer::new(|| {
        SimpleLogger::new().with_utc_timestamps().with_colors(true)
            .with_level(log::LevelFilter::Info)
            .env()
            .init()
            .unwrap();

        let frontend_index: PathBuf = get_env_or_default("LIBRARIAN_INDEX_PATH", "../frontend/dist".into());
        let example_input: PathBuf = get_env_or_default("LIBRARIAN_INPUT_PATH", "../frontend/example_inputs".into());

        App::new()
            .wrap(middleware::Logger::default())
            .service(plot)
            .service(
                Files::new(
                    "/example_inputs",
                    example_input
                )
            )
            .service(
                Files::new(
                    "/",
                    frontend_index
                )
                .index_file("index.html"),
            )
    })
    .bind(("0.0.0.0", get_env_or_default("LIBRARIAN_PORT", 8186)))?
    .run()
    .await
}
