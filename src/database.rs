use std::env;

use clickhouse::Client;
use serde_repr::{Deserialize_repr, Serialize_repr};

const LIMIT: u64 = 100;

const TARGET_DB: &str = "database";

#[derive(Debug)]
pub enum DatabaseError {
    ClickhouseError(clickhouse::error::Error),
    RedisError(redis::RedisError),
}

impl From<clickhouse::error::Error> for DatabaseError {
    fn from(error: clickhouse::error::Error) -> Self {
        DatabaseError::ClickhouseError(error)
    }
}

impl From<redis::RedisError> for DatabaseError {
    fn from(error: redis::RedisError) -> Self {
        DatabaseError::RedisError(error)
    }
}

#[derive(Copy, Clone, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ReceiptStatus {
    Failure = 1,
    Success = 2,
}

#[derive(Copy, Clone, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ActionKind {
    CreateAccount = 1,
    DeployContract = 2,
    FunctionCall = 3,
    Transfer = 4,
    Stake = 5,
    AddKey = 6,
    DeleteKey = 7,
    DeleteAccount = 8,
    Delegate = 9,
}

pub(crate) fn establish_connection() -> Client {
    Client::default()
        .with_url(env::var("DATABASE_URL").unwrap())
        .with_user(env::var("DATABASE_USER").unwrap())
        .with_password(env::var("DATABASE_PASSWORD").unwrap())
        .with_database(env::var("DATABASE_DATABASE").unwrap())
}

pub(crate) async fn query_account_by_public_key(
    client: &Client,
    public_key: &str,
    all_public_keys: bool,
) -> Result<Vec<String>, DatabaseError> {
    let start = std::time::Instant::now();
    let res = client
        .query(&format!("SELECT account_id FROM actions WHERE public_key = ? and status = ? and action = ? {}order by block_height desc limit ?", if !all_public_keys { "and access_key_contract_id IS NULL "} else { "" }))
        .bind(public_key)
        .bind(ReceiptStatus::Success)
        .bind(ActionKind::AddKey)
        .bind(LIMIT)
        .fetch_all::<String>()
        .await;

    let duration = start.elapsed().as_millis();

    tracing::debug!(target: TARGET_DB, "Query {}ms: query_account_by_public_key (all {}) {}",
        duration,
        all_public_keys,
        public_key);

    Ok(res?)
}

pub(crate) async fn query_with_prefix(
    mut connection: redis::aio::Connection,
    prefix: &str,
    account_id: &str,
) -> Result<Vec<String>, DatabaseError> {
    let start = std::time::Instant::now();

    let res: redis::RedisResult<Vec<(String, String)>> = redis::cmd("HGETALL")
        .arg(format!("{}:{}", prefix, account_id))
        .query_async(&mut connection)
        .await;

    let duration = start.elapsed().as_millis();

    tracing::debug!(target: TARGET_DB, "Query {}ms: query_with_prefix {}:{}",
        duration,
        prefix,
        account_id);

    Ok(res?.into_iter().map(|(k, _)| k).collect())
}
