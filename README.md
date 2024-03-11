# FASTNEAR API

The low-latency API for wallets and explorers.

## Overview

There are 4 APIs provided to replace deprecated `api.kitwallet.app` endpoint:

1. Public Key to Account ID mapping.
   - Full Access Public Key to Account ID mapping.
   - Any Public Key to Account ID mapping.
2. Account ID to delegated staking pools (validators).
3. Account ID to fungible tokens (FT contracts).
4. Account ID to non-fungible tokens (NFT contracts).

#### Full Access Public Key to Account ID mapping.

*Expected latency ~`270ms`*

Returns the list of account IDs that are associated with the full-access public key.
```
GET /v0/public_key/{public_key}
```

Example: https://api.fastnear.com/v0/public_key/ed25519:FekbqN74kXhVPRd8ysAqJwLydFvTPYh7ZXHmhqCETcR3
```bash
curl https://api.fastnear.com/v0/public_key/ed25519:FekbqN74kXhVPRd8ysAqJwLydFvTPYh7ZXHmhqCETcR3
```

Result:
```json
{"account_ids":["root.near"],"public_key":"ed25519:FekbqN74kXhVPRd8ysAqJwLydFvTPYh7ZXHmhqCETcR3"}
```

#### Any Public Key to Account ID mapping.
*Expected latency ~`270ms`*

Returns the list of account IDs that are associated with this public key, including limited access keys.
```
GET /v0/public_key/{public_key}/all
```

Example: https://api.fastnear.com/v0/public_key/ed25519:HLcgpHWRn3ij97JfpPNYDScMXVguWSFH1mR58RB7qPpd/all
```bash
curl https://api.fastnear.com/v0/public_key/ed25519:HLcgpHWRn3ij97JfpPNYDScMXVguWSFH1mR58RB7qPpd/all
```

Result:
```json
{"account_ids":["root.near"],"public_key":"ed25519:HLcgpHWRn3ij97JfpPNYDScMXVguWSFH1mR58RB7qPpd"}⏎
```


#### Account ID to delegated staking pools (validators).

*Expected latency ~`26ms`*

Returns the list of staking pools that the account has delegated to in the past.
```
GET /v0/account/{account_id}/staking
```

Example: https://api.fastnear.com/v0/account/root.near/staking
```bash
curl https://api.fastnear.com/v0/account/root.near/staking
```

Result:
```json
{"account_id":"root.near","pools":["ashert.poolv1.near"]}
```

#### Account ID to fungible tokens (FT contracts).

*Expected latency ~`26ms`*

Returns the list of fungible tokens (FT) contracts that the account has interacted with or received.
```
GET /v0/account/{account_id}/ft
```

Example: https://api.fastnear.com/v0/account/root.near/ft
```bash
curl https://api.fastnear.com/v0/account/root.near/ft
```

Result:
```json
{"account_id":"root.near","contract_ids":["pixeltoken.near","ndc.tkn.near","meta-pool.near","coin.asac.near","cheems.tkn.near","baby.tkn.near","meteor-points.near","9aeb50f542050172359a0e1a25a9933bc8c01259.factory.bridge.near","meta-token.near","c.tkn.near","bobo.tkn.near","gold.l2e.near","usn","token.lonkingnearbackto2024.near","utopia.secretskelliessociety.near","wnear-at-150-0.wentokensir.near","v3.oin_finance.near","adtoken.near","nearbit.tkn.near","mvp.tkn.near","youwon500neartoclaimyourgainwwwlotte.laboratory.jumpfinance.near","wnear-150-0000.wentokensir.near","fx.tkn.near","1.laboratory.jumpfinance.near","zod.near","a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48.factory.bridge.near","kusama-airdrop.near","blackdragon.tkn.near","congratulations.laboratory.jumpfinance.near","wrap.near","avb.tkn.near","ftv2.nekotoken.near","superbot.near","fusotao-token.near","deezz.near","ser.tkn.near","near-20-0000.wentokensir.near","aurora.tkn.near","f5cfbc74057c610c8ef151a439252680ac68c6dc.factory.bridge.near","nearkat.tkn.near","youwon500neartoclaimyourgainwwwnearl.laboratory.jumpfinance.near"]}
```

### Account ID to non-fungible tokens (NFT contracts).

*Expected latency ~`26ms`*

Returns the list of non-fungible tokens (NFT) contracts that the account has interacted with or received.
```
GET /v0/account/{account_id}/nft
```

Example: https://api.fastnear.com/v0/account/root.near/nft
```bash
curl https://api.fastnear.com/v0/account/root.near/nft
```

Result:
```json
{"account_id":"root.near","contract_ids":["nft.goodfortunefelines.near","genadrop-contract.nftgen.near","paulcrans.mintbase1.near","mailgun.near","citizen.bodega-lab.near","avtr.near","comic.paras.near","spin-nft-contract.near","ambernft.near","ndcconstellationnft.sharddog.near","learnernft.learnclub.near","freedom.mintbase1.near","nshackathon2022.mintbase1.near","near-punks.near","mint.sharddog.near","nft.genadrop.near","nearnautsnft.near","roughcentury.mintbase1.near","chatgpt.mintbase1.near","tonic_goblin.enleap.near","nft.greedygoblins.near","nep172.nfnft.near","nearcrashnft.near","yearoftherabbit.near","nft-message.nearkits.near","famdom1.nearhubonline.near","nft.widget.near","nearmailbot.near","harvestmoon.sharddog.near","pcards.near","kaizofighters.tenk.near","hot-or-bot.near","nearnauts.mintbase1.near","dotdot.mintbase1.near","qstienft.near","yuzu.recurforever.near","serumnft.near","pack.pack_minter.playible.near","nearcon.mintbase1.near","seoul2020.snft.near","astropup.near","nearnautnft.near","kashmirthroughmylens.mintbase1.near","nearmixtapev1beatdao.mintbase1.near","rtrpkp.mintbase1.near","ouchworld.mintbase1.near","beenftofficial.near","pluminite.near","tigercheck4.near","misfits.tenk.near","nearcon2.mintbase1.near","jwneartokens.mintbase1.near","mmc.nfts.fewandfar.near","nft-v2.keypom.near","proof-of-memories-nearcon-2022.snft.near","cartelgen1.neartopia.near","reginamintbase.mintbase1.near","nearpay-portals.near","near-x-sailgp-f50-fan-token.snft.near","starbox.herewallet.near","mmc-pups.nfts.fewandfar.near","root.mintbase1.near","undead.secretskelliessociety.near","asac.near","x.paras.near","athlete.nfl.playible.near"]}
```
