# FASTNEAR API

The low-latency API for wallets and explorers on NEAR Protocol.

Base URLs:
- Mainnet: https://api.fastnear.com
- Testnet: https://test.api.fastnear.com

## Endpoints

### Status & Health

- `GET /status` — Returns sync status (block height, timestamp, latency, version).
- `GET /health` — Returns `{"status": "ok"}` when healthy.

### API V1 (recommended)

- `GET /v1/public_key/{public_key}` — Full-access public key to account ID(s). Also returns the implicit account ID.
- `GET /v1/public_key/{public_key}/all` — Any public key (including limited access) to account ID(s).
- `GET /v1/account/{account_id}/staking` — Delegated staking pools with `last_update_block_height`.
- `GET /v1/account/{account_id}/ft` — Fungible tokens with `last_update_block_height` and `balance`.
- `GET /v1/account/{account_id}/nft` — Non-fungible tokens with `last_update_block_height`.
- `GET /v1/account/{account_id}/full` — Full account info: staking pools, FTs, NFTs, and account state (balance, locked, storage).
- `GET /v1/ft/{token_id}/top` — Top 100 accounts by balance for a given FT contract.

### API V0 (deprecated, use V1)

- `GET /v0/public_key/{public_key}` — Full-access public key to account ID(s).
- `GET /v0/public_key/{public_key}/all` — Any public key to account ID(s).
- `GET /v0/account/{account_id}/staking` — Delegated staking pools (no block height).
- `GET /v0/account/{account_id}/ft` — Fungible token contract IDs (no balances).
- `GET /v0/account/{account_id}/nft` — NFT contract IDs (no block height).

## Notes

- `balance` is a decimal integer string (not adjusted for token decimals).
- `balance: null` means balance is not yet available; `balance: ""` means the FT contract may be broken.
- `last_update_block_height: null` means no recent updates were recorded (tracking started around block 115000000).
- Public key endpoints also return the implicit account ID, even if it doesn't exist on-chain.
