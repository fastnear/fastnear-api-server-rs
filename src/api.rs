use crate::database::ActionRow;
use crate::*;
use actix_web::ResponseError;
use serde_json::json;
use std::fmt;

const TARGET_API: &str = "api";

#[derive(Debug)]
enum ServiceError {
    DatabaseError(database::DatabaseError),
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
        }
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json("Internal server error")
            }
        }
    }
}

#[get("/lookup/public_key/{public_key}")]
pub async fn lookup_by_public_key(
    request: HttpRequest,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, ServiceError> {
    let public_key = request
        .match_info()
        .get("public_key")
        .unwrap()
        .parse::<String>()
        .unwrap();

    tracing::debug!(target: TARGET_API, "Looking up account_ids for public_key: {}", public_key);

    let query_result: Vec<ActionRow> =
        database::query_account_by_public_key(&app_state.db, &public_key).await?;

    Ok(web::Json(json!({
        "public_key": public_key,
        "account_ids": query_result.into_iter().map(|row| row.account_id).collect::<Vec<_>>(),
    })))
}
