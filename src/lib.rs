pub use crate::hash::to_hash::ToHash;
pub use crate::hash::Keccak256;
pub use crate::hash::Sha256;
pub use crate::hash::Sha512;
pub use crate::merkle_proof::MerkleProof;
pub use crate::merkle_tree::MerkleTree;

pub mod error;
mod hash;
mod merkle_proof;
mod merkle_tree;
mod utils;

pub type Keccak256Tree = MerkleTree<Keccak256>;
pub type Keccak256Proof = MerkleProof<Keccak256>;
pub type Sha256Tree = MerkleTree<Sha256>;
pub type Sha256Proof = MerkleProof<Sha256>;
pub type Sha512Tree = MerkleTree<Sha512>;
pub type Sha512Proof = MerkleProof<Sha512>;
