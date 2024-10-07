# FASTNEAR API

The low-latency API for wallets and explorers.

## Overview

APIs:

1. Public Key to Account ID(s) mapping.

- Full Access Public Key to Account ID(s) mapping.
- Any Public Key to Account ID(s) mapping.

2. Account ID to delegated staking pools (validators).
3. Account ID to fungible tokens (FT contracts).
4. Account ID to non-fungible tokens (NFT contracts).
5. Token ID to top 100 accounts by balance (for FT contracts).
6. Account ID to full info (validators, FT, NFT and account state).

Endpoints:

- Mainnet: https://api.fastnear.com
- Testnet: https://test.api.fastnear.com

## Status

You can check status of the API server.

```
GET /status
```

https://api.fastnear.com/status

```bash
curl https://api.fastnear.com/status
```

Example Result:

```json
{
  "sync_balance_block_height": 129734103,
  "sync_block_height": 129734103,
  "sync_block_timestamp_nanosec": "1728256282197171397",
  "sync_latency_sec": 4.671730603,
  "version": "0.10.0"
}
```

## Health

Returns the health status of the API server.

```
GET /health
```

https://api.fastnear.com/health

```bash
curl https://api.fastnear.com/health
```

Example Result (for healthy):

```json
{
  "status": "ok"
}
```

## API V1

In API V1, the API endpoints provide extra details about the contracts.
E.g. the block height when the last change was made on a contract that affected a given account, or a token balance.

#### Token ID to top 100 accounts by balance (for FT contracts).

Returns the list of account IDs for a given fungible tokens (FT) contract ordered by decreasing FT balance.
Each account result includes the following:

- `account_id` - the account ID.
- `balance` - the last known balance of the account for this token.

Notes:

- the `balance` will be returned as a decimal integer string, e.g. `"100"`.

```
GET /v1/ft/{token_id}/top
```

Example: https://api.fastnear.com/v1/ft/first.tkn.near/top

```bash
curl https://api.fastnear.com/v1/ft/first.tkn.near/top
```

Result:

```json
{
  "token_id": "first.tkn.near",
  "accounts": [
    {
      "account_id": "mob.near",
      "balance": "979894691374420631019486155"
    },
    {
      "account_id": "lucky-bastard.near",
      "balance": "10319841074196024761995069"
    },
    {
      "account_id": "mattlock.near",
      "balance": "9775084808910328058513245"
    },
    {
      "account_id": "ref-finance.near",
      "balance": "10290906529190035816723"
    },
    {
      "account_id": "zilulagg.near",
      "balance": "91835943826124178808"
    },
    {
      "account_id": "kotleta.near",
      "balance": "10000"
    },
    {
      "account_id": "ryanmehta.near",
      "balance": "0"
    }
  ]
}
```

#### Account ID to delegated staking pools (validators).

Returns the list of staking pools that the account has delegated to in the past, including the block
height when the last change was made on the staking pool by the account.

Note, if the `last_update_block_height` is `null`, then no recent updates were made.

```
GET /v1/account/{account_id}/staking
```

Example: https://api.fastnear.com/v1/account/mob.near/staking

```bash
curl https://api.fastnear.com/v1/account/mob.near/staking
```

Result:

```json
{
  "account_id": "mob.near",
  "pools": [
    {
      "last_update_block_height": 114976469,
      "pool_id": "zavodil.poolv1.near"
    },
    {
      "last_update_block_height": null,
      "pool_id": "usn.pool.near"
    },
    {
      "last_update_block_height": null,
      "pool_id": "usn-unofficial.pool.near"
    },
    {
      "last_update_block_height": null,
      "pool_id": "epic.poolv1.near"
    },
    {
      "last_update_block_height": 114976560,
      "pool_id": "here.poolv1.near"
    }
  ]
}
```

#### Account ID to fungible tokens (FT contracts).

Returns the list of fungible tokens (FT) contracts that the account may have.
Each token result includes the following:

- `contract_id` - the account ID of the fungible token contract.
- `last_update_block_height` - the block height when the last change was made on the contract that affected this given
  account.
- `balance` - the last known balance of the account for this token.

Notes:

- if the `last_update_block_height` is `null`, then no recent updates were made. The last update block height change was
  enabled around block `115000000`.
- the `balance` will be returned as a decimal integer string, e.g. `"100"`.
- the `balance` is not adjusted to the decimals in the FT metadata, it's the raw balance as stored in the contract.
- if the `balance` is `null`, then the balance is not available yet. It's likely will be updated soon (within a few
  seconds).
