use crate::hash::to_hash::ToHash;
use sha3::{Digest, Keccak256 as Sha3_Keccak256};

pub struct Keccak256 {}

impl ToHash for Keccak256 {
    type Hash = [u8; 32];

    fn hash(value: &[u8]) -> Self::Hash {
        let mut hasher = Sha3_Keccak256::new();
        hasher.update(value);
        hasher.finalize().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_two_different_values() {
        let value_1: [u8; 1] = [0];
        let value_2: [u8; 1] = [1];

        assert_ne!(Keccak256::hash(&value_1), Keccak256::hash(&value_2));
    }

    #[test]
    fn test_hash_two_identical_values() {
        let value_1: [u8; 1] = [0];

        assert_eq!(Keccak256::hash(&value_1), Keccak256::hash(&value_1))
    }
}
