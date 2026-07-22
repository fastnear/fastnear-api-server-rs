pub mod api;
pub mod database;
#[cfg(feature = "openapi")]
pub mod openapi;
pub mod redis_db;
pub mod rpc;
pub mod status;
pub mod types;

use actix_web::{HttpResponse, Responder};

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

const INDEX_HTML: &str = include_str!("../index.html");
const SKILL_MD: &str = include_str!("../skill.md");

pub async fn index_html() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(INDEX_HTML)
}

pub async fn skill_md() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/markdown; charset=utf-8")
        .body(SKILL_MD)
}
