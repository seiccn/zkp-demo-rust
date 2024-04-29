use zkp_demo_rust::{Prover, Verifier};

fn main() {
    let secret = 3; // The confidential information is exclusively known to the prover.
    let salt = 42; // Typically, this information would be randomly generated.

    println!("Illustration of Zero-Knowledge Proof Using Hashing");
    println!("Secret: {}", secret);

    let prover = Prover::new(secret);
    let proof = prover.generate_proof(salt);
    println!("Generate proof without revealing secret.");

    let public_value = secret; // In a practical scenario, this would typically be a predetermined value associated with the confidential information.
    let verified = Verifier::verify(&proof, public_value);

    if verified {
        println!("Verification successfull.");
    } else {
        println!("Verification failed.");
    }
}
