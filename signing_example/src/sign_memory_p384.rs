use p384::{
    ecdsa::{signature::Signer, Signature, SigningKey},
    SecretKey,
};
use hex::FromHex;


fn main() {
    let hex_key = "ca8d158af9266d93cfc3d2dca8cf3d40ab303760f8f52da9df6461bfa4cd81129713c1c47ed09868db6f2d0cef750d73";

    let private_key_bytes = <[u8; 48]>::from_hex(hex_key).expect("Invalid hex private key");

    let secret_key = SecretKey::from_bytes((&private_key_bytes).into()).expect("Invalid private key bytes");

    let signing_key = SigningKey::from(secret_key);

    let message = b"example message";
    
    let _signature: Signature = signing_key.sign(message);
}
