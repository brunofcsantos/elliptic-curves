use p256::{
    ecdsa::{signature::Signer, Signature, SigningKey},
    SecretKey,
};
use hex::FromHex;

fn main() {
    let hex_key = "82cfce5531be6202cd265fcdcc677dd940482c870cb788de725bfdbb4eeb8b44";

    let private_key_bytes = <[u8; 32]>::from_hex(hex_key).expect("");

    let secret_key = SecretKey::from_bytes((&private_key_bytes).into()).expect("");

    let signing_key = SigningKey::from(secret_key);

    let message = b"";

    let _signature: Signature = signing_key.sign(message);
}
