use crate::*;
use actix_web::{get, web, Responder};
use serde_json::json;

async fn internal_status(
    app_state: &web::Data<AppState>,
) -> Result<serde_json::Value, api::ServiceError> {
    let mut connection = app_state
        .redis_client
        .get_multiplexed_async_connection()
        .await?;

    let latest_sync_block = database::query_get(&mut connection, "meta:latest_block").await?;
    let latest_block_time = database::query_get(&mut connection, "meta:latest_block_time").await?;
    let latest_balance_block =
        database::query_get(&mut connection, "meta:latest_balance_block").await?;

    let sync_latency_sec = latest_block_time.as_ref().map(|t| {
        let t_nano = t.parse::<u128>().unwrap_or(0);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        now.as_nanos().saturating_sub(t_nano) as f64 / 1e9
    });

    Ok(json!({
        "version": env!("CARGO_PKG_VERSION"),
        "sync_block_height": latest_sync_block.map(|s| s.parse::<u64>().unwrap_or(0)),
        "sync_latency_sec": sync_latency_sec,
        "sync_block_timestamp_nanosec": latest_block_time,
        "sync_balance_block_height": latest_balance_block.map(|s| s.parse::<u64>().unwrap_or(0)),
    }))
}

fn is_healthy(v: serde_json::Value, config: &Config) -> Option<()> {
    let latency = v["sync_latency_sec"].as_f64()?;
    if latency > config.max_healthy_latency_sec {
        return None;
    }
    let latest_sync_block = v["sync_block_height"].as_u64()?;
    let latest_balance_block = v["sync_balance_block_height"].as_u64()?;
    if latest_sync_block.saturating_sub(latest_balance_block) > config.max_healthy_sync_block_diff {
        None
    } else {
        Some(())
    }
}

#[get("/status")]
pub async fn status(
    app_state: web::Data<AppState>,
) -> Result<impl Responder, crate::api::ServiceError> {
    internal_status(&app_state).await.map(|res| web::Json(res))
}

#[get("/health")]
pub async fn health(app_state: web::Data<AppState>) -> Result<impl Responder, api::ServiceError> {
    let res = internal_status(&app_state).await?;
    Ok(web::Json(
        json!({"status": is_healthy(res, &app_state.config).map(|_| "ok").unwrap_or("unhealthy")}),
    ))
}
