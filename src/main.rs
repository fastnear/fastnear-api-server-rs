mod api;
mod database;
mod redis_db;
mod rpc;
mod status;

use dotenv::dotenv;
use std::env;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{get, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
pub struct Config {
    pub max_healthy_latency_sec: f64,
    pub max_healthy_sync_block_diff: u64,
}

#[derive(Clone)]
pub struct AppState {
    pub redis_client: redis::Client,
    pub config: Config,
}

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix Web!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    openssl_probe::init_ssl_cert_env_vars();
    dotenv().ok();

    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        // .with_env_filter(EnvFilter::new("debug"))
        .with_writer(std::io::stderr)
        .init();

    let redis_client =
        redis::Client::open(env::var("REDIS_URL").expect("Missing REDIS_URL env var"))
            .expect("Failed to connect to Redis");

    let config = Config {
        max_healthy_latency_sec: env::var("MAX_HEALTHY_SYNC_LATENCY_SEC")
            .map(|s| {
                s.parse()
                    .expect("Failed to parse MAX_HEALTHY_SYNC_LATENCY_SEC")
            })
            .unwrap_or(10.0),
        max_healthy_sync_block_diff: env::var("MAX_HEALTHY_SYNC_BLOCK_DIFF")
            .map(|s| {
                s.parse()
                    .expect("Failed to parse MAX_HEALTHY_SYNC_BLOCK_DIFF")
            })
            .unwrap_or(3),
    };

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

        let api_v0 = web::scope("/v0")
            .service(api::v0::lookup_by_public_key)
            .service(api::v0::lookup_by_public_key_all)
            .service(api::v0::staking)
            .service(api::v0::ft)
            .service(api::v0::nft);

        let mut api_exp = web::scope("/exp");

        if env::var("EXPERIMENTAL_API").ok() == Some("true".to_string()) {
            api_exp = api_exp
                .service(api::exp::ft_with_balances)
                .service(api::exp::ft_all);
        }

        let api_v1 = web::scope("/v1")
            .service(api::v0::lookup_by_public_key)
            .service(api::v0::lookup_by_public_key_all)
            .service(api::v1::staking)
            .service(api::v1::ft)
            .service(api::v1::nft)
            .service(api::v1::ft_top)
            .service(api::v1::account_full);

        App::new()
            .app_data(web::Data::new(AppState {
                redis_client: redis_client.clone(),
                config: config.clone(),
            }))
            .wrap(cors)
            .wrap(middleware::Logger::new(
                "%{r}a \"%r\"	%s %b \"%{Referer}i\" \"%{User-Agent}i\" %T",
            ))
            .wrap(tracing_actix_web::TracingLogger::default())
            .service(api_v0)
            .service(api_exp)
            .service(api_v1)
            .service(status::status)
            .service(status::health)
            .route("/", web::get().to(greet))
    })
    .bind(format!("127.0.0.1:{}", env::var("PORT").unwrap()))?
    .run()
    .await?;

    Ok(())
}
