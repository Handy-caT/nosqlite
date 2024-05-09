use common::structs::hash_table::hash::custom_hashable::CustomHash;

use crate::{data::id::IdGenerator, page::link_struct::PageLink};

/// Struct that generates [`Id`]s
#[derive(Debug)]
pub struct NumericIdGenerator {
    /// Vector of empty [`Id`]s that can be reused
    empty: Vec<NumericId>,

    /// Counter for the next [`Id`] to be generated
    counter: u64,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct NumericId(pub u64);

impl IdGenerator<NumericId> for NumericIdGenerator {
    fn new() -> NumericIdGenerator {
        NumericIdGenerator {
            empty: Vec::<NumericId>::new(),
            counter: 0,
        }
    }

    fn get_id(&mut self) -> NumericId {
        if self.empty.is_empty() {
            self.counter += 1;
            NumericId(self.counter)
        } else {
            self.empty.pop().unwrap()
        }
    }

    fn retrieve_id(&mut self, id: NumericId) {
        self.empty.push(id);
    }

    fn get_id_count(&self) -> u64 {
        self.counter - self.empty.len() as u64
    }
}

impl NumericId {
    pub fn new(id: u64) -> NumericId {
        NumericId(id)
    }
}

impl CustomHash for NumericId {
    fn hash(&self, hash: fn(&[u8]) -> u64) -> u64 {
        let bytes = self.0.to_be_bytes();
        hash(&bytes)
    }
}

impl CustomHash for PageLink {
    fn hash(&self, hash: fn(&[u8]) -> u64) -> u64 {
        let bytes_index = self.page_index.to_be_bytes();
        let bytes_len = self.len.to_be_bytes();
        let bytes_start = self.start.to_be_bytes();

        let mut bytes = Vec::<u8>::new();
        bytes.extend_from_slice(&bytes_index);
        bytes.extend_from_slice(&bytes_len);
        bytes.extend_from_slice(&bytes_start);

        hash(&bytes)
    }
}

#[cfg(test)]
mod tests {
    use crate::data::id::{
        numeric_id_generator::NumericIdGenerator, IdGenerator,
    };

    #[test]
    fn test_id_generator_new() {
        let id_generator = NumericIdGenerator::new();

        assert_eq!(id_generator.counter, 0);
        assert_eq!(id_generator.empty.len(), 0);
    }

    #[test]
    fn test_id_generator_get_id() {
        let mut id_generator = NumericIdGenerator::new();

        let id = id_generator.get_id();

        assert_eq!(id_generator.counter, 1);
        assert_eq!(id_generator.empty.len(), 0);
        assert_eq!(id.0, 1);
    }

    #[test]
    fn test_id_generator_retrieve_id() {
        let mut id_generator = NumericIdGenerator::new();

        let id = id_generator.get_id();

        assert_eq!(id_generator.counter, 1);
        assert_eq!(id_generator.empty.len(), 0);
        assert_eq!(id.0, 1);

        id_generator.retrieve_id(id);

        assert_eq!(id_generator.counter, 1);
        assert_eq!(id_generator.empty.len(), 1);
        assert_eq!(id_generator.empty[0].0, 1);
    }

    #[test]
    fn test_id_generator_get_id_reuse() {
        let mut id_generator = NumericIdGenerator::new();

        let id = id_generator.get_id();

        assert_eq!(id_generator.counter, 1);
        assert_eq!(id_generator.empty.len(), 0);
        assert_eq!(id.0, 1);

        id_generator.retrieve_id(id);

        assert_eq!(id_generator.counter, 1);
        assert_eq!(id_generator.empty.len(), 1);
        assert_eq!(id_generator.empty[0].0, 1);

        let id = id_generator.get_id();

        assert_eq!(id_generator.counter, 1);
        assert_eq!(id_generator.empty.len(), 0);
        assert_eq!(id.0, 1);
    }

    #[test]
    fn test_id_generator_get_id_count() {
        let mut id_generator = NumericIdGenerator::new();

        let id = id_generator.get_id();

        assert_eq!(id_generator.counter, 1);
        assert_eq!(id_generator.empty.len(), 0);
        assert_eq!(id.0, 1);

        let id_count = id_generator.get_id_count();

        assert_eq!(id_count, 1);

        id_generator.retrieve_id(id);

        assert_eq!(id_generator.counter, 1);
        assert_eq!(id_generator.empty.len(), 1);
        assert_eq!(id_generator.empty[0].0, 1);

        let id_count = id_generator.get_id_count();

        assert_eq!(id_count, 0);

        let id = id_generator.get_id();

        assert_eq!(id_generator.counter, 1);
        assert_eq!(id_generator.empty.len(), 0);
        assert_eq!(id.0, 1);

        let id_count = id_generator.get_id_count();

        assert_eq!(id_count, 1);
    }
}
