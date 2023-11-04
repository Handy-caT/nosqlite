pub trait CustomHash {
    fn hash(&self, hash: fn(&[u8]) -> u64) -> u64;
}

impl CustomHash for u64 {
    fn hash(&self, hash: fn(&[u8]) -> u64) -> u64 {
        let bytes = self.to_be_bytes();
        hash(&bytes)
    }
}
