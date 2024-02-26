use crate::*;
use actix_web::ResponseError;
use near_account_id::AccountId;
use near_crypto::PublicKey;
use serde_json::json;
use std::fmt;
use std::str::FromStr;

const TARGET_API: &str = "api";

#[derive(Debug)]
enum ServiceError {
    DatabaseError(database::DatabaseError),
    ArgumentError,
}

impl From<database::DatabaseError> for ServiceError {
    fn from(error: database::DatabaseError) -> Self {
        ServiceError::DatabaseError(error)
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServiceError::DatabaseError(ref message) => write!(f, "Database Error: {:?}", message),
            ServiceError::ArgumentError => write!(f, "Invalid argument"),
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
        }
    }
}

#[get("/public_key/{public_key}")]
pub async fn lookup_by_public_key(
    request: HttpRequest,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, ServiceError> {
    let public_key = PublicKey::from_str(request.match_info().get("public_key").unwrap())
        .map_err(|_| ServiceError::ArgumentError)?;

    tracing::debug!(target: TARGET_API, "Looking up account_ids for public_key: {}", public_key);

    let query_result: Vec<String> =
        database::query_account_by_public_key(&app_state.db, &public_key.to_string()).await?;

    Ok(web::Json(json!({
        "public_key": public_key,
        "account_ids": query_result,
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

    let query_result: Vec<String> = database::query_with_prefix(
        &mut app_state.redis_db.lock().expect("Lock poisoning"),
        "st",
        &account_id.to_string(),
    )
    .await?;

    Ok(web::Json(json!({
        "account_id": account_id,
        "pools": query_result,
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

    let query_result: Vec<String> = database::query_with_prefix(
        &mut app_state.redis_db.lock().expect("Lock poisoning"),
        "ft",
        &account_id.to_string(),
    )
    .await?;

    Ok(web::Json(json!({
        "account_id": account_id,
        "contract_ids": query_result,
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

    let query_result: Vec<String> = database::query_with_prefix(
        &mut app_state.redis_db.lock().expect("Lock poisoning"),
        "nf",
        &account_id.to_string(),
    )
    .await?;

    Ok(web::Json(json!({
        "account_id": account_id,
        "contract_ids": query_result,
    })))
}
