pub mod keygen;
pub mod arithmetic {
    pub mod inv;
}

#[cfg(test)]
mod tests {
    use super::keygen::key_gen;
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
}
