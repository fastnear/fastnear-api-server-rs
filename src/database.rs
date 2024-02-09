use crate::*;
use std::env;

use clickhouse::{Client, Row};
use serde::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};

const TARGET_DB: &str = "database";

#[derive(Debug)]
pub enum DatabaseError {
    ClickhouseError(clickhouse::error::Error),
}

impl From<clickhouse::error::Error> for DatabaseError {
    fn from(error: clickhouse::error::Error) -> Self {
        DatabaseError::ClickhouseError(error)
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

#[derive(Row, Deserialize)]
pub struct ActionRow {
    pub block_height: u64,
    pub block_hash: String,
    pub block_timestamp: u64,
    pub receipt_id: String,
    pub receipt_index: u16,
    pub action_index: u8,
    pub signer_id: String,
    pub signer_public_key: String,
    pub predecessor_id: String,
    pub account_id: String,
    pub status: ReceiptStatus,
    pub action: ActionKind,
    pub contract_hash: Option<String>,
    pub public_key: Option<String>,
    pub access_key_contract_id: Option<String>,
    pub deposit: Option<u128>,
    pub gas_price: u128,
    pub attached_gas: Option<u64>,
    pub gas_burnt: u64,
    pub tokens_burnt: u128,
    pub method_name: Option<String>,
    pub args_account_id: Option<String>,
    pub args_new_account_id: Option<String>,
    pub args_owner_id: Option<String>,
    pub args_receiver_id: Option<String>,
    pub args_sender_id: Option<String>,
    pub args_token_id: Option<String>,
    pub args_amount: Option<u128>,
    pub args_balance: Option<u128>,
    pub args_nft_contract_id: Option<String>,
    pub args_nft_token_id: Option<String>,
    pub args_utm_source: Option<String>,
    pub args_utm_medium: Option<String>,
    pub args_utm_campaign: Option<String>,
    pub args_utm_term: Option<String>,
    pub args_utm_content: Option<String>,
    pub return_value_int: Option<u128>,
}

#[derive(Row, Deserialize)]
pub struct EventRow {
    pub block_height: u64,
    pub block_hash: String,
    pub block_timestamp: u64,
    pub receipt_id: String,
    pub receipt_index: u16,
    pub log_index: u16,
    pub signer_id: String,
    pub signer_public_key: String,
    pub predecessor_id: String,
    pub account_id: String,
    pub status: ReceiptStatus,

    pub version: Option<String>,
    pub standard: Option<String>,
    pub event: Option<String>,
    pub data_account_id: Option<String>,
    pub data_owner_id: Option<String>,
    pub data_old_owner_id: Option<String>,
    pub data_new_owner_id: Option<String>,
    pub data_liquidation_account_id: Option<String>,
    pub data_authorized_id: Option<String>,
    pub data_token_ids: Vec<String>,
    pub data_token_id: Option<String>,
    pub data_position: Option<String>,
    pub data_amount: Option<u128>,
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
) -> Result<Vec<ActionRow>, DatabaseError> {
    let start = std::time::Instant::now();
    let res = client
        .query("SELECT * FROM actions WHERE public_key = ? and status = ? and action = ? order by block_height desc limit 10")
        .bind(public_key)
        .bind(ReceiptStatus::Success)
        .bind(ActionKind::AddKey)
        .fetch_all::<ActionRow>()
        .await;

    let duration = start.elapsed().as_millis();

    tracing::debug!(target: TARGET_DB, "Query {}ms: query_account_by_public_key {}",
        duration,
        public_key);

    Ok(res?)
}
