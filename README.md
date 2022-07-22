# Merkle tree
Simple merkle tree implementation in rust for generating root hashes, 
proof and validating the proof

## Prerequisites
- Rust toolkit (1.62.1)

## Usage
```rust
use merkle_tree::{Keccak256Tree, Keccak256Proof, Sha256, ToHash};
fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let leaves = [
        Sha256::hash("0".as_bytes()), 
        Sha256::hash("1".as_bytes()), 
        Sha256::hash("2".as_bytes()), 
        Sha256::hash("3".as_bytes()), 
        Sha256::hash("4".as_bytes()), 
    ];
    
    let mut tree = Keccak256Tree::from_leaves(&leaves);
    let hash = tree.root_hash()?;
    let proof = tree.get_proof(leaves[3])?;
    
    let proof_validator = Keccak256Proof::new(proof);
    assert!(proof_validator.validate(hash, leaves[3]));
    
    Ok(())
}
```

## Dependencies
#### sha3
[![dependency status](https://deps.rs/crate/sha3/0.10.1/status.svg)](https://deps.rs/crate/sha3/0.10.1)