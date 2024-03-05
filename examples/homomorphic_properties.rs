extern crate safe_paillier_rust;

use safe_paillier_rust::keygen::key_gen;
use safe_paillier_rust::encryption::encryption;
use safe_paillier_rust::decryption::decryption;
use num_bigint::BigUint;

fn main() {
    // Generate public and private keys
    let bit_size = 512;
    let (public_key, private_key) = key_gen(bit_size);

    // Define two plaintext messages
    let m1 = BigUint::from(123u32);
    let m2 = BigUint::from(456u32);

    // Encrypt the messages
    let c1 = encryption(m1.clone(), &public_key);
    let c2 = encryption(m2.clone(), &public_key);

    // Perform homomorphic addition: (E(m1) * E(m2)) mod n^2
    let homomorphic_add = (&c1 * &c2) % public_key.n.pow(2u32);
    let decrypted_add = decryption(homomorphic_add, &public_key, &private_key);
    println!("Homomorphic addition result: {}", decrypted_add);

    // Perform homomorphic multiplication: E(m1)^(m2) mod n^2
    let homomorphic_mult = c1.modpow(&m2, &public_key.n.pow(2u32));
    let decrypted_mult = decryption(homomorphic_mult, &public_key, &private_key);
    println!("Homomorphic multiplication result: {}", decrypted_mult);
}
