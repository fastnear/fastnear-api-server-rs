use actix_web::{get, web, Responder};

use crate::api::{HealthError, ServiceError};
use crate::types::{HealthResponse, StatusResponse};
use crate::{database, AppState, Config};

async fn internal_status(app_state: &web::Data<AppState>) -> Result<StatusResponse, ServiceError> {
    let mut connection = app_state
        .redis_client
        .get_multiplexed_async_connection()
        .await?;

    let latest_sync_block = database::query_get(&mut connection, "meta:latest_block").await?;
    let latest_block_time = database::query_get(&mut connection, "meta:latest_block_time").await?;
    let latest_balance_block =
        database::query_get(&mut connection, "meta:latest_balance_block").await?;

    let sync_latency_sec = latest_block_time.as_ref().map(|timestamp| {
        let timestamp_nanos = timestamp.parse::<u128>().unwrap_or(0);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        now.as_nanos().saturating_sub(timestamp_nanos) as f64 / 1e9
    });

    Ok(StatusResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
        sync_block_height: latest_sync_block.and_then(|value| value.parse::<u64>().ok()),
        sync_latency_sec,
        sync_block_timestamp_nanosec: latest_block_time,
        sync_balance_block_height: latest_balance_block.and_then(|value| value.parse::<u64>().ok()),
    })
}

fn is_healthy(status_response: &StatusResponse, config: &Config) -> Result<(), HealthError> {
    let latency = status_response
        .sync_latency_sec
        .ok_or(HealthError::MissingSyncLatency)?;
    if latency > config.max_healthy_latency_sec {
        return Err(HealthError::HighSyncLatency {
            latency,
            max_latency: config.max_healthy_latency_sec,
        });
    }

    let latest_sync_block = status_response
        .sync_block_height
        .ok_or(HealthError::MissingSyncBlockHeight)?;
    let latest_balance_block = status_response
        .sync_balance_block_height
        .ok_or(HealthError::MissingSyncBalanceBlockHeight)?;
    let sync_difference = latest_sync_block.saturating_sub(latest_balance_block);
    if sync_difference > config.max_healthy_sync_block_diff {
        return Err(HealthError::HighSyncBlockDiff {
            sync_difference,
            max_sync_difference: config.max_healthy_sync_block_diff,
        });
    }

    Ok(())
}

#[get("/status")]
pub async fn status(app_state: web::Data<AppState>) -> Result<impl Responder, ServiceError> {
    internal_status(&app_state).await.map(web::Json)
}

#[get("/health")]
pub async fn health(app_state: web::Data<AppState>) -> Result<impl Responder, ServiceError> {
    let status_response = internal_status(&app_state).await?;
    let response = HealthResponse {
        status: is_healthy(&status_response, &app_state.config)
            .map(|_| "ok".to_string())
            .unwrap_or_else(|error| format!("{:?}", error)),
    };

    Ok(web::Json(response))
}
