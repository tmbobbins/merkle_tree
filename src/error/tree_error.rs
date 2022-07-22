use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum TreeErrorKind {
    TreeEmpty,
    LeafEmpty,
    PathLeafEmpty,
    ProofEmpty,
}

#[derive(Debug)]
pub struct TreeError {
    kind: TreeErrorKind,
    message: String,
}

impl Error for TreeError {}

impl Display for TreeError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl TreeError {
    pub fn new(kind: TreeErrorKind, message: &str) -> Self {
        Self {
            kind,
            message: message.to_owned(),
        }
    }

    pub fn kind(&self) -> &TreeErrorKind {
        &self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn tree_empty() -> Self {
        Self::new(
            TreeErrorKind::TreeEmpty,
            "Tree must contain at least a single leaf",
        )
    }

    pub fn leaf_empty() -> Self {
        Self::new(
            TreeErrorKind::LeafEmpty,
            "Leaves of the tree cannot be empty",
        )
    }

    pub fn path_leaf_not_set() -> Self {
        Self::new(
            TreeErrorKind::PathLeafEmpty,
            "Current path leaf must be set to analyse path",
        )
    }

    pub fn proof_empty() -> Self {
        Self::new(TreeErrorKind::ProofEmpty, "proof is empty")
    }
}
