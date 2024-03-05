use criterion::{black_box, criterion_group, criterion_main, Criterion};
use safe_paillier_rust::{encryption, keygen, decryption};
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

fn paillier_homomorphic_addition_benchmark(c: &mut Criterion) {
    let bit_size = 512;
    let (public_key, private_key) = keygen::key_gen(bit_size);
    let m1 = BigUint::from(123u32);
    let m2 = BigUint::from(456u32);
    let c1 = encryption::encryption(m1, &public_key);
    let c2 = encryption::encryption(m2, &public_key);

    c.bench_function("paillier_homomorphic_addition", |b| {
        b.iter(|| {
            let homomorphic_add = (&c1 * &c2) % public_key.n.pow(2u32);
            decryption::decryption(black_box(homomorphic_add), &public_key, &private_key)
        })
    });
}

fn paillier_homomorphic_multiplication_benchmark(c: &mut Criterion) {
    let bit_size = 512;
    let (public_key, private_key) = keygen::key_gen(bit_size);
    let m1 = BigUint::from(123u32);
    let m2 = BigUint::from(3u32);
    let c1 = encryption::encryption(m1, &public_key);

    c.bench_function("paillier_homomorphic_multiplication", |b| {
        b.iter(|| {
            let homomorphic_mult = c1.modpow(&m2, &public_key.n.pow(2u32));
            decryption::decryption(black_box(homomorphic_mult), &public_key, &private_key)
        })
    });
}

criterion_group!(benches, 
    paillier_encryption_benchmark, 
    paillier_decryption_benchmark,
    paillier_homomorphic_addition_benchmark,
    paillier_homomorphic_multiplication_benchmark
);
criterion_main!(benches);
