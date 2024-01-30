# Generate mpc private key for near protocol

## Instructions

1. How to get the MPC Public Key: `NEAR_ENV=[testnet/mainnet] near view multichain-testnet-2.testnet public_key`
2. Example response: `secp256k1:5Kwe7Ho7gicqBeTUGQLjKeRo87A3xyXjw1MbJVFe6GSiGzL4rK6i6Ycx8ksXJsFBPuxHv97U481HbM96KRYvbkX6`
3. Put into `lib.rs`
4. Add your NEAR AccountId
5. Add your path based on naming conventions: https://github.com/near/near-fastauth-wallet/blob/dmd/chain_sig_docs/docs/chain_signature_api.org
6. `cargo run`

## WIP
- Right now the focus is on Ethereum private keys and accounts (see output)

## TODO
[] Add cli args
[] Generalize to other chains