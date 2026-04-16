use anyhow::Result;
use fastnear_openapi_generator::{
    build_service_doc, write_or_check_yaml, AggregateOperationSpec, ApiInfo, ApiServer,
    HttpMethod, ParameterLocation, ParameterSpec, ResponseContent, ResponseSpec, SchemaRegistry,
};
use schemars::JsonSchema;
use serde_json::{json, Value};

use crate::types::{
    AccountFullResponse, ExpFtWithBalancesResponse, HealthResponse, PublicKeyLookupResponse,
    StatusResponse, TokenAccountsResponse, V0ContractsResponse, V0StakingResponse, V1FtResponse,
    V1NftResponse, V1StakingResponse,
};

const API_VERSION: &str = "3.0.3";
const SERVICE_INFO: ApiInfo<'static> = ApiInfo {
    title: "FastNEAR API",
    version: API_VERSION,
    description: "Low-latency indexed account, token, and public-key lookup APIs for wallets and explorers. Embedded portal clients may forward an optional `apiKey` query parameter, but the public FastNEAR API does not require it.",
    servers: &[
        ApiServer {
            url: "https://api.fastnear.com",
            description: "Mainnet",
        },
        ApiServer {
            url: "https://test.api.fastnear.com",
            description: "Testnet",
        },
    ],
};

