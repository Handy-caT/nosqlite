#[macro_export]
macro_rules! gen_name {
    () => {
        /// Name of the [`Column`].
        #[derive(
            Debug,
            Default,
            Clone,
            derive_more::Display,
            PartialEq,
            Hash,
            derive_more::From,
            derive_more::Into,
        )]
        pub struct Name(pub String);

        impl From<&str> for Name {
            fn from(name: &str) -> Self {
                Name(name.to_string())
            }
        }

        impl common::structs::hash_table::hash::custom_hashable::CustomHash
            for Name
        {
            fn hash(&self, hash: fn(&[u8]) -> u64) -> u64 {
                self.0.hash(hash)
            }
        }
    };
}
