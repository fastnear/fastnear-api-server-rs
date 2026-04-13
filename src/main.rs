use std::env;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use fastnear_api_server_rs::{
    api_exp_scope, api_v0_scope, api_v1_scope, index_html, skill_md, status, AppState, Config,
};
use tracing_subscriber::EnvFilter;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    openssl_probe::init_ssl_cert_env_vars();
    dotenv().ok();

    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let redis_client =
        redis::Client::open(env::var("REDIS_URL").expect("Missing REDIS_URL env var"))
            .expect("Failed to connect to Redis");

    let config = Config {
        max_healthy_latency_sec: env::var("MAX_HEALTHY_SYNC_LATENCY_SEC")
            .map(|value| {
                value
                    .parse()
                    .expect("Failed to parse MAX_HEALTHY_SYNC_LATENCY_SEC")
            })
            .unwrap_or(10.0),
        max_healthy_sync_block_diff: env::var("MAX_HEALTHY_SYNC_BLOCK_DIFF")
            .map(|value| {
                value
                    .parse()
                    .expect("Failed to parse MAX_HEALTHY_SYNC_BLOCK_DIFF")
            })
            .unwrap_or(3),
    };

    let enable_experimental = env::var("EXPERIMENTAL_API").ok() == Some("true".to_string());

    HttpServer::new(move || {
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
                redis_client: redis_client.clone(),
                config: config.clone(),
            }))
            .wrap(cors)
            .wrap(middleware::Logger::new(
                "%{r}a \"%r\"\t%s %b \"%{Referer}i\" \"%{User-Agent}i\" %T",
            ))
            .wrap(tracing_actix_web::TracingLogger::default())
            .service(api_v0_scope())
            .service(api_exp_scope(enable_experimental))
            .service(api_v1_scope())
            .service(status::status)
            .service(status::health)
            .route("/index.html", web::get().to(index_html))
            .route("/skill.md", web::get().to(skill_md))
            .route("/", web::get().to(index_html))
    })
    .bind(format!("127.0.0.1:{}", env::var("PORT").unwrap()))?
    .run()
    .await?;

    Ok(())
}