pub fn generate(check: bool, include_exp: bool) -> Result<()> {
    let output_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("openapi");
    let mut registry = SchemaRegistry::openapi3();

    let mut operations = vec![
        get_operation_spec::<StatusResponse>(
            &mut registry,
            "status",
            "FastNEAR API - Status",
            "/status",
            "get_status",
            "Get service sync status",
            "Check the current indexed block height, latency, and deployed service version.",
            &["system"],
            vec![api_key_parameter()],
            "Current FastNEAR API sync status",
            Some(json!({
                "sync_balance_block_height": 129734103,
                "sync_block_height": 129734103,
                "sync_block_timestamp_nanosec": "1728256282197171397",
                "sync_latency_sec": 4.671730603,
                "version": "0.10.0"
            })),
            false,
        ),
        get_operation_spec::<HealthResponse>(
            &mut registry,
            "health",
            "FastNEAR API - Health",
            "/health",
            "get_health",
            "Get service health",
            "Use this lightweight probe to confirm the FastNear API is healthy.",
            &["system"],
            vec![api_key_parameter()],
            "Health status string",
            Some(json!({ "status": "ok" })),
            false,
        ),
        get_operation_spec::<PublicKeyLookupResponse>(
            &mut registry,
            "public_key_lookup",
            "FastNEAR API - V0 Public Key Lookup",
            "/v0/public_key/{public_key}",
            "lookup_by_public_key_v0",
            "Lookup full-access accounts by public key",
            "Fetch the indexed account IDs associated with a full-access public key.",
            &["public-key"],
            with_api_key(vec![path_parameter(
                "public_key",
                "NEAR public key in `ed25519:...` or `secp256k1:...` form.",
                json!("ed25519:CCaThr3uokqnUs6Z5vVnaDcJdrfuTpYJHJWcAGubDjT"),
            )]),
            "Matching account IDs for the supplied full-access public key",
            Some(json!({
                "public_key": "ed25519:CCaThr3uokqnUs6Z5vVnaDcJdrfuTpYJHJWcAGubDjT",
                "account_ids": ["root.near"]
            })),
            true,
        ),
        get_operation_spec::<PublicKeyLookupResponse>(
            &mut registry,
            "public_key_lookup_all",
            "FastNEAR API - V0 Public Key Lookup (All)",
            "/v0/public_key/{public_key}/all",
            "lookup_by_public_key_all_v0",
            "Lookup all indexed accounts by public key",
            "Use this variant when one public key may control multiple accounts and you want the full set.",
            &["public-key"],
            with_api_key(vec![path_parameter(
                "public_key",
                "NEAR public key in `ed25519:...` or `secp256k1:...` form.",
                json!("ed25519:CCaThr3uokqnUs6Z5vVnaDcJdrfuTpYJHJWcAGubDjT"),
            )]),
            "Matching account IDs for the supplied public key, including limited-access keys",
            Some(json!({
                "public_key": "ed25519:CCaThr3uokqnUs6Z5vVnaDcJdrfuTpYJHJWcAGubDjT",
                "account_ids": ["root.near"]
            })),
            true,
        ),
        get_operation_spec::<V0StakingResponse>(
            &mut registry,
            "account_staking",
            "FastNEAR API - V0 Account Staking",
            "/v0/account/{account_id}/staking",
            "account_staking_v0",
            "Lookup staking pool account IDs for an account",
            "Fetch the staking pool account IDs FastNear has indexed for one account. This v0 route returns pool IDs only, without block-height metadata.",
            &["staking"],
            with_api_key(vec![path_parameter(
                "account_id",
                "NEAR account ID to inspect.",
                json!("mob.near"),
            )]),
            "Staking pool account IDs for the requested account",
            Some(json!({
                "account_id": "mob.near",
                "pools": ["zavodil.poolv1.near"]
            })),
            true,
        ),
        get_operation_spec::<V0ContractsResponse>(
            &mut registry,
            "account_ft",
            "FastNEAR API - V0 Account FT",
            "/v0/account/{account_id}/ft",
            "account_ft_v0",
            "Lookup fungible token contract IDs for an account",
            "Retrieve the indexed fungible token contract IDs for an account. This v0 route does not return balances.",
            &["fungible-tokens"],
            with_api_key(vec![path_parameter(
                "account_id",
                "NEAR account ID to inspect.",
                json!("here.tg"),
            )]),
            "Fungible token contract IDs for the requested account",
            Some(json!({
                "account_id": "here.tg",
                "contract_ids": ["wrap.near", "usdt.tether-token.near"]
            })),
            true,
        ),
        get_operation_spec::<V0ContractsResponse>(
            &mut registry,
            "account_nft",
            "FastNEAR API - V0 Account NFT",
            "/v0/account/{account_id}/nft",
            "account_nft_v0",
            "Lookup NFT contract IDs for an account",
            "Fetch the indexed NFT contract IDs associated with an account. This v0 route does not include block-height metadata.",
            &["non-fungible-tokens"],
            with_api_key(vec![path_parameter(
                "account_id",
                "NEAR account ID to inspect.",
                json!("sharddog.near"),
            )]),
            "NFT contract IDs for the requested account",
            Some(json!({
                "account_id": "sharddog.near",
                "contract_ids": ["nft.example.near"]
            })),
            true,
        ),
        get_operation_spec::<PublicKeyLookupResponse>(
            &mut registry,
            "public_key_lookup",
            "FastNEAR API - V1 Public Key Lookup",
            "/v1/public_key/{public_key}",
            "lookup_by_public_key_v1",
            "Lookup full-access accounts by public key",
            "Use the v1 endpoint for the newer namespace. It currently returns the same response shape as the v0 route.",
            &["public-key"],
            with_api_key(vec![path_parameter(
                "public_key",
                "NEAR public key in `ed25519:...` or `secp256k1:...` form.",
                json!("ed25519:CCaThr3uokqnUs6Z5vVnaDcJdrfuTpYJHJWcAGubDjT"),
            )]),
            "Matching account IDs for the supplied full-access public key",
            Some(json!({
                "public_key": "ed25519:CCaThr3uokqnUs6Z5vVnaDcJdrfuTpYJHJWcAGubDjT",
                "account_ids": ["root.near"]
            })),
            true,
        ),
        get_operation_spec::<PublicKeyLookupResponse>(
            &mut registry,
            "public_key_lookup_all",
            "FastNEAR API - V1 Public Key Lookup (All)",
            "/v1/public_key/{public_key}/all",
            "lookup_by_public_key_all_v1",
            "Lookup all indexed accounts by public key",
            "Fetch every indexed account tied to a public key. This v1 route currently uses the same response shape as the v0 route.",
            &["public-key"],
            with_api_key(vec![path_parameter(
                "public_key",
                "NEAR public key in `ed25519:...` or `secp256k1:...` form.",
                json!("ed25519:CCaThr3uokqnUs6Z5vVnaDcJdrfuTpYJHJWcAGubDjT"),
            )]),
            "Matching account IDs for the supplied public key, including limited-access keys",
            Some(json!({
                "public_key": "ed25519:CCaThr3uokqnUs6Z5vVnaDcJdrfuTpYJHJWcAGubDjT",
                "account_ids": ["root.near"]
            })),
            true,
        ),
        get_operation_spec::<V1StakingResponse>(
            &mut registry,
            "account_staking",
            "FastNEAR API - V1 Account Staking",
            "/v1/account/{account_id}/staking",
            "account_staking_v1",
            "Lookup indexed staking pools for an account",
            "Retrieve staking pool rows for an account, including block-height metadata for each pool relationship.",
            &["staking"],
            with_api_key(vec![path_parameter(
                "account_id",
                "NEAR account ID to inspect.",
                json!("mob.near"),
            )]),
            "Indexed staking pool rows for the requested account",
            Some(json!({
                "account_id": "mob.near",
                "pools": [
                    {
                        "pool_id": "zavodil.poolv1.near",
                        "last_update_block_height": null
                    }
                ]
            })),
            true,
        ),
        get_operation_spec::<V1FtResponse>(
            &mut registry,
            "account_ft",
            "FastNEAR API - V1 Account FT",
            "/v1/account/{account_id}/ft",
            "account_ft_v1",
            "Lookup indexed fungible token rows for an account",
            "Fetch the v1 indexed fungible token balance rows for one account.",
            &["fungible-tokens"],
            with_api_key(vec![path_parameter(
                "account_id",
                "NEAR account ID to inspect.",
                json!("here.tg"),
            )]),
            "Indexed fungible token rows for the requested account",
            Some(json!({
                "account_id": "here.tg",
                "tokens": [
                    {
                        "contract_id": "wrap.near",
                        "last_update_block_height": null,
                        "balance": "1000000000000000000000000"
                    }
                ]
            })),
            true,
        ),
        get_operation_spec::<V1NftResponse>(
            &mut registry,
            "account_nft",
            "FastNEAR API - V1 Account NFT",
            "/v1/account/{account_id}/nft",
            "account_nft_v1",
            "Lookup indexed NFT contract rows for an account",
            "Fetch NFT contract rows for an account, including block-height metadata for each contract.",
            &["non-fungible-tokens"],
            with_api_key(vec![path_parameter(
                "account_id",
                "NEAR account ID to inspect.",
                json!("sharddog.near"),
            )]),
            "Indexed NFT contract rows for the requested account",
            Some(json!({
                "account_id": "sharddog.near",
                "tokens": [
                    {
                        "contract_id": "nft.example.near",
                        "last_update_block_height": null
                    }
                ]
            })),
            true,
        ),
        get_operation_spec::<TokenAccountsResponse>(
            &mut registry,
            "ft_top",
            "FastNEAR API - V1 FT Top Holders",
            "/v1/ft/{token_id}/top",
            "ft_top_v1",
            "Lookup top indexed holders for a fungible token",
            "Use this endpoint to inspect the indexed top holders for a fungible token contract.",
            &["fungible-tokens"],
            with_api_key(vec![path_parameter(
                "token_id",
                "Fungible token contract account ID.",
                json!("wrap.near"),
            )]),
            "Indexed top holders for the requested fungible token",
            Some(json!({
                "token_id": "wrap.near",
                "accounts": [
                    {
                        "account_id": "mob.near",
                        "balance": "979894691374420631019486155"
                    }
                ]
            })),
            true,
        ),
        get_operation_spec::<AccountFullResponse>(
            &mut registry,
            "account_full",
            "FastNEAR API - V1 Account Full",
            "/v1/account/{account_id}/full",
            "account_full_v1",
            "Lookup full indexed account information",
            "Fetch the combined indexed account view, including staking pools, FT balances, NFTs, and account state.",
            &["accounts"],
            with_api_key(vec![path_parameter(
                "account_id",
                "NEAR account ID to inspect.",
                json!("here.tg"),
            )]),
            "Full indexed account information for the requested account",
            Some(json!({
                "account_id": "here.tg",
                "pools": [],
                "tokens": [],
                "nfts": [],
                "state": {
                    "balance": "1000000000000000000000000",
                    "locked": "0",
                    "storage_bytes": 512
                }
            })),
            true,
        ),
    ];

    if include_exp {
        operations.push(get_operation_spec::<ExpFtWithBalancesResponse>(
            &mut registry,
            "ft_with_balances",
            "FastNEAR API - Experimental FT With Balances",
            "/exp/account/{account_id}/ft_with_balances",
            "exp_ft_with_balances",
            "Resolve fungible token balances for one account",
            "Returns a token-to-balance map for the requested account. This endpoint is deploy-gated and not part of the default published FastNEAR portal until `EXPERIMENTAL_API=true` is enabled publicly.",
            &["experimental"],
            with_api_key(vec![path_parameter(
                "account_id",
                "NEAR account ID to inspect.",
                json!("here.tg"),
            )]),
            "Token-to-balance map for the requested account",
            Some(json!({
                "account_id": "here.tg",
                "tokens": {
                    "wrap.near": "1000000000000000000000000",
                    "usdt.tether-token.near": null
                }
            })),
            true,
        ));
        operations.push(get_operation_spec::<TokenAccountsResponse>(
            &mut registry,
            "ft_all",
            "FastNEAR API - Experimental FT Holders",
            "/exp/ft/{token_id}/all",
            "exp_ft_all",
            "Get every indexed holder for a fungible token",
            "Returns every indexed account FastNEAR has stored for the requested fungible token, without the v1 top-holder limit. This endpoint is deploy-gated and not part of the default published FastNEAR portal until `EXPERIMENTAL_API=true` is enabled publicly.",
            &["experimental"],
            with_api_key(vec![path_parameter(
                "token_id",
                "Fungible token contract account ID.",
                json!("first.tkn.near"),
            )]),
            "All indexed holders for the requested fungible token",
            Some(json!({
                "token_id": "first.tkn.near",
                "accounts": [
                    {
                        "account_id": "foundation.near",
                        "balance": "1000000000"
                    }
                ]
            })),
            true,
        ));
    }

    let components = registry.into_components();
    let mut service_doc = build_service_doc(&SERVICE_INFO, operations, components);
    patch_component_requirements(&mut service_doc);
    write_or_check_yaml(output_root.join("openapi.yaml"), &service_doc, check)?;
    Ok(())
}