- if the `balance` is empty string (`""`), then the account fungible token contract might be broken, because it didn't
  return the proper balance.

```
GET /v1/account/{account_id}/ft
```

Example: https://api.fastnear.com/v1/account/here.tg/ft

```bash
curl https://api.fastnear.com/v1/account/here.tg/ft
```

Result:

```json
{
  "account_id": "here.tg",
  "tokens": [
    {
      "balance": "10000",
      "contract_id": "game.hot.tg",
      "last_update_block_height": 115615375
    },
    {
      "balance": "81000",
      "contract_id": "usdt.tether-token.near",
      "last_update_block_height": null
    }
  ]
}
```

#### Account ID to non-fungible tokens (NFT contracts).

Returns the list of non-fungible tokens (NFT) contracts that the account has interacted with or received, including the
block height when the last change was made on the contract that affected this given account.

Note, if the `last_update_block_height` is `null`, then no recent updates were made.

```
GET /v1/account/{account_id}/nft
```

Example: https://api.fastnear.com/v1/account/sharddog.near/nft

```bash
curl https://api.fastnear.com/v1/account/sharddog.near/nft
```

Result:

```json
{
  "account_id": "sharddog.near",
  "tokens": [
    {
      "contract_id": "mint.sharddog.near",
      "last_update_block_height": 115034954
    },
    {
      "contract_id": "open.sharddog.near",
      "last_update_block_height": null
    },
    {
      "contract_id": "humansofbrazil.sharddog.near",
      "last_update_block_height": null
    },
    {
      "contract_id": "nft.bluntdao.near",
      "last_update_block_height": null
    },
    {
      "contract_id": "ndcconstellationnft.sharddog.near",
      "last_update_block_height": null
    },
    {
      "contract_id": "mmc.sharddog.near",
      "last_update_block_height": null
    },
    {
      "contract_id": "nft.genadrop.near",
      "last_update_block_height": null
    },
    {
      "contract_id": "harvestmoon.sharddog.near",
      "last_update_block_height": null
    },
    {
      "contract_id": "comic.sharddog.near",
      "last_update_block_height": 114988538
    },
    {
      "contract_id": "meteor.sharddog.near",
      "last_update_block_height": null
    },
    {
      "contract_id": "nstreetwolves.near",
      "last_update_block_height": null
    },
    {
      "contract_id": "starpause.mintbase1.near",
      "last_update_block_height": null
    },
    {
      "contract_id": "rubenm4rcusstore.mintbase1.near",
      "last_update_block_height": null
    },
    {
      "contract_id": "rogues-genesis.nfts.fewandfar.near",
      "last_update_block_height": null
    },
    {
      "contract_id": "nft.regens.near",
      "last_update_block_height": null
    },
    {
      "contract_id": "mmc-mint.sharddog.near",
      "last_update_block_height": null
    },
    {
      "contract_id": "badges.devhub.near",
      "last_update_block_height": null
    },
    {
      "contract_id": "secretnft.devhub.near",
      "last_update_block_height": null
    },
    {
      "contract_id": "giveaway.mydev.near",
      "last_update_block_height": null
    },
    {
      "contract_id": "mintv2.sharddog.near",
      "last_update_block_height": 114973604
    },
    {
      "contract_id": "nearvidia.sharddog.near",
      "last_update_block_height": null
    },
    {
      "contract_id": "claim.sharddog.near",
      "last_update_block_height": 115039779
    }
  ]
}
```

#### Account ID to full info (validators, FT, NFT and account state)

Returns the full information about the account, including the following:

- Delegated staking pools (validators).
- Fungible tokens (FT) contracts and balances.
- Non-fungible tokens (NFT) contracts.
- Account state (balance, locked balance, storage usage).

```
GET /v1/account/{account_id}/full
```

Example: https://api.fastnear.com/v1/account/here.tg/full

```bash
curl https://api.fastnear.com/v1/account/here.tg/full
```

Result:

