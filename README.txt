# Voting Simulation with Cupcake Encryption

## Overview

This project simulates a voting system where votes are encrypted, processed, and decrypted using the **Cupcake** library, an experimental cryptographic library for Rust. The program uses Rust's standard library for basic CLI interactions and the `rand` crate for generating randomized data to simulate votes.

### What Does the Program Do?

- **Vote Simulation**: Generates random votes to mimic user input.
- **Encryption**: Encrypts each vote using the Cupcake library for data privacy.
- **Processing**: Decrypts votes to tally the results, showcasing how encrypted data can be processed securely.
- **CLI Interface**: Provides basic command-line interaction for users to view results or distribution of votes.

## Technology Behind Cupcake

**Cupcake** is a lightweight cryptographic library designed for educational purposes and experimental cryptography in Rust:
- **Encryption**: Implements various encryption techniques, likely focusing on ease of use and learning rather than industrial-strength security.
- **Decryption**: Provides corresponding decryption methods to reverse the encryption process.

**Note**: Cupcake is not intended for production use due to its experimental nature. Use in real-world applications should be done with caution and under the understanding that it might not provide robust security.

## Installation

1. **Prerequisites**:
	- Rust toolchain installed (`rustc`, `cargo`). You can install Rust from [rust-lang.org](https://www.rust-lang.org/tools/install).

2. **Clone Cupcake Library**:    - Cupcake needs to be cloned and compiled separately since it's not available on crates.io:
        - git clone https://github.com/facebookresearch/Cupcake.git
        - cd Cupcake
	- cargo build --release

3. **Clone the Repository**:
	- git clone https://github.com/Marc0LovesCode/CupcakeVotingSim
	- cd CupcakeVotingSim
	- cargo build --release
	- cargo run
