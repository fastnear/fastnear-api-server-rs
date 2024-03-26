use crate::*;
use actix_web::ResponseError;
use near_account_id::AccountId;
use near_crypto::PublicKey;
use serde_json::json;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

const TARGET_API: &str = "api";

pub type BlockHeight = u64;

#[derive(Debug)]
enum ServiceError {
    DatabaseError(database::DatabaseError),
    RpcError(rpc::RpcError),
    ArgumentError,
}

impl From<redis::RedisError> for ServiceError {
    fn from(error: redis::RedisError) -> Self {
        ServiceError::DatabaseError(database::DatabaseError::RedisError(error))
    }
}

impl From<database::DatabaseError> for ServiceError {
    fn from(error: database::DatabaseError) -> Self {
        ServiceError::DatabaseError(error)
    }
}

impl From<rpc::RpcError> for ServiceError {
    fn from(error: rpc::RpcError) -> Self {
        ServiceError::RpcError(error)
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServiceError::DatabaseError(ref err) => write!(f, "Database Error: {:?}", err),
            ServiceError::ArgumentError => write!(f, "Invalid argument"),
            ServiceError::RpcError(ref err) => write!(f, "Rpc Error: {:?}", err),
        }
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json("Internal server error")
            }
            ServiceError::ArgumentError => HttpResponse::BadRequest().json("Invalid argument"),
            ServiceError::RpcError(ref e) => {
                HttpResponse::InternalServerError().json(format!("Internal server error {:?}", e))
            }
        }
    }
}

pub mod v0 {
    use super::*;

    #[get("/public_key/{public_key}")]
    pub async fn lookup_by_public_key(
        request: HttpRequest,
        app_state: web::Data<AppState>,
    ) -> Result<impl Responder, ServiceError> {
        let public_key = PublicKey::from_str(request.match_info().get("public_key").unwrap())
            .map_err(|_| ServiceError::ArgumentError)?;

        tracing::debug!(target: TARGET_API, "Looking up account_ids for public_key: {}", public_key);

        let connection = app_state.redis_client.get_async_connection().await?;

        let public_key = public_key.to_string();

        let account_ids = database::query_with_prefix(connection, "pk", &public_key).await?;

        Ok(web::Json(json!({
            "public_key": public_key,
            "account_ids": account_ids.into_iter().filter_map(|(k, v)| if v == "f" {
                Some(k)
            } else {
                None
            }).collect::<Vec<_>>(),
        })))
    }

    #[get("/public_key/{public_key}/all")]
    pub async fn lookup_by_public_key_all(
        request: HttpRequest,
        app_state: web::Data<AppState>,
    ) -> Result<impl Responder, ServiceError> {
        let public_key = PublicKey::from_str(request.match_info().get("public_key").unwrap())
            .map_err(|_| ServiceError::ArgumentError)?;

        tracing::debug!(target: TARGET_API, "Looking up account_ids for all public_key: {}", public_key);

        let connection = app_state.redis_client.get_async_connection().await?;

        let public_key = public_key.to_string();

        let account_ids = database::query_with_prefix(connection, "pk", &public_key).await?;

        Ok(web::Json(json!({
            "public_key": public_key,
            "account_ids": account_ids.into_iter().map(|(k, _v)| k).collect::<Vec<_>>(),
        })))
    }

    #[get("/account/{account_id}/staking")]
    pub async fn staking(
        request: HttpRequest,
        app_state: web::Data<AppState>,
    ) -> Result<impl Responder, ServiceError> {
        let account_id =
            AccountId::try_from(request.match_info().get("account_id").unwrap().to_string())
                .map_err(|_| ServiceError::ArgumentError)?;

        tracing::debug!(target: TARGET_API, "Looking up validators for account_id: {}", account_id);

        let connection = app_state.redis_client.get_async_connection().await?;

        let query_result =
            database::query_with_prefix(connection, "st", &account_id.to_string()).await?;

        Ok(web::Json(json!({
            "account_id": account_id,
            "pools": query_result.into_iter().map(|(k, _v)| k).collect::<Vec<String>>(),
        })))
    }

    #[get("/account/{account_id}/ft")]
    pub async fn ft(
        request: HttpRequest,
        app_state: web::Data<AppState>,
    ) -> Result<impl Responder, ServiceError> {
        let account_id =
            AccountId::try_from(request.match_info().get("account_id").unwrap().to_string())
                .map_err(|_| ServiceError::ArgumentError)?;

        tracing::debug!(target: TARGET_API, "Looking up fungible tokens for account_id: {}", account_id);

        let connection = app_state.redis_client.get_async_connection().await?;

        let query_result =
            database::query_with_prefix(connection, "ft", &account_id.to_string()).await?;

        Ok(web::Json(json!({
            "account_id": account_id,
            "contract_ids": query_result.into_iter().map(|(k, _v)| k).collect::<Vec<String>>(),
        })))
    }

