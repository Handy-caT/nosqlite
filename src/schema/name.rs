#[macro_export]
macro_rules! gen_name {
    () => {
        use common::structs::hash_table::hash::custom_hashable::CustomHash;
        use derive_more::{From, Into};

        /// Name of the [`Column`].
        #[derive(Debug, Default, Clone, PartialEq, Hash, From, Into)]
        pub struct Name(pub String);

        impl From<&str> for Name {
            fn from(name: &str) -> Self {
                Name(name.to_string())
            }
        }

        impl CustomHash for Name {
            fn hash(&self, hash: fn(&[u8]) -> u64) -> u64 {
                self.0.hash(hash)
            }
        }
    };
}
