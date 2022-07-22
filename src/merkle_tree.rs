use crate::error::tree_error::TreeError;
use crate::hash::to_hash::ToHash;

pub type TreeResult<T> = Result<T, TreeError>;

pub struct MerkleTree<T: ToHash> {
    leaves: Vec<T::Hash>,
    path: Vec<T::Hash>,
    current_path_leaf: Option<T::Hash>,
}

impl<T: ToHash> MerkleTree<T> {
    pub fn new() -> Self {
        Self {
            leaves: Vec::new(),
            path: Vec::new(),
            current_path_leaf: None,
        }
    }

    pub fn from_leaves(leaves: &[T::Hash]) -> Self {
        Self {
            leaves: leaves.into(),
            path: Vec::new(),
            current_path_leaf: None,
        }
    }

    /// Appends a leaf to the tree
    ///
    /// ##Examples
    /// ```
    /// use merkle_tree::{Sha256Tree, Sha256, ToHash};
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut tree = Sha256Tree::new();
    ///     tree.append(Sha256::hash("0".as_bytes()));
    ///     tree.append(Sha256::hash("1".as_bytes()));
    ///     tree.append(Sha256::hash("2".as_bytes()));
    ///     tree.append(Sha256::hash("3".as_bytes()));
    ///     tree.append(Sha256::hash("4".as_bytes()));
    ///
    ///     let hash = tree.root_hash()?;
    ///     let expected_hash = [
    ///         79, 188, 211, 24, 18, 206, 74, 81, 223, 16, 54, 39, 77, 67, 62, 72, 61,
    ///         164, 15, 45, 199, 119, 195, 239, 187, 172, 49, 64, 115, 159, 58, 83
    ///     ];
    ///     assert_eq!(hash, expected_hash);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn append(&mut self, leaf: T::Hash) {
        self.leaves.push(leaf);
    }

    /// Creates a root hash for the given tree
    ///
    /// ##Examples
    /// ```
    /// use merkle_tree::{Sha256Tree, Sha256, ToHash};
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let leaves = [
    ///         Sha256::hash("0".as_bytes()),
    ///         Sha256::hash("1".as_bytes()),
    ///         Sha256::hash("2".as_bytes()),
    ///         Sha256::hash("3".as_bytes()),
    ///         Sha256::hash("4".as_bytes()),
    ///     ];
    ///     let mut tree = Sha256Tree::from_leaves(&leaves);
    ///     let hash = tree.root_hash()?;
    ///     let expected_hash = [
    ///         79, 188, 211, 24, 18, 206, 74, 81, 223, 16, 54, 39, 77, 67, 62, 72, 61,
    ///         164, 15, 45, 199, 119, 195, 239, 187, 172, 49, 64, 115, 159, 58, 83
    ///     ];
    ///     assert_eq!(hash, expected_hash);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn root_hash(&mut self) -> TreeResult<T::Hash> {
        if self.leaves.is_empty() {
            return Err(TreeError::tree_empty());
        }

        Ok(self.reduce_tree(&mut self.leaves.clone(), false)?[0])
    }

    /// Creates a proof (path) for validating presence of the leaf in the tree
    ///
    /// ##Examples
    /// ```
    /// use merkle_tree::{Sha256Tree, Sha256, ToHash};
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let leaves = [
    ///         Sha256::hash("0".as_bytes()),
    ///         Sha256::hash("1".as_bytes()),
    ///         Sha256::hash("2".as_bytes()),
    ///         Sha256::hash("3".as_bytes()),
    ///         Sha256::hash("4".as_bytes()),
    ///     ];
    ///     let mut tree = Sha256Tree::from_leaves(&leaves);
    ///     let proof = tree.get_proof(leaves[3])?;
    ///     let expected_proof = vec![
    ///         leaves[2],
    ///         Sha256::combine(leaves[0], leaves[1]),
    ///         leaves[4],
    ///     ];
    ///
    ///     assert_eq!(proof, expected_proof);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn get_proof(&mut self, leaf: T::Hash) -> TreeResult<Vec<T::Hash>> {
        self.current_path_leaf = Some(leaf);
        self.reduce_tree(&mut self.leaves.clone(), true)?;
        let proof = self.path.clone();
        self.clear_path();

        Ok(proof)
    }

    fn clear_path(&mut self) {
        self.path = Vec::new();
        self.current_path_leaf = None;
    }

    fn reduce_tree(
        &mut self,
        leaves: &mut Vec<T::Hash>,
        generate_path: bool,
    ) -> TreeResult<Vec<T::Hash>> {
        let mut processed_leaves = self.process_leaves_in_pairs(leaves, generate_path)?;

        if processed_leaves.len() > 1 {
            processed_leaves = self.reduce_tree(&mut processed_leaves, generate_path)?;
        }

        Ok(processed_leaves)
    }

    fn process_leaves_in_pairs(
        &mut self,
        leaves: &mut Vec<T::Hash>,
        generate_path: bool,
    ) -> TreeResult<Vec<T::Hash>> {
        let mut processed_leaves = vec![];
        for index in 0..leaves.len() / 2 {
            let leaf_left = leaves[2 * index];
            let leaf_right = leaves[2 * index + 1];
            let combined_leaf = T::combine(leaf_left, leaf_right);
            processed_leaves.push(combined_leaf);
            if generate_path {
                self.add_to_path(leaf_left, leaf_right, combined_leaf)?;
            }
        }

        if leaves.len() % 2 == 1 {
            processed_leaves.push(*leaves.last().ok_or_else(TreeError::leaf_empty)?);
        }

        Ok(processed_leaves)
    }

    fn add_to_path(
        &mut self,
        leaf_left: T::Hash,
        leaf_right: T::Hash,
        combined_leaf: T::Hash,
    ) -> TreeResult<()> {
        let current_path_leaf = self
            .current_path_leaf
            .ok_or_else(TreeError::path_leaf_not_set)?;

        if leaf_left != current_path_leaf && leaf_right != current_path_leaf {
            return Ok(());
        }

        if leaf_left == current_path_leaf {
            self.path.push(leaf_right);
        }

        if leaf_right == current_path_leaf {
            self.path.push(leaf_left);
        }

        self.current_path_leaf = Some(combined_leaf);

        Ok(())
    }
}

