use criterion::{criterion_group, criterion_main, Criterion};
use p521::{
    ecdsa::{signature::Signer, Signature, SigningKey},
    SecretKey,
};
use p521::ecdsa::signature::Verifier;
use hex::FromHex;

use nix::sched::{sched_setaffinity, CpuSet};
use nix::unistd::Pid;
use std::time::Duration;

mod cycles;
use cycles::CpuCycles;

pub fn comparisons_signing(c: &mut Criterion<CpuCycles>) {
    let mut group = c.benchmark_group("P521");
    group.measurement_time(Duration::from_secs(10));

    let hex_key = "004cd18fb54d41542d43c1fc8957aff5590c9e76d48570d390aca50b9f199442a0ecbc57b2117c9127efbd94eb724035aec08f231ff1e66b370d5d0f620ba61cd8b4";

    let private_key_bytes: [u8; 66] = <[u8; 66]>::from_hex(hex_key).expect("Invalid hex private key");

    let secret_key = SecretKey::from_bytes((&private_key_bytes).into()).expect("Invalid private key bytes");

    let signing_key = SigningKey::from(secret_key);

    let message = b"example message";

    group.bench_function("sign", move |b| {
        b.iter(|| {
            let _signature: Signature = signing_key.sign(message);
        })
    });

    group.finish();
}

pub fn comparisons_verifying(c: &mut Criterion<CpuCycles>) {
    let mut group = c.benchmark_group("P521");
    group.measurement_time(Duration::from_secs(10));

    let public_key_hex = "0400211033d232353598272065619d0b413f2ab6e89f679fbb10fa631edf79aaf6cd526a3a71b713d8435fba360d9d97bc38f380559a8e7b857b6e1fd7491ee8f10a5701b4cd27339a0a6722b9bf43e9d7c5c1596093829de71458a8c9c7ea84e92ed1cdf85bc3563c724465918eb5274c1350214ebe5c58fdf205cd657df53193523f1579";

    let signature_hex = "30818702412b61dc0161fd8dfbfb9171e01ce8af70f92b8a5dfa261951318beaa870b9b53e63c3402c1b4372c1c78634ae9dbd96215c403c659615affbe11a900480e4c13d94024201f0c1a66f18c91fe18e72d7fc0ae58b27dc67bbef21e54bb371091eed6cf3f0d6eec9544ee1a81bed81b37c279aecced330e40025af9a8b8d2942ea379c18722881";

    let message = b"example message";

    let public_key_vec = Vec::from_hex(public_key_hex).expect("Invalid public key hex");
    let public_key_bytes: [u8; 133] = public_key_vec.try_into().expect("Invalid length for public key");
    

    let verifying_key = p521::ecdsa::VerifyingKey::from_sec1_bytes(&public_key_bytes)
        .expect("Invalid public key bytes");

    let signature_bytes = Vec::from_hex(signature_hex).expect("Invalid signature hex");
    let signature = Signature::from_der(&signature_bytes).expect("Invalid DER signature");

    group.bench_function("verify", |b| {
        b.iter(|| {
            verifying_key.verify(message, &signature).expect("verification failed");
        })
    });

    group.finish();
}


pub fn comparisons(c: &mut Criterion<CpuCycles>) {
    let num_cpus = unsafe {
        let n = nix::libc::sysconf(nix::libc::_SC_NPROCESSORS_ONLN);
        if n < 1 {
            panic!("Failed to get number of CPUs");
        }
        n as usize
    };

    let mut cpuset = CpuSet::new();
    cpuset.set(num_cpus - 1).expect("Failed to set CPU in cpuset");

    sched_setaffinity(Pid::from_raw(0), &cpuset).expect("Failed to set CPU affinity");

    comparisons_signing(c);
    comparisons_verifying(c);
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_measurement(CpuCycles);
    targets = comparisons
);
criterion_main!(benches);
