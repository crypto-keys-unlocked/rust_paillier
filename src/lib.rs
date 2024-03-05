pub mod keygen;
pub mod encryption;
pub mod decryption; 
pub mod arithmetic {
    pub mod inv;
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;
    use keygen::key_gen;
    use num_traits::Zero;
    use std::time::Instant;

    #[test]
    fn test_key_gen_timing() {
        let bit_size = 512;

        let start = Instant::now();
        
        let (public_key, private_key) = key_gen(bit_size);
        
        let duration = start.elapsed();

        println!("Public Key: (n: {}, g: {})", public_key.n, public_key.g);
        println!("Private Key: (lambda: {}, mu: {})", private_key.lambda, private_key.mu);
        println!("Time required for key generation: {:?}", duration);

        assert!(!public_key.n.is_zero(), "Public key 'n' should not be zero.");
        assert!(!public_key.g.is_zero(), "Public key 'g' should not be zero.");

    }
    #[test]
    fn test_encryption_decryption_cycle() {
        let bit_size = 512;

        let (public_key, private_key) = keygen::key_gen(bit_size);

        let message_str = "4236483582634342425878462735423874625";
        let message = BigUint::parse_bytes(message_str.as_bytes(), 10).unwrap();
        println!("Plaintext: {}", message);

        let ciphertext = encryption::encryption(message.clone(), &public_key);
        println!("Ciphertext: {}", ciphertext);

        let decrypted_message = decryption::decryption(ciphertext, &public_key, &private_key);
        println!("Decrypted message: {}", decrypted_message);

        assert_eq!(message, decrypted_message, "Decryption did not match the original message.");
    }
}
