use base64::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::time::Duration;

const RPC_TIMEOUT: Duration = Duration::from_secs(10);
const TARGET_RPC: &str = "rpc";

#[derive(Debug)]
pub enum RpcError {
    ReqwestError(reqwest::Error),
    InvalidJsonRpcResponse,
    InvalidFunctionCallResponse,
}

impl From<reqwest::Error> for RpcError {
    fn from(error: reqwest::Error) -> Self {
        RpcError::ReqwestError(error)
    }
}

#[derive(Serialize)]
struct JsonRequest {
    jsonrpc: String,
    method: String,
    params: Value,
    id: String,
}

#[derive(Deserialize)]
struct JsonResponse {
    id: String,
    // jsonrpc: String,
    result: Option<Value>,
    // error: Option<Value>,
}

#[derive(Deserialize)]
struct FunctionCallResponse {
    // block_hash: String,
    // block_height: u64,
    result: Vec<u8>,
}

pub(crate) async fn get_ft_balances(
    account_id: &str,
    token_ids: &[String],
) -> Result<HashMap<String, Option<String>>, RpcError> {
    let start = std::time::Instant::now();
    let client = Client::new();
    let request = token_ids
        .iter()
        .enumerate()
        .map(|(id, token_id)| JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "query".to_string(),
            params: json!({
                "request_type": "call_function",
                "finality": "final",
                "account_id": token_id,
                "method_name": "ft_balance_of",
                "args_base64": BASE64_STANDARD.encode(format!("{{\"account_id\": \"{}\"}}", account_id)),
            }),
            id: id.to_string(),
        })
        .collect::<Vec<_>>();
    let response = client
        .post("https://beta.rpc.mainnet.near.org")
        .json(&request)
        .timeout(RPC_TIMEOUT)
        .send()
        .await?;
    let responses = response.json::<Vec<JsonResponse>>().await?;
    let mut token_balances = HashMap::new();
    for response in responses {
        let id: usize = response
            .id
            .parse()
            .map_err(|_| RpcError::InvalidJsonRpcResponse)?;
        let token_id = token_ids
            .get(id)
            .ok_or(RpcError::InvalidJsonRpcResponse)?
            .clone();
        let balance = if let Some(res) = response.result {
            let fc: FunctionCallResponse =
                serde_json::from_value(res).map_err(|_| RpcError::InvalidFunctionCallResponse)?;
            let balance: Option<String> = serde_json::from_slice(&fc.result).ok();
            let parsed_balance: Option<u128> = balance.and_then(|s| s.parse().ok());
            parsed_balance.map(|b| b.to_string())
        } else {
            None
        };
        token_balances.insert(token_id, balance);
    }
    let duration = start.elapsed().as_millis();

    tracing::debug!(target: TARGET_RPC, "Query {}ms: get_ft_balances {} with {} tokens",
        duration,
        account_id,
        token_ids.len());

    Ok(token_balances)
}
