use p521::{
    ecdsa::{signature::Verifier, Signature},
};
use hex::FromHex;

fn main() {
    let public_key_hex = "0400717930698b0ea352bd66d70d24bebf8150b17e353bc0b96be804c850bd1af43b31f04ff734feb3fe22557a11777bc6ff7cf677711a1f8be3a91dffe8ddc2f278dc019a58c86f6e30ad3b458f296de52a2800a2b461ec36773924d36f5abe83f12e075499d6fce7ad8a3160a4f8063cb96e354a90977c16ff4718fb5072de7a2bea2e80";

    let signature_hex = "3081870242019cff6e204655739ffd693ba4c59bb864b94f8ee87dd0be069b4c0621005e3fbb6f0ad6ecf3ae4407796642964f7fc309f41394c0723b7d3ee3dea41857e48c96b2024124a67741cb28bcc539d7a75a74b681376860d4701145d140f540f5504d65076c8d2c5cacaab477631bf2813c2017e83bdd6bdb152275e5ac00636da7a883946eca";

    let message = b"";

    let public_key_vec = Vec::from_hex(public_key_hex).expect("");

    let verifying_key = p521::ecdsa::VerifyingKey::from_sec1_bytes(&public_key_vec)
        .expect("");

    let signature_bytes = Vec::from_hex(signature_hex).expect("");
    let signature = Signature::from_der(&signature_bytes).expect("");

    verifying_key.verify(message, &signature).expect("");

    println!("Signature verified successfully");
}
