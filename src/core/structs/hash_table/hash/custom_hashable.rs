pub trait CustomHash {
    fn hash(&self, hash: fn(&[u8]) -> u64) -> u64;
}
macro_rules! impl_custom_hash {
    ($($t:ty),*) => {
        $(
            impl CustomHash for $t {
                fn hash(&self, hash: fn(&[u8]) -> u64) -> u64 {
                    let bytes = self.to_be_bytes();
                    hash(&bytes)
                }
            }
        )*
    };
}

impl_custom_hash!(usize, u64);
    
impl CustomHash for String {
    fn hash(&self, hash: fn(&[u8]) -> u64) -> u64 {
        hash(self.as_bytes())
    }
}