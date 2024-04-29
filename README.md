
# Zero Knowledge Proof (ZKP) Demonstration in Rust

This project is a Rust-based demonstration of a simple Zero-Knowledge Proof (ZKP) system. It illustrates how a prover can convince a verifier that they know a secret without revealing the secret itself.

## Introduction to ZKP

Zero-Knowledge Proofs constitute a subset of cryptographic techniques enabling one party (the prover) to verify the truth of a statement to another party (the verifier) without disclosing any additional information beyond the statement's validity.

Analogously, consider a scenario where a color-blind individual seeks to ascertain whether two balls share the same color, without discerning the actual color. The prover, possessing color perception, can affirm or negate their similarity without divulging the precise color to the color-blind verifier.

## Project Overview

The project is organized in the following manner:

- `src/lib.rs`: Houses the primary ZKP functionality, featuring a basic arithmetic circuit as an illustrative example.
- `tests/lib_test.rs`: Comprises test cases to verify the accuracy of the ZKP implementation.
- `Cargo.toml`: Serves as the configuration file for Rust's package manager, encompassing project metadata and dependencies.

## How to Build and Run

Make sure you have Rust installed on your system. If not, you can install it using [rustup](https://rustup.rs/).

To execute the project, navigate to its directory in your terminal and enter the following command:

```
cargo run
```

## Implementation Details

The project embodies a straightforward ZKP system where the 'circuit' is a basic square operation. The prover crafts a proof by squaring the confidential value, and the verifier verifies whether the squared value of the confidential input (publicly known) corresponds to the commitment presented in the proof.
