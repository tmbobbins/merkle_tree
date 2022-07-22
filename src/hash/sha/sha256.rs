use crate::hash::to_hash::ToHash;
use sha3::{Digest, Sha3_256};

pub struct Sha256 {}

impl ToHash for Sha256 {
    type Hash = [u8; 32];

    fn hash(value: &[u8]) -> Self::Hash {
        let mut hasher = Sha3_256::new();
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

        assert_ne!(Sha256::hash(&value_1), Sha256::hash(&value_2));
    }

    #[test]
    fn test_hash_two_identical_values() {
        let value_1: [u8; 1] = [0];

        assert_eq!(Sha256::hash(&value_1), Sha256::hash(&value_1))
    }
}
