use crate::error::tree_error::TreeError;
use crate::hash::to_hash::ToHash;
use crate::merkle_tree::TreeResult;

pub struct MerkleProof<T: ToHash> {
    proof: Vec<T::Hash>,
}

impl<T: ToHash> MerkleProof<T> {
    pub fn new(proof: Vec<T::Hash>) -> Self {
        Self { proof }
    }

    /// Validates a partial proof against a root hash
    ///
    /// ##Examples
    /// ```
    /// use merkle_tree::{Keccak256Tree, Keccak256Proof, Sha256, ToHash};
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let leaves = [
    ///         Sha256::hash("0".as_bytes()),
    ///         Sha256::hash("1".as_bytes()),
    ///         Sha256::hash("2".as_bytes()),
    ///         Sha256::hash("3".as_bytes()),
    ///         Sha256::hash("4".as_bytes()),
    ///     ];
    ///     let mut tree = Keccak256Tree::from_leaves(&leaves);
    ///     let hash = tree.root_hash()?;
    ///     let proof = tree.get_proof(leaves[3])?;
    ///
    ///     let proof_validator = Keccak256Proof::new(proof);
    ///     assert!(proof_validator.validate(hash, leaves[3]));
    ///
    ///     Ok(())
    /// }
    pub fn validate(&self, root_hash: T::Hash, leaf: T::Hash) -> bool {
        let proof_root_hash = match self.reduce_proof(leaf) {
            Ok(hash) => hash,
            Err(_) => return false,
        };

        root_hash == proof_root_hash
    }

    fn reduce_proof(&self, leaf: T::Hash) -> TreeResult<<T as ToHash>::Hash> {
        let mut proof = self.proof.clone();
        proof.insert(0, leaf);
        proof
            .into_iter()
            .reduce(|left, right| T::combine(left, right))
            .ok_or_else(TreeError::proof_empty)
    }
}

#[cfg(test)]
mod tests {
    use crate::hash::Sha256;
    use crate::utils::test::{full_root_hash, raw_leaves_to_hashed_leaves};
    use crate::{Sha256Proof, Sha256Tree};

    #[test]
    fn test_valid_proof() {
        let leaves_raw = ["0", "1", "2", "3", "4"];
        let leaves = raw_leaves_to_hashed_leaves::<Sha256>(&leaves_raw);

        // root hash expectation
        //      [01234]
        //       /   \
        //    [0123]  4-
        //    /   \    \
        //  [01]- [23]  4
        //  /\    /\     \
        // 0 1   2- 3     4
        //
        // Validating 3, requiring 2 <- 3, [01] <- [23], [0123] -> 4

        let full_hash = full_root_hash::<Sha256>(&leaves);

        let mut tree = Sha256Tree::from_leaves(&leaves);
        let partial_proof = tree.get_proof(leaves[3]).unwrap();
        let is_valid = Sha256Proof::new(partial_proof);
        assert!(is_valid.validate(full_hash, leaves[3]));
    }
}
