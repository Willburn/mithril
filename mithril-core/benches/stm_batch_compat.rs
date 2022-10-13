use blake2::digest::{Digest, FixedOutput};
use blake2::{digest::consts::U32, Blake2b};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use mithril::key_reg::KeyReg;
use mithril::stm::StmParameters;
use mithril::stm_batch_compat::{
    StmClerkBatchCompact, StmInitializerBatchCompat, StmSignerBatchCompat,
};
use rand_chacha::ChaCha20Rng;
use rand_core::{RngCore, SeedableRng};
use rayon::prelude::*;
use std::fmt::Debug;

///
/// This benchmark framework is not ideal. We really have to think what is the best mechanism for
/// benchmarking these signatures, over which parameters, how many times to run them, etc:
/// * Registration depends on the number of parties (should be constant, as it is a lookup table)
/// * Signing depends on the parameter `m`, as it defines the number of lotteries a user can play
/// * Aggregation depends on `k`.
/// * Verification is independent from the parameters.

fn stm_benches<H>(c: &mut Criterion, nr_parties: usize, params: StmParameters, hashing_alg: &str)
where
    H: Clone + Debug + Digest + Send + Sync + FixedOutput,
{
    let mut group = c.benchmark_group(format!("STM/{}", hashing_alg));
    let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
    let mut msg = [0u8; 16];
    rng.fill_bytes(&mut msg);

    let param_string = format!(
        "k: {}, m: {}, nr_parties: {}",
        params.k, params.m, nr_parties
    );

    let stakes = (0..nr_parties)
        .into_iter()
        .map(|_| 1 + (rng.next_u64() % 9999))
        .collect::<Vec<_>>();

    let mut initializers: Vec<StmInitializerBatchCompat> = Vec::with_capacity(nr_parties);
    for stake in stakes {
        initializers.push(StmInitializerBatchCompat::setup(params, stake, &mut rng));
    }
    let mut key_reg = KeyReg::init();

    group.bench_function(BenchmarkId::new("Key registration", &param_string), |b| {
        b.iter(|| {
            // We need to initialise the key_reg at each iteration
            key_reg = KeyReg::init();
            for p in initializers.iter() {
                key_reg.register(p.stake, p.verification_key()).unwrap();
            }
        })
    });

    let closed_reg = key_reg.close();

    let signers = initializers
        .into_par_iter()
        .map(|p| p.new_signer_batch_compat(closed_reg.clone()).unwrap())
        .collect::<Vec<StmSignerBatchCompat<H>>>();

    group.bench_function(BenchmarkId::new("Play all lotteries", &param_string), |b| {
        b.iter(|| {
            signers[0].sign(&msg);
        })
    });

    let sigs = signers
        .par_iter()
        .filter_map(|p| p.sign(&msg))
        .collect::<Vec<_>>();

    let clerk = StmClerkBatchCompact::from_signer_batch_compat(&signers[0]);
    let msig = clerk.aggregate_batch_compat(&sigs, &msg).unwrap();

    group.bench_function(BenchmarkId::new("Aggregation", &param_string), |b| {
        b.iter(|| clerk.aggregate_batch_compat(&sigs, &msg))
    });

    group.bench_function(BenchmarkId::new("Verification", &param_string), |b| {
        b.iter(|| {
            msig.verify(&msg, &clerk.compute_avk_batch_compat(), &params)
                .is_ok()
        })
    });
}

fn stm_benches_blake_300(c: &mut Criterion) {
    stm_benches::<Blake2b<U32>>(
        c,
        300,
        StmParameters {
            m: 150,
            k: 25,
            phi_f: 0.2,
        },
        "Blake2b",
    );
}

fn stm_benches_blake_2000(c: &mut Criterion) {
    stm_benches::<Blake2b<U32>>(
        c,
        2000,
        StmParameters {
            m: 1523,
            k: 250,
            phi_f: 0.2,
        },
        "Blake2b",
    );
}

criterion_group!(name = benches;
                 config = Criterion::default().nresamples(1000);
                 targets = stm_benches_blake_300, stm_benches_blake_2000);
criterion_main!(benches);
