use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use fastq2comp::BaseComp;

async fn plot_comp(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
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