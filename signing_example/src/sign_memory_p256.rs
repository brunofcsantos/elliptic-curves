use p256::{
    ecdsa::{signature::Signer, Signature, SigningKey},
    SecretKey,
};
use hex::FromHex;

fn main() {
    let hex_key = "69624171561a63340de0e7d869f2a05492558e1a04868b6a9f854a866788188d";

    let private_key_bytes = <[u8; 32]>::from_hex(hex_key).expect("Invalid hex private key");

    let secret_key = SecretKey::from_bytes((&private_key_bytes).into()).expect("Invalid private key bytes");

    let signing_key = SigningKey::from(secret_key);

    let message = b"example message";

    let _signature: Signature = signing_key.sign(message);
}