fn get_operation_spec<Response>(
    registry: &mut SchemaRegistry,
    slug: &'static str,
    title: &'static str,
    path: &'static str,
    operation_id: &'static str,
    summary: &'static str,
    description: &'static str,
    tags: &'static [&'static str],
    parameters: Vec<ParameterSpec<'static>>,
    ok_description: &'static str,
    ok_example: Option<Value>,
    include_bad_request: bool,
) -> AggregateOperationSpec<'static>
where
    Response: JsonSchema,
{
    let response = registry.schema_ref::<Response>();

    let mut responses = vec![ResponseSpec {
        status: "200",
        description: ok_description,
        content: Some(ResponseContent::Json {
            schema: response,
            example: ok_example,
            examples: vec![],
        }),
    }];

    if include_bad_request {
        responses.push(ResponseSpec {
            status: "400",
            description: "Bad Request",
            content: Some(ResponseContent::Json {
                schema: json!({ "type": "string" }),
                example: None,
                examples: vec![],
            }),
        });
    }

    responses.push(ResponseSpec {
        status: "500",
        description: "Internal Server Error",
        content: Some(ResponseContent::Json {
            schema: json!({ "type": "string" }),
            example: None,
            examples: vec![],
        }),
    });

    AggregateOperationSpec {
        slug,
        title,
        path,
        method: HttpMethod::Get,
        operation_id,
        summary,
        description,
        tags,
        parameters,
        request_body: None,
        responses,
    }
}

