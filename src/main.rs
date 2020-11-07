extern crate hex;
mod merkle_tree;

use merkle_tree::construct_merkle_tree;
use sha2::{Digest, Sha256};

fn main() {
    let leaves = ["to", "be", "or", "not", "to", "be", "is", "the", "question"];

    let merkle_tree = construct_merkle_tree(&leaves, custom_hash_algo);
    println!("Sha256 hash is: {:?}", hex::encode(merkle_tree.root));
}

/**
 * Customizable method as required by merkle_tree mod
 */
fn custom_hash_algo(content: &[u8]) -> Vec<u8> {
    let mut hash_fn = Sha256::new();
    hash_fn.update(content);
    hash_fn.finalize().iter().cloned().collect()
}
