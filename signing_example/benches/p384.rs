use criterion::{criterion_group, criterion_main, Criterion};
use p384::{
    ecdsa::{signature::Signer, Signature, SigningKey},
    SecretKey,
};
use p384::ecdsa::signature::Verifier;
use hex::FromHex;

use nix::sched::{sched_setaffinity, CpuSet};
use nix::unistd::Pid;
use std::time::Duration;

mod cycles;
use cycles::CpuCycles;

pub fn comparisons_signing(c: &mut Criterion<CpuCycles>) {
    let mut group = c.benchmark_group("P384");
    group.measurement_time(Duration::from_secs(10));

    let hex_key = "ca8d158af9266d93cfc3d2dca8cf3d40ab303760f8f52da9df6461bfa4cd81129713c1c47ed09868db6f2d0cef750d73";

    let private_key_bytes = <[u8; 48]>::from_hex(hex_key).expect("Invalid hex private key");

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
    let mut group = c.benchmark_group("P384");
    group.measurement_time(Duration::from_secs(10));

    let public_key_hex = "044a6f91016089928524d8e49ede184cc71797453a74570a431ea2300fb8b667c9ac98a61a322433e814f8c64c9a86f986c102c9cff5cb6057900e7416882d00fd786804fe4b39facb6b2562c2c88899910b3342332c946b4a852ea83d118c9c69";

    let signature_hex = "3065023019ea802f67d16ebfa7654e4343ef3c2122e07d14e753678b3abced0f8584a47a468116b379b4bf720a29b3886d5bc5c3023100e7b6f930f7a7b49ad86b8ae69c0d7cd92894cf33e37111fba0d1809b41526db8b835449866d334af158d8dca0bdaf357";

    let message = b"example message";

    let public_key_bytes: [u8; 97] = <[u8; 97]>::from_hex(public_key_hex).expect("Invalid public key hex");

    let verifying_key = p384::ecdsa::VerifyingKey::from_sec1_bytes(&public_key_bytes)
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
