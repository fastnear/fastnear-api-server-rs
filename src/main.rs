mod api;
mod database;

use dotenv::dotenv;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
pub struct AppState {
    pub db: clickhouse::Client,
}

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix Web!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let db = database::establish_connection();

    HttpServer::new(move || {
        // Configure CORS middleware
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .max_age(3600)
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .wrap(cors)
            .service(web::scope("/api").service(api::lookup_by_public_key))
            .route("/", web::get().to(greet))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
