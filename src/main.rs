use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use log::info;

#[get("/")]
async fn hello() -> impl Responder {
    info!("Handling /hello request");
    HttpResponse::Ok().body("Hello Rust App!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    info!("Handling /echo request");
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    info!("Handling /hey request");
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // For local testing change to http://127.0.0.1:8080
    info!("Starting server at http://0.0.0.0:8080");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        // For local testing change to http://127.0.0.1:8080 -> Docker works with "0.0.0.0"
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}