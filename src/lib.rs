use crate::types::PublicKey;
use crate::util::{NearPublicKeyExt, ScalarExt};
use hkdf::Hkdf;
use k256::elliptic_curve::group::GroupEncoding;
use k256::elliptic_curve::CurveArithmetic;
use k256::{Scalar, Secp256k1};
use near_primitives::hash::CryptoHash;
use near_primitives::types::AccountId;
use sha2::{Digest, Sha256};
use hex::encode;
use sha3::Keccak256;
use ethereum_types::H160;

pub mod types;
pub mod util;

// Constant prefix that ensures epsilon derivation values are used specifically for
// near-mpc-recovery with key derivation protocol vX.Y.Z.
const EPSILON_DERIVATION_PREFIX: &str = "near-mpc-recovery v0.1.0 epsilon derivation:";
// Constant prefix that ensures delta derivation values are used specifically for
// near-mpc-recovery with key derivation protocol vX.Y.Z.
const DELTA_DERIVATION_PREFIX: &str = "near-mpc-recovery v0.1.0 delta derivation:";

pub fn derive_epsilon(signer_id: &AccountId, path: &str) -> Scalar {
    // TODO: Use a key derivation library instead of doing this manually.
    // https://crates.io/crates/hkdf might be a good option?
    //
    // ',' is ACCOUNT_DATA_SEPARATOR from nearcore that indicate the end
    // of the accound id in the trie key. We reuse the same constant to
    // indicate the end of the account id in derivation path.
    let derivation_path = format!("{EPSILON_DERIVATION_PREFIX}{},{}", signer_id, path);
    let mut hasher = Sha256::new();
    hasher.update(derivation_path);
    Scalar::from_bytes(&hasher.finalize())
}

// In case there are multiple requests in the same block (hence same entropy), we need to ensure
// that we generate different random scalars as delta tweaks.
// Receipt ID should be unique inside of a block, so it serves us as the request identifier.
pub fn derive_delta(receipt_id: CryptoHash, entropy: [u8; 32]) -> Scalar {
    let hk = Hkdf::<Sha256>::new(None, &entropy);
    let info = format!("{DELTA_DERIVATION_PREFIX}:{}", receipt_id);
    let mut okm = [0u8; 32];
    hk.expand(info.as_bytes(), &mut okm).unwrap();
    Scalar::from_bytes(&okm)
}

pub fn derive_key(public_key: PublicKey, epsilon: Scalar) -> PublicKey {
    (<Secp256k1 as CurveArithmetic>::ProjectivePoint::GENERATOR * epsilon + public_key).to_affine()
}

pub fn main() {
    // ethereum mainnet: 1, sepolia testnet: 11155111
    let chain_id: u32 = 1;
    // your account
	let account_id = "md1.testnet".to_string().try_into().unwrap();
    // path to ethereum accounts and numbered offset of specific account to create
	let path = ",ethereum,1";
    // mpc_public_key from: NEAR_ENV=testnet near view multichain-testnet-2.testnet public_key
    let mpc_public_key: near_sdk::PublicKey = "secp256k1:5Kwe7Ho7gicqBeTUGQLjKeRo87A3xyXjw1MbJVFe6GSiGzL4rK6i6Ycx8ksXJsFBPuxHv97U481HbM96KRYvbkX6".parse().unwrap();
	let epsilon = derive_epsilon(&account_id, path);
	let pk_point = derive_key(mpc_public_key.into_affine_point(), epsilon).to_bytes();
    // for the sig v value used in ecdsa recovery of public key from signatures
    let parity = match pk_point.get(0).unwrap() {
        0x02 => 0,
        0x03 => 1,
        _ => 0
    };
    let address = H160::from_slice(&Keccak256::digest(&pk_point).as_slice()[12..]);

    // output
    println!("Ethereum Address: {:?}", address);
    // see: https://eips.ethereum.org/EIPS/eip-155
    println!("Ethereum v sig value {}", encode((parity + chain_id * 2 + 35).to_be_bytes()));
}