    #[get("/account/{account_id}/nft")]
    pub async fn nft(
        request: HttpRequest,
        app_state: web::Data<AppState>,
    ) -> Result<impl Responder, ServiceError> {
        let account_id =
            AccountId::try_from(request.match_info().get("account_id").unwrap().to_string())
                .map_err(|_| ServiceError::ArgumentError)?;

        tracing::debug!(target: TARGET_API, "Looking up non-fungible tokens for account_id: {}", account_id);

        let connection = app_state.redis_client.get_async_connection().await?;

        let query_result =
            database::query_with_prefix(connection, "nf", &account_id.to_string()).await?;

        Ok(web::Json(json!({
            "account_id": account_id,
            "contract_ids": query_result.into_iter().map(|(k, _v)| k).collect::<Vec<String>>(),
        })))
    }
}

pub mod exp {
    use super::*;

    #[get("/account/{account_id}/ft_with_balances")]
    pub async fn ft_with_balances(
        request: HttpRequest,
        app_state: web::Data<AppState>,
    ) -> Result<impl Responder, ServiceError> {
        let account_id =
            AccountId::try_from(request.match_info().get("account_id").unwrap().to_string())
                .map_err(|_| ServiceError::ArgumentError)?;

        tracing::debug!(target: TARGET_API, "Looking up fungible tokens for account_id: {}", account_id);

        let connection = app_state.redis_client.get_async_connection().await?;

        let account_id = account_id.to_string();

        let token_ids = database::query_with_prefix_parse(connection, "ft", &account_id).await?;

        let token_balances: HashMap<String, Option<String>> =
            rpc::get_ft_balances(&account_id, &token_ids).await?;

        Ok(web::Json(json!({
            "account_id": account_id,
            "tokens": token_balances,
        })))
    }
}

pub mod v1 {
    use super::*;

    #[get("/account/{account_id}/staking")]
    pub async fn staking(
        request: HttpRequest,
        app_state: web::Data<AppState>,
    ) -> Result<impl Responder, ServiceError> {
        let account_id =
            AccountId::try_from(request.match_info().get("account_id").unwrap().to_string())
                .map_err(|_| ServiceError::ArgumentError)?;

        tracing::debug!(target: TARGET_API, "Looking up validators for account_id: {}", account_id);

        let connection = app_state.redis_client.get_async_connection().await?;

        let query_result =
            database::query_with_prefix_parse(connection, "st", &account_id.to_string()).await?;

        Ok(web::Json(json!({
            "account_id": account_id,
            "pools": query_result.into_iter().map(|(pool_id, last_update_block_height)| json!({
                "pool_id": pool_id,
                "last_update_block_height": last_update_block_height,
            })).collect::<Vec<_>>()
        })))
    }

    #[get("/account/{account_id}/ft")]
    pub async fn ft(
        request: HttpRequest,
        app_state: web::Data<AppState>,
    ) -> Result<impl Responder, ServiceError> {
        let account_id =
            AccountId::try_from(request.match_info().get("account_id").unwrap().to_string())
                .map_err(|_| ServiceError::ArgumentError)?;

        tracing::debug!(target: TARGET_API, "Looking up fungible tokens for account_id: {}", account_id);

        let connection = app_state.redis_client.get_async_connection().await?;

        let query_result =
            database::query_with_prefix_parse(connection, "ft", &account_id.to_string()).await?;

        Ok(web::Json(json!({
            "account_id": account_id,
            "tokens": query_result.into_iter().map(|(contract_id, last_update_block_height)| json!({
                "contract_id": contract_id,
                "last_update_block_height": last_update_block_height,
            })).collect::<Vec<_>>()
        })))
    }

    #[get("/account/{account_id}/nft")]
    pub async fn nft(
        request: HttpRequest,
        app_state: web::Data<AppState>,
    ) -> Result<impl Responder, ServiceError> {
        let account_id =
            AccountId::try_from(request.match_info().get("account_id").unwrap().to_string())
                .map_err(|_| ServiceError::ArgumentError)?;

        tracing::debug!(target: TARGET_API, "Looking up non-fungible tokens for account_id: {}", account_id);

        let connection = app_state.redis_client.get_async_connection().await?;

        let query_result =
            database::query_with_prefix_parse(connection, "nf", &account_id.to_string()).await?;

        Ok(web::Json(json!({
            "account_id": account_id,
            "tokens": query_result.into_iter().map(|(contract_id, last_update_block_height)| json!({
                "contract_id": contract_id,
                "last_update_block_height": last_update_block_height,
            })).collect::<Vec<_>>()
        })))
    }
}
