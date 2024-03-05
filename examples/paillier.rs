use num_bigint::BigUint;
use std::time::Instant;

use rust_paillier::keygen::key_gen;
use rust_paillier::encryption::encryption;
use rust_paillier::decryption::decryption;

fn main() {
    let start = Instant::now();

    let bit_size = 512;

    let (public_key, private_key) = key_gen(bit_size);

    let message_str = "4236483582634342425878462735423874625";
    let message = BigUint::parse_bytes(message_str.as_bytes(), 10).unwrap();
    println!("Plaintext message: {}", message);

    let ciphertext = encryption(message.clone(), &public_key);
    println!("Encrypted ciphertext: {}", ciphertext);

    let decrypted_message = decryption(ciphertext, &public_key, &private_key);
    println!("Decrypted message: {}", decrypted_message);

    assert_eq!(message, decrypted_message, "Decryption did not return the original message!");
    println!("Decryption verified: The decrypted message matches the original message.");

    let duration = start.elapsed();
    println!("Total encryption and decryption process took {:?}", duration);
}
