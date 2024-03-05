use crate::arithmetic::inv::modinv;
use num_bigint::{BigUint, RandBigInt};
use num_prime::{PrimalityTestConfig, RandPrime};
use rand::thread_rng;
use num_traits::{One, Zero};

/// Represents the public key for the Paillier cryptosystem.
///
/// # Fields
///
/// * `n`: The modulus, a product of two large primes. It's part of the public key and used in encryption.
/// * `g`: A generator in Z*_{n^2}, which is also part of the public key and used in encryption.

pub struct PublicKey {
    pub n: BigUint,
    pub g: BigUint,
}

/// Represents the private key for the Paillier cryptosystem.
///
/// # Fields
///
/// * `lambda`: λ(lcm(p-1, q-1)), where p and q are the prime numbers used to generate `n`. It's used in decryption.
/// * `mu`: The modular multiplicative inverse of L(g^λ mod n^2) mod n, where L is a function defined as L(u) = (u-1)/n. It's also used in decryption.

pub struct PrivateKey {
    pub lambda: BigUint,
    pub mu: BigUint,
}

/// Generates a pair of prime numbers each of a specified bit size for use in the Paillier cryptosystem.
///
/// # Arguments
///
/// * `bit_size` - The bit size for each prime number to be generated, typically ensuring the security level of the cryptosystem.
///
/// # Returns
///
/// A tuple containing two `BigUint` prime numbers `(p, q)`.
///

pub fn generate_prime_pair(bit_size: usize) -> (BigUint, BigUint) {
    let mut rng = thread_rng();
    let config = PrimalityTestConfig::default();
    let p = rng.gen_prime(bit_size, Some(config));
    let q = rng.gen_prime(bit_size, Some(config));

    (p, q)
}

/// Calculates the Carmichael function for two prime numbers.
///
/// The Carmichael function of two primes, `p` and `q`, is defined as the
/// least common multiple (LCM) of `p-1` and `q-1`. This simplified version
/// calculates `p-1` * `q-1` assuming `p` and `q` are prime.
///
/// ## Arguments
///
/// * `p` - A prime number as `BigUint`.
/// * `q` - Another prime number as `BigUint`.
///
/// ## Returns
///
/// The Carmichael function result as a `BigUint`.
///

pub fn carmichael_function(p: BigUint, q: BigUint) -> BigUint {
    let p_minus_one = &p - 1u32;
    let q_minus_one = &q - 1u32;

    p_minus_one * q_minus_one
}


/// Chooses a non-zero `BigUint` in the range 1 to n^2.
///
/// # Arguments
///
/// * `n` - The upper bound for generating a random number.
///
/// # Returns
///
/// A `BigUint` that is greater than 0 and less than n^2.
pub fn choose_g(n: &BigUint) -> BigUint {
    let mut rng = thread_rng();
    let n_square = n.pow(2u32);
    loop {
        let g = rng.gen_biguint_range(&BigUint::one(), &n_square);
        if g != BigUint::zero() {
            return g; 
        }
    }
}

/// When we choose p & q of same length we can simply take g=n+1 this will do the job.
///
/// # Arguments
///
/// * `n` - p*q.
///
/// # Returns
///
/// A `BigUint` n+1.
pub fn g_simple(n: &BigUint) -> BigUint{
    n+1u32
}


/// Calculates the modular multiplicative inverse of phi (φ(n)) modulo n^2.
///
/// This function is used in the context of cryptographic algorithms like Paillier,
/// where finding the modular inverse of the totient function's result (φ(n))
/// modulo n^2 is essential for key generation.
///
/// # Arguments
///
/// * `phi` - The totient of n, φ(n), as a `BigUint`.
/// * `n` - The modulus n, as a `BigUint`.
///
/// # Returns
///
/// The modular multiplicative inverse of φ(n) modulo n^2, if it exists.
/// 
/// # Panics
///
/// This function panics if the modular inverse does not exist, which implies
/// that φ(n) and n^2 are not coprime. In practice, for cryptographic uses,
/// such a scenario should be prevented by proper choice of `n`.
///

pub fn mue(phi: BigUint, n: BigUint) -> BigUint {
    let n_square = &n * &n;
    let inv = modinv(&phi, &n_square)
        .expect("Modular inverse does not exist");

    inv
}


/// Generates a public and private key pair for the Paillier cryptosystem.
///
/// This function generates a pair of public and private keys for the Paillier cryptosystem. The security
/// of the cryptosystem heavily relies on the bit size of the prime numbers `p` and `q` used to generate
/// the keys. Larger bit sizes provide higher security but increase computation time.
///
/// ## Arguments
///
/// * `bit_size` - The bit size for the prime numbers `p` and `q`. This determines the strength of the keys
///                and the security level of the Paillier cryptosystem.
///
/// ## Returns
///
/// A tuple containing the public and private keys `(public_key, private_key)`.
pub fn key_gen(bit_size: usize) -> (PublicKey, PrivateKey) {

    let (p, q) = generate_prime_pair(bit_size);
    
    let n = &p * &q;
    
    let lambda = carmichael_function(p.clone(), q.clone());
    
    let g = g_simple(&n);
    
    let mu = mue(lambda.clone(), n.clone());
    
    let public_key = PublicKey { n: n.clone(), g };
    let private_key = PrivateKey { lambda, mu };
    
    (public_key, private_key)
}