```json
{
  "account_id": "here.tg",
  "nfts": [
    {
      "contract_id": "harvestmoon.sharddog.near",
      "last_update_block_height": null
    },
    {
      "contract_id": "nft.hot.tg",
      "last_update_block_height": 115282010
    },
    {
      "contract_id": "nearvoucherstore.mintbase1.near",
      "last_update_block_height": 118841842
    },
    {
      "contract_id": "nearreward.mintbase1.near",
      "last_update_block_height": 121969370
    }
  ],
  "pools": [
    {
      "last_update_block_height": null,
      "pool_id": "here.poolv1.near"
    }
  ],
  "state": {
    "balance": "240420562203528059226991880",
    "locked": "0",
    "storage_bytes": 26340
  },
  "tokens": [
    {
      "balance": "9990",
      "contract_id": "game.hot.tg",
      "last_update_block_height": 123971814
    },
    {
      "balance": "10283000",
      "contract_id": "usdt.tether-token.near",
      "last_update_block_height": 116301157
    },
    {
      "balance": "0",
      "contract_id": "aurora",
      "last_update_block_height": 118627759
    },
    {
      "balance": "2318000000000000",
      "contract_id": "c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2.factory.bridge.near",
      "last_update_block_height": 118667336
    },
    {
      "balance": "10000000000000000000",
      "contract_id": "nearrewards.near",
      "last_update_block_height": 118842567
    },
    {
      "balance": "8999999999899999",
      "contract_id": "wbnb.hot.tg",
      "last_update_block_height": 121310030
    },
    {
      "balance": "",
      "contract_id": "v1.omni.hot.tg",
      "last_update_block_height": 128025061
    }
  ]
}
```

## API V0

#### Full Access Public Key to Account ID mapping.

Returns the list of account IDs that are associated with the full-access public key.

Note, the API will also return an implicit account ID for the public key, even if the implicit account might not exist.

```
GET /v0/public_key/{public_key}
```

Example: https://api.fastnear.com/v0/public_key/ed25519:FekbqN74kXhVPRd8ysAqJwLydFvTPYh7ZXHmhqCETcR3

```bash
curl https://api.fastnear.com/v0/public_key/ed25519:FekbqN74kXhVPRd8ysAqJwLydFvTPYh7ZXHmhqCETcR3
```

Result:

```json
{
  "account_ids": [
    "root.near",
    "d9af67ff794a93e05bdba5c25ad7af027d72b3b76823051c0fb4b6e3e79ac51e"
  ],
  "public_key": "ed25519:FekbqN74kXhVPRd8ysAqJwLydFvTPYh7ZXHmhqCETcR3"
}
```

#### Any Public Key to Account ID mapping.

Returns the list of account IDs that are associated with this public key, including limited access keys.

Note, the API will also return an implicit account ID for the public key, even if the implicit account might not exist.

```
GET /v0/public_key/{public_key}/all
```

Example: https://api.fastnear.com/v0/public_key/ed25519:HLcgpHWRn3ij97JfpPNYDScMXVguWSFH1mR58RB7qPpd/all

```bash
curl https://api.fastnear.com/v0/public_key/ed25519:HLcgpHWRn3ij97JfpPNYDScMXVguWSFH1mR58RB7qPpd/all
```

Result:

```json
{
  "account_ids": [
    "root.near",
    "f2c160840040d637041a5dc63eeb23b8aae41a79fc9b0f2d8df07adb613d1d82"
  ],
  "public_key": "ed25519:HLcgpHWRn3ij97JfpPNYDScMXVguWSFH1mR58RB7qPpd"
}
```

#### Account ID to delegated staking pools (validators).

Returns the list of staking pools that the account has delegated to in the past.

*Deprecated in favor of API V1.*

```
GET /v0/account/{account_id}/staking
```

Example: https://api.fastnear.com/v0/account/root.near/staking

```bash
curl https://api.fastnear.com/v0/account/root.near/staking
```

Result:

```json
{
  "account_id": "root.near",
  "pools": [
    "ashert.poolv1.near"
  ]
}
```

#### Account ID to fungible tokens (FT contracts).

Returns the list of fungible tokens (FT) contracts that the account has interacted with or received.

*Deprecated in favor of API V1.*

```
GET /v0/account/{account_id}/ft
```

Example: https://api.fastnear.com/v0/account/root.near/ft

```bash
curl https://api.fastnear.com/v0/account/root.near/ft
```

Result:

