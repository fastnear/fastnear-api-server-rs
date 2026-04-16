pub mod api;
pub mod database;
#[cfg(feature = "openapi")]
pub mod openapi;
pub mod redis_db;
pub mod rpc;
pub mod status;
pub mod types;

use actix_web::{web, HttpResponse, Responder, Scope};

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

pub fn api_v0_scope() -> Scope {
    web::scope("/v0")
        .service(api::v0::lookup_by_public_key)
        .service(api::v0::lookup_by_public_key_all)
        .service(api::v0::staking)
        .service(api::v0::ft)
        .service(api::v0::nft)
}

pub fn api_v1_scope() -> Scope {
    web::scope("/v1")
        .service(api::v0::lookup_by_public_key)
        .service(api::v0::lookup_by_public_key_all)
        .service(api::v1::staking)
        .service(api::v1::ft)
        .service(api::v1::nft)
        .service(api::v1::ft_top)
        .service(api::v1::account_full)
}

pub fn api_exp_scope(enable_experimental: bool) -> Scope {
    let mut scope = web::scope("/exp");

    if enable_experimental {
        scope = scope
            .service(api::exp::ft_with_balances)
            .service(api::exp::ft_all);
    }

    scope
}
