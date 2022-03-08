# TLAND token

This is a implementation of a TLAND token contract. It implements
the [CW20 spec](../../packages/cw20/README.md) and is designed to
be deployed as is, or imported into other contracts to easily build
cw20-compatible tokens with custom logic.

Implements:

- [x] CW20 Base
- [x] Allowances extension

## Running this contract

You will need Rust 1.44.1+ with `wasm32-unknown-unknown` target installed.

You can run unit tests on this via:

`cargo test`

Once you are happy with the content, you can compile it to wasm via:

```
RUSTFLAGS='-C link-arg=-s' cargo wasm
cp ../../target/wasm32-unknown-unknown/release/tland_token.wasm .
ls -l tland_token.wasm
sha256sum tland_token.wasm
```

The optimized contracts are generated in the `artifacts/` directory.

## Create contract

```json
{
  "owner": "terra1dnf8xxhal8rc9vul43a3v3lsu79uym68znyk3q"
  "decimals": 6,
  "name": "HighwayLand token",
  "symbol": "HWLD",
  "initial_balances": [
    {
      "address": "terra1dnf8xxhal8rc9vul43a3v3lsu79uym68znyk3q",
      "amount": "100000000000000"
    }
  ]
}
```