fn api_key_parameter() -> ParameterSpec<'static> {
    ParameterSpec {
        name: "apiKey",
        location: ParameterLocation::Query,
        required: false,
        description:
            "Optional API key forwarded by embedded portal clients. The public FastNEAR API does not require it.",
        schema: json!({ "type": "string" }),
        example: None,
    }
}

fn path_parameter(
    name: &'static str,
    description: &'static str,
    example: Value,
) -> ParameterSpec<'static> {
    ParameterSpec {
        name,
        location: ParameterLocation::Path,
        required: true,
        description,
        schema: json!({ "type": "string" }),
        example: Some(example),
    }
}

fn with_api_key(mut parameters: Vec<ParameterSpec<'static>>) -> Vec<ParameterSpec<'static>> {
    parameters.push(api_key_parameter());
    parameters
}

fn patch_component_requirements(doc: &mut Value) {
    let schemas = doc["components"]["schemas"].as_object_mut().unwrap();
    set_required(
        schemas,
        "StatusResponse",
        &[
            "version",
            "sync_block_height",
            "sync_latency_sec",
            "sync_block_timestamp_nanosec",
            "sync_balance_block_height",
        ],
    );
    set_required(schemas, "HealthResponse", &["status"]);
    set_required(
        schemas,
        "PublicKeyLookupResponse",
        &["public_key", "account_ids"],
    );
    set_required(schemas, "V0StakingResponse", &["account_id", "pools"]);
    set_required(
        schemas,
        "V0ContractsResponse",
        &["account_id", "contract_ids"],
    );
    set_required(schemas, "PoolRow", &["pool_id", "last_update_block_height"]);
    set_required(
        schemas,
        "TokenRow",
        &["contract_id", "last_update_block_height", "balance"],
    );
    set_required(
        schemas,
        "NftRow",
        &["contract_id", "last_update_block_height"],
    );
    set_required(schemas, "V1StakingResponse", &["account_id", "pools"]);
    set_required(schemas, "V1FtResponse", &["account_id", "tokens"]);
    set_required(schemas, "V1NftResponse", &["account_id", "tokens"]);
    set_required(schemas, "AccountBalanceRow", &["account_id", "balance"]);
    set_required(schemas, "TokenAccountsResponse", &["token_id", "accounts"]);
    set_required(
        schemas,
        "AccountStateResponse",
        &["balance", "locked", "storage_bytes"],
    );
    set_required(
        schemas,
        "AccountFullResponse",
        &["account_id", "pools", "tokens", "nfts", "state"],
    );
    set_nullable_ref_property(
        schemas,
        "AccountFullResponse",
        "state",
        "#/components/schemas/AccountStateResponse",
    );
    set_required(
        schemas,
        "ExpFtWithBalancesResponse",
        &["account_id", "tokens"],
    );
}

fn set_required(
    schemas: &mut serde_json::Map<String, Value>,
    schema_name: &str,
    required: &[&str],
) {
    let Some(schema) = schemas.get_mut(schema_name) else {
        return;
    };

    schema.as_object_mut().unwrap().insert(
        "required".to_string(),
        Value::Array(
            required
                .iter()
                .map(|field| Value::String((*field).to_string()))
                .collect(),
        ),
    );
}

fn set_nullable_ref_property(
    schemas: &mut serde_json::Map<String, Value>,
    schema_name: &str,
    property_name: &str,
    reference: &str,
) {
    let Some(schema) = schemas.get_mut(schema_name) else {
        return;
    };

    let Some(properties) = schema
        .as_object_mut()
        .and_then(|schema| schema.get_mut("properties"))
        .and_then(Value::as_object_mut)
    else {
        return;
    };

    let Some(property) = properties.get_mut(property_name) else {
        return;
    };

    *property = json!({
        "type": "object",
        "allOf": [
            {
                "$ref": reference
            }
        ],
        "nullable": true
    });
}

#[cfg(test)]
mod tests {
    use super::patch_component_requirements;
    use fastnear_openapi_generator::SchemaRegistry;
    use serde_json::json;

    use crate::types::{AccountFullResponse, PublicKeyLookupResponse, StatusResponse, TokenRow};

    #[test]
    fn component_requirement_patch_restores_required_nullable_fields() {
        let mut registry = SchemaRegistry::openapi3();
        registry.schema_ref::<StatusResponse>();
        registry.schema_ref::<PublicKeyLookupResponse>();
        registry.schema_ref::<TokenRow>();
        registry.schema_ref::<AccountFullResponse>();

        let mut doc = json!({
            "components": {
                "schemas": registry.into_components()
            }
        });
        patch_component_requirements(&mut doc);

        assert_eq!(
            doc["components"]["schemas"]["StatusResponse"]["required"],
            json!([
                "version",
                "sync_block_height",
                "sync_latency_sec",
                "sync_block_timestamp_nanosec",
                "sync_balance_block_height"
            ])
        );
        assert_eq!(
            doc["components"]["schemas"]["TokenRow"]["required"],
            json!(["contract_id", "last_update_block_height", "balance"])
        );
        assert_eq!(
            doc["components"]["schemas"]["AccountFullResponse"]["properties"]["state"]["nullable"],
            true
        );
    }
}
