mod api;
mod database;
mod redis_db;

use dotenv::dotenv;
use std::env;
use std::sync::{Arc, Mutex};

use crate::redis_db::RedisDB;
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{get, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
pub struct AppState {
    pub db: clickhouse::Client,
    pub redis_db: Arc<Mutex<RedisDB>>,
}

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix Web!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        // .with_env_filter(EnvFilter::new("debug"))
        .with_writer(std::io::stderr)
        .init();

    let db = database::establish_connection();
    let redis_db = Arc::new(Mutex::new(RedisDB::new(None).await));

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
            .app_data(web::Data::new(AppState {
                db: db.clone(),
                redis_db: redis_db.clone(),
            }))
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .wrap(tracing_actix_web::TracingLogger::default())
            .service(
                web::scope("/v0")
                    .service(api::lookup_by_public_key)
                    .service(api::lookup_by_public_key_all)
                    .service(api::staking)
                    .service(api::ft)
                    .service(api::nft),
            )
            .route("/", web::get().to(greet))
    })
    .bind(format!("127.0.0.1:{}", env::var("PORT").unwrap()))?
    .run()
    .await
}
