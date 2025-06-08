use criterion::{criterion_group, criterion_main, Criterion};
use p256::{
    ecdsa::{signature::Signer, Signature, SigningKey},
    SecretKey,
};
use rand_core::OsRng;
use p256::ecdsa::signature::Verifier;
use hex::FromHex;

use nix::sched::{sched_setaffinity, CpuSet};
use nix::unistd::Pid;
use std::time::Duration;

mod cycles;
use cycles::CpuCycles;

pub fn comparisons_signing(c: &mut Criterion<CpuCycles>) {
    let mut group = c.benchmark_group("P256");
    group.measurement_time(Duration::from_secs(10));

    let hex_key = "69624171561a63340de0e7d869f2a05492558e1a04868b6a9f854a866788188d";

    let private_key_bytes = <[u8; 32]>::from_hex(hex_key).expect("");

    let secret_key = SecretKey::from_bytes((&private_key_bytes).into()).expect("");

    let signing_key = SigningKey::from(secret_key);

    let message = b"example";

    group.bench_function("sign", move |b| {
        b.iter(|| {
            let _signature: Signature = signing_key.sign(message);
        })
    });

    group.finish();
}

pub fn comparisons_verifying(c: &mut Criterion<CpuCycles>) {
    let mut group = c.benchmark_group("P256");
    group.measurement_time(Duration::from_secs(10));

    let public_key_hex = "041cacffb55f2f2cefd89d89eb374b2681152452802deea09916068137d839cf7fc481a44492304d7ef66ac117befe83a8d08f155f2b52f9f618dd447029048e0f";

    let signature_hex = "304402201dc361817b59807431b3bc66160c0e9911bb4b1356a6cbd097754cbdff1d057e0220090dc2ee4a886400aebeb40218fff574c9b45e5a92f0a38ca0e7eefb75fea64d";

    let message = b"example message";

    let public_key_bytes = <[u8; 65]>::from_hex(public_key_hex).expect("");

    let verifying_key = p256::ecdsa::VerifyingKey::from_sec1_bytes(&public_key_bytes)
        .expect("");

    let signature_bytes = Vec::from_hex(signature_hex).expect("");
    let signature = Signature::from_der(&signature_bytes).expect("");

    group.bench_function("verify", |b| {
        b.iter(|| {
            verifying_key.verify(message, &signature).expect("");
        })
    });

    group.finish();
}

pub fn comparisons_generate_key(c: &mut Criterion<CpuCycles>) {
    let mut group = c.benchmark_group("P256");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("generate_key", |b| {
        b.iter(|| {
            let _signing_key = SigningKey::try_from_rng(&mut OsRng).unwrap();
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
    cpuset.set(num_cpus - 1).expect("");

    sched_setaffinity(Pid::from_raw(0), &cpuset).expect("");

    comparisons_signing(c);
    comparisons_verifying(c);
    comparisons_generate_key(c);
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_measurement(CpuCycles);
    targets = comparisons
);
criterion_main!(benches);
