use crate::hash::to_hash::ToHash;
use sha3::{Digest, Sha3_512};

pub struct Sha512 {}

impl ToHash for Sha512 {
    type Hash = [u8; 64];

    fn hash(value: &[u8]) -> Self::Hash {
        let mut hasher = Sha3_512::new();
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

        assert_ne!(Sha512::hash(&value_1), Sha512::hash(&value_2));
    }

    #[test]
    fn test_hash_two_identical_values() {
        let value_1: [u8; 1] = [0];

        assert_eq!(Sha512::hash(&value_1), Sha512::hash(&value_1))
    }
}
