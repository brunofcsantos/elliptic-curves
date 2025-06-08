use p256::{
    ecdsa::{signature::Verifier, Signature},
};
use hex::FromHex;

fn main() {
    let public_key_hex = "046067afbece1e859e115908f50734af538372fd5b22668cfbc685f1f6c38d0621c6669f2abe510e3905135ea2a83c9a4fb0b7077d799bd3c6d252a24e84a236ef";

    let signature_hex = "3045022019f98fe00ba45391a893285bec1bffb698169a0cea5ee4718ae2e6745ccfec5c022100ccacd813e2bd216343c0638c58cdc9cb9b280492ba39b9050a9afd19ca9d0fa8";

    let message = b"";

    let public_key_vec = Vec::from_hex(public_key_hex).expect("");

    let verifying_key = p256::ecdsa::VerifyingKey::from_sec1_bytes(&public_key_vec)
        .expect("");

    let signature_bytes = Vec::from_hex(signature_hex).expect("");
    let signature = Signature::from_der(&signature_bytes).expect("");

    verifying_key.verify(message, &signature).expect("");

    println!("Signature verified successfully");
}