```json
{
  "account_id": "root.near",
  "contract_ids": [
    "pixeltoken.near",
    "ndc.tkn.near",
    "meta-pool.near",
    "coin.asac.near",
    "cheems.tkn.near",
    "baby.tkn.near",
    "meteor-points.near",
    "9aeb50f542050172359a0e1a25a9933bc8c01259.factory.bridge.near",
    "meta-token.near",
    "c.tkn.near",
    "bobo.tkn.near",
    "gold.l2e.near",
    "usn",
    "token.lonkingnearbackto2024.near",
    "utopia.secretskelliessociety.near",
    "wnear-at-150-0.wentokensir.near",
    "v3.oin_finance.near",
    "adtoken.near",
    "nearbit.tkn.near",
    "mvp.tkn.near",
    "youwon500neartoclaimyourgainwwwlotte.laboratory.jumpfinance.near",
    "wnear-150-0000.wentokensir.near",
    "fx.tkn.near",
    "1.laboratory.jumpfinance.near",
    "zod.near",
    "a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48.factory.bridge.near",
    "kusama-airdrop.near",
    "blackdragon.tkn.near",
    "congratulations.laboratory.jumpfinance.near",
    "wrap.near",
    "avb.tkn.near",
    "ftv2.nekotoken.near",
    "superbot.near",
    "fusotao-token.near",
    "deezz.near",
    "ser.tkn.near",
    "near-20-0000.wentokensir.near",
    "aurora.tkn.near",
    "f5cfbc74057c610c8ef151a439252680ac68c6dc.factory.bridge.near",
    "nearkat.tkn.near",
    "youwon500neartoclaimyourgainwwwnearl.laboratory.jumpfinance.near"
  ]
}
```

#### Account ID to non-fungible tokens (NFT contracts).

Returns the list of non-fungible tokens (NFT) contracts that the account has interacted with or received.

*Deprecated in favor of API V1.*

```
GET /v0/account/{account_id}/nft
```

Example: https://api.fastnear.com/v0/account/root.near/nft

```bash
curl https://api.fastnear.com/v0/account/root.near/nft
```

Result:

```json
{
  "account_id": "root.near",
  "contract_ids": [
    "nft.goodfortunefelines.near",
    "genadrop-contract.nftgen.near",
    "paulcrans.mintbase1.near",
    "mailgun.near",
    "citizen.bodega-lab.near",
    "avtr.near",
    "comic.paras.near",
    "spin-nft-contract.near",
    "ambernft.near",
    "ndcconstellationnft.sharddog.near",
    "learnernft.learnclub.near",
    "freedom.mintbase1.near",
    "nshackathon2022.mintbase1.near",
    "near-punks.near",
    "mint.sharddog.near",
    "nft.genadrop.near",
    "nearnautsnft.near",
    "roughcentury.mintbase1.near",
    "chatgpt.mintbase1.near",
    "tonic_goblin.enleap.near",
    "nft.greedygoblins.near",
    "nep172.nfnft.near",
    "nearcrashnft.near",
    "yearoftherabbit.near",
    "nft-message.nearkits.near",
    "famdom1.nearhubonline.near",
    "nft.widget.near",
    "nearmailbot.near",
    "harvestmoon.sharddog.near",
    "pcards.near",
    "kaizofighters.tenk.near",
    "hot-or-bot.near",
    "nearnauts.mintbase1.near",
    "dotdot.mintbase1.near",
    "qstienft.near",
    "yuzu.recurforever.near",
    "serumnft.near",
    "pack.pack_minter.playible.near",
    "nearcon.mintbase1.near",
    "seoul2020.snft.near",
    "astropup.near",
    "nearnautnft.near",
    "kashmirthroughmylens.mintbase1.near",
    "nearmixtapev1beatdao.mintbase1.near",
    "rtrpkp.mintbase1.near",
    "ouchworld.mintbase1.near",
    "beenftofficial.near",
    "pluminite.near",
    "tigercheck4.near",
    "misfits.tenk.near",
    "nearcon2.mintbase1.near",
    "jwneartokens.mintbase1.near",
    "mmc.nfts.fewandfar.near",
    "nft-v2.keypom.near",
    "proof-of-memories-nearcon-2022.snft.near",
    "cartelgen1.neartopia.near",
    "reginamintbase.mintbase1.near",
    "nearpay-portals.near",
    "near-x-sailgp-f50-fan-token.snft.near",
    "starbox.herewallet.near",
    "mmc-pups.nfts.fewandfar.near",
    "root.mintbase1.near",
    "undead.secretskelliessociety.near",
    "asac.near",
    "x.paras.near",
    "athlete.nfl.playible.near"
  ]
}
```
