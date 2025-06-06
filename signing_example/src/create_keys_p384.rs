use p384::{
    ecdsa::{signature::{Signer, SignatureEncoding}, Signature, SigningKey},
    pkcs8::DecodePrivateKey,
    SecretKey,
};
use std::fs;

fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn main() {
    let pem_path = "/root/Documents/QuantumUC/pqc-bench2/submodules/elliptic-curves/signing_example/test/test_data/p384-private-key.pem";
    let pem_str = fs::read_to_string(pem_path).expect("Failed to read PEM file");

    let secret_key = SecretKey::from_pkcs8_pem(&pem_str).expect("Failed to parse PKCS#8 PEM");

    let private_bytes = secret_key.to_bytes();

    let public_key = secret_key.public_key();
    let public_bytes = public_key.to_sec1_bytes();
    let signing_key = SigningKey::from(secret_key);
    let message = b"example message";

    let signature: Signature = signing_key.sign(message);
    let der_signature_bytes = signature.to_der().to_vec();  

    println!("Private Key (raw bytes): {:02x?}", to_hex(&private_bytes));
    println!("Public Key (SEC1 uncompressed): {:02x?}", to_hex(&public_bytes));
    println!("Signature (DER-encoded): {:02x?}", to_hex(&der_signature_bytes));
}
