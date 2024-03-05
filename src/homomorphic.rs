use num_bigint::{BigUint, ToBigUint};
use crate::keygen::{PublicKey, PrivateKey};

/// Adds two ciphertexts homomorphically, resulting in the encryption of the sum of their plaintexts.
///
/// # Arguments
///
/// * `c1` - A ciphertext corresponding to the encryption of `m1`.
/// * `c2` - A ciphertext corresponding to the encryption of `m2`.
/// * `public_key` - The public key used for encryption.
///
/// # Returns
///
/// A `BigUint` representing the encrypted sum of `m1` and `m2`.
pub fn add(c1: BigUint, c2: BigUint, public_key: &PublicKey) -> BigUint {
    let n_square = &public_key.n.pow(2u32);
    (c1 * c2) % n_square
}

/// Multiplies a ciphertext by a plaintext homomorphically, resulting in the encryption of the product.
///
/// # Arguments
///
/// * `c` - A ciphertext corresponding to the encryption of `m1`.
/// * `m2` - A plaintext to multiply by, as a `BigUint`.
/// * `public_key` - The public key used for encryption.
///
/// # Returns
///
/// A `BigUint` representing the encrypted product of `m1` and `m2`.
pub fn multiply(c: BigUint, m2: BigUint, public_key: &PublicKey) -> BigUint {
    let n_square = &public_key.n.pow(2u32);
    c.modpow(&m2, n_square)
}

