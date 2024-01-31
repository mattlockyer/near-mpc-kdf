# Generate mpc private key for near protocol

## Concept Overview

- Using Rust project (or some JS/TS in the future) a client generates an Ethereum private key, address and sig `v` value (for ECDSA recovery)
- The client builds an Ethereum TX client side (Chain ID matters)
- The client calls `sign` with their near account
- An indexer picks up the `response` transaction
- In the response is the `big_r` and `s` values for the Ethereum TX
- The client can build a signed and RLP encoded Ethereum TX, and broadcast it to an Ethereum network

## Instructions
*Steps 1 and 2 are already done for the current testnet contract, but this contract and your NEAR network may differ*

1. How to get the MPC Public Key: `NEAR_ENV=[testnet/mainnet] near view [MPC_CONTRACT] public_key`
(for testnet: `NEAR_ENV=testnet near view multichain-testnet-2.testnet public_key`)
2. Example response: `secp256k1:5Kwe7Ho7gicqBeTUGQLjKeRo87A3xyXjw1MbJVFe6GSiGzL4rK6i6Ycx8ksXJsFBPuxHv97U481HbM96KRYvbkX6`
3. Put into `lib.rs`
4. Choose your Chain ID: https://chainlist.org/
5. Add your NEAR AccountId
6. Add your path based on naming conventions: https://github.com/near/near-fastauth-wallet/blob/dmd/chain_sig_docs/docs/chain_signature_api.org
7. `cargo run`
8. Note all values including `v sig value` you will need it later

## Preparing an Ethereum transaction

Reference: https://cypherpunks-core.github.io/ethereumbook/06transactions.html (section: `Transaction Signing in Practice`)

To sign a transaction in Ethereum, the originator must:
1. Create a transaction data structure, containing nine fields: nonce, gasPrice, gasLimit, to, value, data, chainID, 0, 0.

Example:
```
[
	nonce,
	gasPrice,
	gasLimit,
	to,
	value,
	data,
	chainId,
	0,
	0
]
```

2. Produce an RLP-encoded serialized message of the transaction data structure.

Example: ethers.utils.RLP.encode([payload from step 1])

3. Compute the Keccak-256 hash of this serialized message.

Example: 

4. CALL NEAR MPC CONTRACT METHOD `sign`: Compute the ECDSA signature, signing the hash with the originating EOA’s private key.

```
NEAR_ENV=testnet near call multichain-testnet-2.testnet sign '{"path":",ethereum,1","payload":[BYTES OF: Keccak-256 hash of this serialized message]} --accountId=[YOUR_ACCOUNT_ID]'
```

5. GET MPC CONTRACT RESPONSE

Example: 
```
{
  "big_r": "029CA55598B0F280EE5D9023B74B62FFCC68D9CDCF0D33636715F0AE72CA7CE213",
  "receipt_id": [...some bytes...],
  "s": "3392F41127631F364AF9068C10F8F4B524489C9A0506943FEBC4FB867AECAE5E"
}
```

6. Create a new RLP encoded transaction like in step 1 & 2 (ethers.utils.RLP.encode([...])) but instead switch the last 3 values with:
- `v (from rust project output)`
ECDSA signature parts from MPC Contract Response
- `big_r`
- `s`

Some notes about final transaction, every piece is prefixed with a length in RLP encoding: https://ethereum.stackexchange.com/a/38726

7. Broadcast this tx to your ethereum network (same chain_id from rust project!)

Example: `ethers.provider.sendTransaction(YOUR_FULL_RLP_ENCODED_TX_IN_HEX)` (should have prefix 0x)

## WIP
- Right now the focus is on Ethereum private keys and accounts (see output)

## TODO
[] Add cli args
[] Generalize to other chains