#![allow(unused_imports)]
use crate::hash::to_hash::ToHash;

#[cfg(test)]
pub fn raw_leaves_to_hashed_leaves<T: ToHash>(leaves: &[&str]) -> Vec<T::Hash> {
    leaves.iter().map(|leaf| T::hash(leaf.as_bytes())).collect()
}

#[cfg(test)]
pub fn full_root_hash<T: ToHash>(leaves: &[T::Hash]) -> T::Hash {
    // root hash expectation
    //      [01234]
    //       /   \
    //    [0123]  4
    //    /   \    \
    //  [01] [23]   4
    //  /\    /\     \
    // 0 1   2 3      4
    T::combine(
        T::combine(
            T::combine(leaves[0], leaves[1]),
            T::combine(leaves[2], leaves[3]),
        ),
        leaves[4],
    )
}
