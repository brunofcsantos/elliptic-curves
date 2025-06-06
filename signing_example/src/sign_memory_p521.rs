use p521::{
    ecdsa::{signature::Signer, Signature, SigningKey},
    SecretKey,
};
use hex::FromHex;

fn main() {
    let hex_key = "004cd18fb54d41542d43c1fc8957aff5590c9e76d48570d390aca50b9f199442a0ecbc57b2117c9127efbd94eb724035aec08f231ff1e66b370d5d0f620ba61cd8b4";

    let private_key_bytes: [u8; 66] = <[u8; 66]>::from_hex(hex_key).expect("Invalid hex private key");

    let secret_key = SecretKey::from_bytes((&private_key_bytes).into()).expect("Invalid private key bytes");

    let signing_key = SigningKey::from(secret_key);

    let message = b"example message";

    let _signature: Signature = signing_key.sign(message);
}
