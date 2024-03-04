# Rust Paillier Cryptosystem Implementation

## Overview
This project is a Rust implementation of the Paillier cryptosystem, a probabilistic asymmetric algorithm for public key cryptography. The Paillier cryptosystem is known for its homomorphic properties, which allow for computations on ciphertexts that, when decrypted, match the result of operations as if they were carried out on the plaintexts.

The implementation aims to leverage Rust's strong type system and memory safety features to provide a secure and efficient library for cryptographic operations.

## Features
- **Encryption & Decryption**: Message can be encrypted using public key. And ciphertext can be decrypted using the private key. 
- **Homomorphic Encryption**: Utilizing the additive homomorphic properties of the Paillier cryptosystem to perform secure computations on encrypted data.

## Dependencies
This project relies on several Rust crates for handling large integer arithmetic and random number generation:

```toml
[dependencies]
num-bigint = "0.4"
num-prime = "0.4.3"
rand = "0.8"
num-traits = "0.2"
num-modular = "0.6.1"
```

## Getting Started

### Prerequisites
Make sure you have Rust and Cargo installed on your machine. If you don't have them installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/learn/get-started).

### Installation
To start using this Paillier cryptosystem implementation, first, clone the repository to your local machine:

```bash
git clone https://github.com/crypto-keys-unlocked/rust_paillier.git
cd rust_paillier
```
### Compiling
Compile the project using Cargo:

```bash
Copy code
cargo build --release
```
This will compile the project and create an executable in the target/release directory.


## Documentation
For more details on the Paillier cryptosystem and its homomorphic properties, you can refer to the original paper: [Paillier (1999)](https://link.springer.com/content/pdf/10.1007%2F3-540-48910-X_16.pdf).

## Contributing
Contributions to improve this implementation or extend its functionality are welcome. Please ensure that your code adheres to the existing coding standards and includes tests covering new features.

## License
This project is licensed under the MIT License - see the LICENSE file for details.

## Contact
For any queries or collaboration, please open an issue in the repository or contact the maintainers directly.