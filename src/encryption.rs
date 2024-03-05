use num_bigint::{BigUint, RandBigInt};
use rand::thread_rng;
use num_traits::One;
use crate::keygen::PublicKey;

/// Generates a random `BigUint` in the range [1, n).
///
/// # Arguments
///
/// * `n` - The upper bound for random number generation.
///
/// # Returns
///
/// A random `BigUint` in the specified range.
fn gen_rand(n: &BigUint) -> BigUint {
    let mut rng = thread_rng();
    rng.gen_biguint_range(&BigUint::one(), n)
}

/// Encrypts a plaintext message using the Paillier cryptosystem's public key.
///
/// # Arguments
///
/// * `m` - The plaintext message as a `BigUint`.
/// * `public_key` - The public key containing `n` and `g`.
///
/// # Returns
///
/// The encrypted ciphertext as a `BigUint`.
pub fn encryption(m: BigUint, public_key: &PublicKey) -> BigUint {
    let n = &public_key.n;
    let g = &public_key.g;
    let r = gen_rand(n);
    let n_square = n.pow(2u32);
    let gm = g.modpow(&m, &n_square);
    let rn = r.modpow(n, &n_square);
    (gm * rn) % &n_square
}
