use p384::{
    ecdsa::{signature::Verifier, Signature},
};
use hex::FromHex;

fn main() {
    let public_key_hex = "046948cd7fd64c36a388acbf041ab0740330db4477b42ac8906a81c827c8e677ec3e6dd54db5b1d4879a7e1840298e610704d49e34fee0670764c39b08aaca76f3e7fda5873b3a69bef16b1e6c2a317ee3e3ffeae58f6536a656c3236f015bce74";

    let signature_hex = "3065023100a204ebc090ac53fa071cec200a56b0296ae90f6aa127fb2ffc9a60b33afeea31592682a079cfee598b5980909ee1a119023053890459af4786e3e8806e14e4aaab93ad2378ad045062dc083e140045cbcce3af8a7d272ee486db3abe5a8a85b1c9ba";

    let message = b"";

    let public_key_vec = Vec::from_hex(public_key_hex).expect("");

    let verifying_key = p384::ecdsa::VerifyingKey::from_sec1_bytes(&public_key_vec)
        .expect("");

    let signature_bytes = Vec::from_hex(signature_hex).expect("");
    let signature = Signature::from_der(&signature_bytes).expect("");

    verifying_key.verify(message, &signature).expect("");

    println!("Signature verified successfully");
}
