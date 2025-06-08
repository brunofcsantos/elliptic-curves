use p521::{
    ecdsa::{signature::Signer, Signature, SigningKey},
    SecretKey,
};
use hex::FromHex;

fn main() {
    let hex_key = "01a0a65f74535a0ab65ff0235e9800dbff005b290deb0b24805b23191961101bbfe6bdddb0f8932fab8bf19d7b13493d48915ac4422d290cbc8adadb70433ff72d8d";

    let private_key_bytes: [u8; 66] = <[u8; 66]>::from_hex(hex_key).expect("");

    let secret_key = SecretKey::from_bytes((&private_key_bytes).into()).expect("");

    let signing_key = SigningKey::from(secret_key);

    let message = b"";

    let _signature: Signature = signing_key.sign(message);
}
