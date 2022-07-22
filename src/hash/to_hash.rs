pub trait ToHash {
    type Hash: Copy + PartialEq + PartialOrd + IntoIterator + Into<Vec<u8>>;

    fn hash(value: &[u8]) -> Self::Hash;
    fn combine(left: Self::Hash, right: Self::Hash) -> Self::Hash {
        if left <= right {
            return Self::hash(&[right.into(), left.into()].concat());
        }

        Self::hash(&[left.into(), right.into()].concat())
    }
}
