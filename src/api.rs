use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use actix_web::{get, web, HttpRequest, HttpResponse, Responder, ResponseError};
use near_account_id::AccountId;
use near_crypto::PublicKey;

use crate::types::{
    parse_account_state, AccountBalanceRow, AccountFullResponse, ExpFtWithBalancesResponse, NftRow,
    PoolRow, PublicKeyLookupResponse, TokenAccountsResponse, TokenRow, V0ContractsResponse,
    V0StakingResponse, V1FtResponse, V1NftResponse, V1StakingResponse,
};
use crate::{database, rpc, AppState};

const TARGET_API: &str = "api";

#[derive(Debug)]
pub enum ServiceError {
    DatabaseError(database::DatabaseError),
    RpcError(rpc::RpcError),
    ArgumentError,
}

#[derive(Debug)]
pub enum HealthError {
    HighSyncLatency {
        latency: f64,
        max_latency: f64,
    },
    MissingSyncLatency,
    MissingSyncBlockHeight,
    MissingSyncBalanceBlockHeight,
    HighSyncBlockDiff {
        sync_difference: u64,
        max_sync_difference: u64,
    },
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
                tracing::error!(target: TARGET_API, "Service error: {}", self);
                HttpResponse::InternalServerError().json("Internal server error")
            }
            ServiceError::ArgumentError => {
                tracing::info!(target: TARGET_API, "Service error: {}", self);
                HttpResponse::BadRequest().json("Invalid argument")
            }
            ServiceError::RpcError(ref e) => {
                tracing::error!(target: TARGET_API, "Service error: {}", self);
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

        let mut connection = app_state
            .redis_client
            .get_multiplexed_async_connection()
            .await?;

        let public_key = public_key.to_string();

        let account_ids = database::query_with_prefix(&mut connection, "pk", &public_key).await?;

        Ok(web::Json(PublicKeyLookupResponse {
            public_key,
            account_ids: account_ids
                .into_iter()
                .filter_map(|(k, v)| if v == "f" { Some(k) } else { None })
                .collect(),
        }))
    }

    #[get("/public_key/{public_key}/all")]
    pub async fn lookup_by_public_key_all(
        request: HttpRequest,
        app_state: web::Data<AppState>,
    ) -> Result<impl Responder, ServiceError> {
        let public_key = PublicKey::from_str(request.match_info().get("public_key").unwrap())
            .map_err(|_| ServiceError::ArgumentError)?;

        tracing::debug!(target: TARGET_API, "Looking up account_ids for all public_key: {}", public_key);

        let mut connection = app_state
            .redis_client
            .get_multiplexed_async_connection()
            .await?;

        let public_key = public_key.to_string();

        let account_ids = database::query_with_prefix(&mut connection, "pk", &public_key).await?;

        Ok(web::Json(PublicKeyLookupResponse {
            public_key,
            account_ids: account_ids.into_iter().map(|(k, _v)| k).collect(),
        }))
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

        let mut connection = app_state
            .redis_client
            .get_multiplexed_async_connection()
            .await?;

        let query_result =
            database::query_with_prefix(&mut connection, "st", &account_id.to_string()).await?;

        Ok(web::Json(V0StakingResponse {
            account_id: account_id.to_string(),
            pools: query_result.into_iter().map(|(k, _v)| k).collect(),
        }))
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

        let mut connection = app_state
            .redis_client
            .get_multiplexed_async_connection()
            .await?;

        let query_result =
            database::query_with_prefix(&mut connection, "ft", &account_id.to_string()).await?;

        Ok(web::Json(V0ContractsResponse {
            account_id: account_id.to_string(),
            contract_ids: query_result.into_iter().map(|(k, _v)| k).collect(),
        }))
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

        let mut connection = app_state
            .redis_client
            .get_multiplexed_async_connection()
            .await?;

        let query_result =
            database::query_with_prefix(&mut connection, "nf", &account_id.to_string()).await?;

        Ok(web::Json(V0ContractsResponse {
            account_id: account_id.to_string(),
            contract_ids: query_result.into_iter().map(|(k, _v)| k).collect(),
        }))
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

        let mut connection = app_state
            .redis_client
            .get_multiplexed_async_connection()
            .await?;

        let account_id = account_id.to_string();

        let token_ids =
            database::query_with_prefix_parse(&mut connection, "ft", &account_id).await?;

        let token_balances: HashMap<String, Option<String>> =
            rpc::get_ft_balances(&account_id, &token_ids).await?;

        Ok(web::Json(ExpFtWithBalancesResponse {
            account_id,
            tokens: token_balances,
        }))
    }

    #[get("/ft/{token_id}/all")]
    pub async fn ft_all(
        request: HttpRequest,
        app_state: web::Data<AppState>,
    ) -> Result<impl Responder, ServiceError> {
        let token_id =
            AccountId::try_from(request.match_info().get("token_id").unwrap().to_string())
                .map_err(|_| ServiceError::ArgumentError)?;

        tracing::debug!(target: TARGET_API, "Retrieving all holders for token: {}", token_id);

        let mut connection = app_state
            .redis_client
            .get_multiplexed_async_connection()
            .await?;

        let token_id = token_id.to_string();

        let tokens_with_balances =
            database::query_with_prefix(&mut connection, "b", &token_id).await?;

        Ok(web::Json(TokenAccountsResponse {
            token_id,
            accounts: tokens_with_balances
                .into_iter()
                .map(|(account_id, balance)| AccountBalanceRow {
                    account_id,
                    balance: Some(balance),
                })
                .collect(),
        }))
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

        let mut connection = app_state
            .redis_client
            .get_multiplexed_async_connection()
            .await?;

        let query_result =
            database::query_with_prefix_parse(&mut connection, "st", &account_id.to_string())
                .await?;

        Ok(web::Json(V1StakingResponse {
            account_id: account_id.to_string(),
            pools: query_result
                .into_iter()
                .map(|(pool_id, last_update_block_height)| PoolRow {
                    pool_id,
                    last_update_block_height,
                })
                .collect(),
        }))
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

        let mut connection = app_state
            .redis_client
            .get_multiplexed_async_connection()
            .await?;

        let account_id = account_id.to_string();

        let query_result =
            database::query_with_prefix_parse(&mut connection, "ft", &account_id).await?;
        let balances = database::query_balances(
            &mut connection,
            query_result
                .iter()
                .map(|(token_id, _)| (token_id.as_str(), account_id.as_str()))
                .collect::<Vec<_>>()
                .as_slice(),
        )
        .await?;

        Ok(web::Json(V1FtResponse {
            account_id,
            tokens: query_result
                .into_iter()
                .zip(balances.into_iter())
                .map(
                    |((contract_id, last_update_block_height), balance)| TokenRow {
                        contract_id,
                        last_update_block_height,
                        balance,
                    },
                )
                .collect(),
        }))
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

        let mut connection = app_state
            .redis_client
            .get_multiplexed_async_connection()
            .await?;

        let query_result =
            database::query_with_prefix_parse(&mut connection, "nf", &account_id.to_string())
                .await?;

        Ok(web::Json(V1NftResponse {
            account_id: account_id.to_string(),
            tokens: query_result
                .into_iter()
                .map(|(contract_id, last_update_block_height)| NftRow {
                    contract_id,
                    last_update_block_height,
                })
                .collect(),
        }))
    }

    #[get("/ft/{token_id}/top")]
    pub async fn ft_top(
        request: HttpRequest,
        app_state: web::Data<AppState>,
    ) -> Result<impl Responder, ServiceError> {
        let token_id =
            AccountId::try_from(request.match_info().get("token_id").unwrap().to_string())
                .map_err(|_| ServiceError::ArgumentError)?;

        tracing::debug!(target: TARGET_API, "Retrieving top holders for token: {}", token_id);

        let mut connection = app_state
            .redis_client
            .get_multiplexed_async_connection()
            .await?;

        let token_id = token_id.to_string();

        let query_result =
            database::query_zset_by_score(&mut connection, &format!("tb:{}", token_id), 100)
                .await?;
        let balances = database::query_balances(
            &mut connection,
            query_result
                .iter()
                .map(|account_id| (token_id.as_str(), account_id.as_str()))
                .collect::<Vec<_>>()
                .as_slice(),
        )
        .await?;

        let mut top_holders = query_result
            .into_iter()
            .zip(balances.into_iter())
            .collect::<Vec<_>>();

        top_holders.sort_unstable_by(|a, b| {
            (
                b.1.as_ref()
                    .and_then(|b| b.parse::<u128>().ok())
                    .unwrap_or(0),
                &b.0,
            )
                .cmp(&(
                    a.1.as_ref()
                        .and_then(|b| b.parse::<u128>().ok())
                        .unwrap_or(0),
                    &a.0,
                ))
        });

        Ok(web::Json(TokenAccountsResponse {
            token_id,
            accounts: top_holders
                .into_iter()
                .map(|(account_id, balance)| AccountBalanceRow {
                    account_id,
                    balance,
                })
                .collect(),
        }))
    }

    #[get("/account/{account_id}/full")]
    pub async fn account_full(
        request: HttpRequest,
        app_state: web::Data<AppState>,
    ) -> Result<impl Responder, ServiceError> {
        let account_id =
            AccountId::try_from(request.match_info().get("account_id").unwrap().to_string())
                .map_err(|_| ServiceError::ArgumentError)?;

        tracing::debug!(target: TARGET_API, "Looking full data for account_id: {}", account_id);

        let mut connection = app_state
            .redis_client
            .get_multiplexed_async_connection()
            .await?;

        let account_id = account_id.to_string();

        let query_result =
            database::query_with_prefix_parse(&mut connection, "st", &account_id.to_string())
                .await?;

        let pools = query_result
            .into_iter()
            .map(|(pool_id, last_update_block_height)| PoolRow {
                pool_id,
                last_update_block_height,
            })
            .collect::<Vec<_>>();

        let query_result =
            database::query_with_prefix_parse(&mut connection, "ft", &account_id).await?;
        let balances = database::query_balances(
            &mut connection,
            query_result
                .iter()
                .map(|(token_id, _)| (token_id.as_str(), account_id.as_str()))
                .collect::<Vec<_>>()
                .as_slice(),
        )
        .await?;
        let tokens = query_result
            .into_iter()
            .zip(balances.into_iter())
            .map(
                |((contract_id, last_update_block_height), balance)| TokenRow {
                    contract_id,
                    last_update_block_height,
                    balance,
                },
            )
            .collect::<Vec<_>>();

        let query_result =
            database::query_with_prefix_parse(&mut connection, "nf", &account_id.to_string())
                .await?;

        let nfts = query_result
            .into_iter()
            .map(|(contract_id, last_update_block_height)| NftRow {
                contract_id,
                last_update_block_height,
            })
            .collect::<Vec<_>>();

        let state = parse_account_state(
            database::query_hget(&mut connection, "accounts", &account_id).await?,
        );

        Ok(web::Json(AccountFullResponse {
            account_id,
            pools,
            tokens,
            nfts,
            state,
        }))
    }
}
