# safe_paillier_rust

## Overview
`safe_paillier_rust` is a Rust implementation of the Paillier cryptosystem, a probabilistic asymmetric algorithm for public key cryptography renowned for its homomorphic properties. These properties enable secure computations on ciphertexts that, once decrypted, yield results as if operations were directly performed on the plaintexts.

Leveraging Rust's strong type system and memory safety, this library aims to offer a secure and efficient solution for cryptographic operations, particularly in contexts requiring data privacy and integrity.

## Features
- **Key Generation**: Generates secure public and private keys for encryption and decryption.
- **Encryption & Decryption**: Facilitates message encryption using a public key and decryption with the corresponding private key.
- **Homomorphic Properties**: Supports additive and multiplicative operations on encrypted data, maintaining consistency with operations performed on plaintexts.
- **Modular Arithmetic Support**: Provides utilities for modular inversion and other arithmetic operations essential to the cryptosystem.

## Dependencies
The library depends on several Rust crates for big integer arithmetic and random number generation, ensuring robust and efficient implementation:

```toml
[dependencies]
num-bigint = "0.4"
num-prime = "0.4.3"
rand = "0.8"
num-traits = "0.2"
num-modular = "0.6.1"
```

## Installation and Usage

### Adding the Crate to Your Project
To use `safe_paillier_rust` in your Rust project, add it as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
safe_paillier_rust = "0.1.4"
```

Make sure to use the latest version of `safe_paillier_rust` available on [crates.io](https://crates.io/crates/safe_paillier_rust).

### Example Usage
After adding the crate to your project, you can use it to perform cryptographic operations, such as key generation, encryption, and decryption. Here's a basic example demonstrating these functionalities:

```rust
// Import the necessary modules from the crate
use safe_paillier_rust::{keygen::key_gen, encryption::encryption, decryption::decryption};
use num_bigint::BigUint;

fn main() {
    // Generate public and private keys with a specified bit size
    let bit_size = 512;
    let (public_key, private_key) = key_gen(bit_size);

    // Define a message as a BigUint
    let message = BigUint::from(123u32);

    // Encrypt the message using the public key
    let ciphertext = encryption(message.clone(), &public_key);

    // Decrypt the ciphertext using the private key and public key
    let decrypted_message = decryption(ciphertext, &public_key, &private_key);

    // Verify the decryption result matches the original message
    assert_eq!(message, decrypted_message);
    println!("Encryption and decryption successful!");
}
```

This example demonstrates how to integrate `safe_paillier_rust` into your Rust project for secure cryptographic operations. Replace the `bit_size` and `message` values as needed for your specific use case.

## Benchmarking
The library includes benchmarks to evaluate the performance of cryptographic operations, including homomorphic additions and multiplications. To run these benchmarks:

1. Ensure you have the Rust Nightly toolchain installed.
2. Execute `cargo bench` from the project root.

Refer to the project documentation for detailed benchmark instructions and results.

## Documentation
For a deeper understanding of the Paillier cryptosystem and its homomorphic capabilities, consult the [original paper by Pascal Paillier (1999)](https://link.springer.com/content/pdf/10.1007%2F3-540-48910-X_16.pdf).

## Contributing
Contributions are welcome to enhance the library's features or performance. Please adhere to existing coding standards and include tests for new functionalities.

## License
This project is made available under the MIT License. See the LICENSE file for more details.

## Contact
For inquiries or collaboration, please open an issue in the [GitHub repository](https://github.com/crypto-keys-unlocked/safe_paillier_rust) or contact the maintainers directly.