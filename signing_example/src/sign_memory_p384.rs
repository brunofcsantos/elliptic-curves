use p384::{
    ecdsa::{signature::Signer, Signature, SigningKey},
    SecretKey,
};
use hex::FromHex;


fn main() {
    let hex_key = "ce66cdba28eb4d65e010a403667861f1ddbb0a0686b8e06e951b4d218daed77f2501215fdafc701b1825f216f0d6d253";

    let private_key_bytes = <[u8; 48]>::from_hex(hex_key).expect("");

    let secret_key = SecretKey::from_bytes((&private_key_bytes).into()).expect("");

    let signing_key = SigningKey::from(secret_key);

    let message = b"";
    
    let _signature: Signature = signing_key.sign(message);
}
