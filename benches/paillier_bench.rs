use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_paillier::{encryption, keygen, decryption};
use num_bigint::BigUint;

fn paillier_encryption_benchmark(c: &mut Criterion) {
    let bit_size = 512;
    let (public_key, _private_key) = keygen::key_gen(bit_size);
    let message = BigUint::from(123u32);

    c.bench_function("paillier_encryption", |b| {
        b.iter(|| encryption::encryption(black_box(message.clone()), &public_key))
    });
}

fn paillier_decryption_benchmark(c: &mut Criterion) {
    let bit_size = 512;
    let (public_key, private_key) = keygen::key_gen(bit_size);
    let message = BigUint::from(123u32);
    let ciphertext = encryption::encryption(message, &public_key);

    c.bench_function("paillier_decryption", |b| {
        b.iter(|| decryption::decryption(black_box(ciphertext.clone()), &public_key, &private_key))
    });
}

criterion_group!(benches, paillier_encryption_benchmark, paillier_decryption_benchmark);
criterion_main!(benches);
