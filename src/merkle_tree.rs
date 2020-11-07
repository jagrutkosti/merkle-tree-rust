use bytevec::ByteEncodable;

pub struct MerkleTree {
    pub root: Vec<u8>,
    hashed_leaves: Vec<Vec<u8>>,
    hash_fn: fn(&[u8]) -> Vec<u8>,
}

/**
 * Constructs merkle tree by
 * 1: Converts any vector/array type to array of bytes
 * 2: Hashes those leaves to get hashed_leaves by using the given hash function
 * 3: Calculates the root hash of the tree and returns MerkleTree
 */
pub fn construct_merkle_tree<T>(leaves: &[T], hash_fn: fn(&[u8]) -> Vec<u8>) -> MerkleTree
where
    T: ByteEncodable,
{
    let byte_leaves = convert_to_bytes(leaves);
    let mut hashed_leaves: Vec<Vec<u8>> = Vec::with_capacity(byte_leaves.len());

    for leaf in byte_leaves {
        hashed_leaves.push(hash_fn(&leaf));
    }

    let root = generate_merkle_root(&hashed_leaves, hash_fn);

    MerkleTree {
        root,
        hashed_leaves,
        hash_fn,
    }
}

/**
 * Takes an array of any type and for each item in array converts to bytes
 */
fn convert_to_bytes<T>(vector: &[T]) -> Vec<Vec<u8>>
where
    T: ByteEncodable,
{
    let mut bytes_vector: Vec<Vec<u8>> = Vec::new();
    for item in vector {
        // the conversion is unsafe. Cannot be guaranteed it works everytime!!
        bytes_vector.push(item.encode::<u8>().unwrap());
    }

    bytes_vector
}

/**
 * Calculates the merkle root
 */
fn generate_merkle_root(leaves: &[Vec<u8>], hash_fn: fn(&[u8]) -> Vec<u8>) -> Vec<u8> {
    let mut i = 0;
    let mut parent: Vec<Vec<u8>> = Vec::new();
    if leaves.len() == 1 {
        return leaves[0].clone();
    }

    while i < leaves.len() - 1 {
        let mut combined_content: Vec<u8> = Vec::new();
        combined_content.append(&mut leaves[i].clone());
        combined_content.append(&mut leaves[i + 1].clone());
        parent.push(hash_fn(&combined_content));
        i += 2;
    }

    generate_merkle_root(&parent, hash_fn)
}
