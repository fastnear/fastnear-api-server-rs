use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub type BlockHeight = u64;

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct StatusResponse {
    pub version: String,
    pub sync_block_height: Option<BlockHeight>,
    pub sync_latency_sec: Option<f64>,
    pub sync_block_timestamp_nanosec: Option<String>,
    pub sync_balance_block_height: Option<BlockHeight>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct HealthResponse {
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct PublicKeyLookupResponse {
    pub public_key: String,
    pub account_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct V0StakingResponse {
    pub account_id: String,
    pub pools: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct V0ContractsResponse {
    pub account_id: String,
    pub contract_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct PoolRow {
    pub pool_id: String,
    pub last_update_block_height: Option<BlockHeight>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct TokenRow {
    pub contract_id: String,
    pub last_update_block_height: Option<BlockHeight>,
    pub balance: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct NftRow {
    pub contract_id: String,
    pub last_update_block_height: Option<BlockHeight>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct V1StakingResponse {
    pub account_id: String,
    pub pools: Vec<PoolRow>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct V1FtResponse {
    pub account_id: String,
    pub tokens: Vec<TokenRow>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct V1NftResponse {
    pub account_id: String,
    pub tokens: Vec<NftRow>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct AccountBalanceRow {
    pub account_id: String,
    pub balance: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct TokenAccountsResponse {
    pub token_id: String,
    pub accounts: Vec<AccountBalanceRow>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct AccountStateResponse {
    pub balance: Option<String>,
    pub locked: Option<String>,
    pub storage_bytes: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct AccountFullResponse {
    pub account_id: String,
    pub pools: Vec<PoolRow>,
    pub tokens: Vec<TokenRow>,
    pub nfts: Vec<NftRow>,
    pub state: Option<AccountStateResponse>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "openapi", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "openapi", schemars(deny_unknown_fields))]
pub struct ExpFtWithBalancesResponse {
    pub account_id: String,
    pub tokens: HashMap<String, Option<String>>,
}

#[derive(Debug, Deserialize)]
struct StoredAccountState {
    #[serde(rename = "b", default)]
    balance: Option<String>,
    #[serde(rename = "l", default)]
    locked: Option<String>,
    #[serde(rename = "s", default)]
    storage_bytes: Option<u64>,
}

pub fn parse_account_state(raw_state: Option<String>) -> Option<AccountStateResponse> {
    raw_state.and_then(|state| {
        if state.is_empty() {
            return None;
        }

        serde_json::from_str::<StoredAccountState>(&state)
            .ok()
            .map(|stored| AccountStateResponse {
                balance: stored.balance,
                locked: stored.locked,
                storage_bytes: stored.storage_bytes,
            })
    })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::json;

    use super::{
        parse_account_state, AccountBalanceRow, AccountFullResponse, ExpFtWithBalancesResponse,
        HealthResponse, NftRow, PoolRow, PublicKeyLookupResponse, StatusResponse,
        TokenAccountsResponse, TokenRow, V0ContractsResponse, V0StakingResponse,
    };

    #[test]
    fn system_responses_preserve_nullable_wire_shape() {
        let status = StatusResponse {
            version: "0.10.1".to_string(),
            sync_block_height: Some(193_473_935),
            sync_latency_sec: Some(2.159742085),
            sync_block_timestamp_nanosec: Some("1775874067605835029".to_string()),
            sync_balance_block_height: Some(193_473_935),
        };
        let health = HealthResponse {
            status: "ok".to_string(),
        };

        assert_eq!(
            serde_json::to_value(status).unwrap(),
            json!({
                "version": "0.10.1",
                "sync_block_height": 193473935,
                "sync_latency_sec": 2.159742085,
                "sync_block_timestamp_nanosec": "1775874067605835029",
                "sync_balance_block_height": 193473935
            })
        );
        assert_eq!(
            serde_json::to_value(health).unwrap(),
            json!({ "status": "ok" })
        );
    }

    #[test]
    fn public_key_and_v0_responses_match_current_wire_format() {
        let public_key = PublicKeyLookupResponse {
            public_key: "ed25519:test".to_string(),
            account_ids: vec!["example.near".to_string()],
        };
        let staking = V0StakingResponse {
            account_id: "root.near".to_string(),
            pools: vec!["zavodil.poolv1.near".to_string()],
        };
        let contracts = V0ContractsResponse {
            account_id: "root.near".to_string(),
            contract_ids: vec!["wrap.near".to_string()],
        };

        assert_eq!(
            serde_json::to_value(public_key).unwrap(),
            json!({
                "public_key": "ed25519:test",
                "account_ids": ["example.near"]
            })
        );
        assert_eq!(
            serde_json::to_value(staking).unwrap(),
            json!({
                "account_id": "root.near",
                "pools": ["zavodil.poolv1.near"]
            })
        );
        assert_eq!(
            serde_json::to_value(contracts).unwrap(),
            json!({
                "account_id": "root.near",
                "contract_ids": ["wrap.near"]
            })
        );
    }

    #[test]
    fn account_full_and_top_holder_responses_preserve_nullable_fields() {
        let token_accounts = TokenAccountsResponse {
            token_id: "first.tkn.near".to_string(),
            accounts: vec![AccountBalanceRow {
                account_id: "mob.near".to_string(),
                balance: Some("979894691374420631019486155".to_string()),
            }],
        };
        let full = AccountFullResponse {
            account_id: "here.tg".to_string(),
            pools: vec![PoolRow {
                pool_id: "here.poolv1.near".to_string(),
                last_update_block_height: None,
            }],
            tokens: vec![TokenRow {
                contract_id: "v1.omni.hot.tg".to_string(),
                last_update_block_height: Some(128_025_061),
                balance: Some(String::new()),
            }],
            nfts: vec![NftRow {
                contract_id: "nft.hot.tg".to_string(),
                last_update_block_height: Some(115_282_010),
            }],
            state: Some(
                parse_account_state(Some(
                    r#"{"b":"323562725144261345105389596","l":"0","s":29720}"#.to_string(),
                ))
                .unwrap(),
            ),
        };

        assert_eq!(
            serde_json::to_value(token_accounts).unwrap(),
            json!({
                "token_id": "first.tkn.near",
                "accounts": [
                    {
                        "account_id": "mob.near",
                        "balance": "979894691374420631019486155"
                    }
                ]
            })
        );
        assert_eq!(
            serde_json::to_value(full).unwrap(),
            json!({
                "account_id": "here.tg",
                "pools": [
                    {
                        "pool_id": "here.poolv1.near",
                        "last_update_block_height": null
                    }
                ],
                "tokens": [
                    {
                        "contract_id": "v1.omni.hot.tg",
                        "last_update_block_height": 128025061,
                        "balance": ""
                    }
                ],
                "nfts": [
                    {
                        "contract_id": "nft.hot.tg",
                        "last_update_block_height": 115282010
                    }
                ],
                "state": {
                    "balance": "323562725144261345105389596",
                    "locked": "0",
                    "storage_bytes": 29720
                }
            })
        );
    }

    #[test]
    fn exp_shapes_preserve_map_and_row_formats() {
        let mut tokens = HashMap::new();
        tokens.insert(
            "wrap.near".to_string(),
            Some("4200000000000000000000000".to_string()),
        );
        tokens.insert("usdt.tether-token.near".to_string(), None);
        let response = ExpFtWithBalancesResponse {
            account_id: "here.tg".to_string(),
            tokens,
        };

        let json = serde_json::to_value(response).unwrap();
        assert_eq!(json["account_id"], "here.tg");
        assert_eq!(json["tokens"]["wrap.near"], "4200000000000000000000000");
        assert!(json["tokens"]["usdt.tether-token.near"].is_null());
    }

    #[test]
    fn empty_or_invalid_account_state_stays_nullable() {
        assert!(parse_account_state(Some(String::new())).is_none());
        assert!(parse_account_state(Some("{".to_string())).is_none());
        assert_eq!(
            serde_json::to_value(
                parse_account_state(Some(r#"{"b":"1","l":"0"}"#.to_string())).unwrap()
            )
            .unwrap(),
            json!({
                "balance": "1",
                "locked": "0",
                "storage_bytes": null
            })
        );
    }
}
