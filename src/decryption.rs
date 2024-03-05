use num_traits::One;
use num_bigint::BigUint;
use crate::keygen::{PrivateKey,PublicKey};

/// Computes the L function, L(x) = (x - 1) / n, used in the decryption process.
///
/// # Arguments
///
/// * `x` - A `BigUint` value to apply the L function to.
/// * `n` - The modulus `n` from the public key.
///
/// # Returns
///
/// The result of applying the L function to `x`.
fn l_function(x: BigUint, n: &BigUint) -> BigUint {
    (x - BigUint::one()) / n
}

/// Decrypts a ciphertext using the Paillier cryptosystem's private key.
///
/// # Arguments
///
/// * `c` - The ciphertext to decrypt, as a `BigUint`.
/// * `public_key` - The public key, containing `n`.
/// * `private_key` - The private key, containing `lambda` and `mu`.
///
/// # Returns
///
/// The decrypted plaintext, as a `BigUint`.
pub fn decryption(c: BigUint, public_key: &PublicKey, private_key: &PrivateKey) -> BigUint {
    let n_square = &public_key.n * &public_key.n;
    let c_lambda = c.modpow(&private_key.lambda, &n_square);
    let l_c_lambda = l_function(c_lambda, &public_key.n);
    let m = (l_c_lambda * &private_key.mu) % &public_key.n;
    
    m
}