impl<T: ToHash> Default for MerkleTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::error::tree_error::TreeErrorKind;
    use crate::hash::Sha256;
    use crate::utils::test::{full_root_hash, raw_leaves_to_hashed_leaves};
    use crate::Sha256Tree;

    #[test]
    fn test_empty_tree_returns_error() {
        let mut tree = Sha256Tree::from_leaves(&[]);
        assert_eq!(
            tree.root_hash().unwrap_err().kind(),
            &TreeErrorKind::TreeEmpty
        );
    }

    #[test]
    fn test_valid_root_hash() {
        let leaves_raw = ["0", "1", "2", "3", "4"];
        let leaves = raw_leaves_to_hashed_leaves::<Sha256>(&leaves_raw);

        let mut tree = Sha256Tree::from_leaves(&leaves);
        assert_eq!(tree.root_hash().unwrap(), full_root_hash::<Sha256>(&leaves));
    }

    #[test]
    fn test_proof() {
        let leaves_raw = ["0", "1", "2", "3", "4"];
        let leaves = raw_leaves_to_hashed_leaves::<Sha256>(&leaves_raw);

        // root hash expectation
        //      [01234]
        //       /    \
        //    [0123]   4-
        //    /    \    \
        //  [01]- [23]   4
        //  /\     /\     \
        // 0 1    2- 3     4
        // proof for leaf [2 <-, [01] <-, -> 4]
        let expected_proof = vec![leaves[2], Sha256::combine(leaves[0], leaves[1]), leaves[4]];

        let mut tree = Sha256Tree::from_leaves(&leaves);
        assert_eq!(tree.get_proof(leaves[3]).unwrap(), expected_proof);
    }
}
