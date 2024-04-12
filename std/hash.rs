use std::crypto::digest::Digest;
use std::crypto::sha2::Sha256;

fn main() {
    // Hashing with SHA-256
    let mut hasher = Sha256::new();
    hasher.input_str("Hello, Rust!");

    let result = hasher.result_str();
    println!("SHA-256 hash: {}", result);
}
