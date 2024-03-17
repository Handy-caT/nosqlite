pub mod advisors;
pub mod data_allocator;
pub mod data_storage;
pub mod id;

use crate::{
    core::structs::hash_table::{
        static_hash_table::StaticHashTable, HashTable, VecFunctions,
    },
    schema::r#type::r#enum::StorageData,
};

/// Type of data unit. It represents a column row data using a hash table.
/// Key is the name of the column and value is the data.
pub struct DataUnit(StaticHashTable<String, StorageData>);

impl DataUnit {
    /// Creates a new instance of `DataUnit`.
    /// # Arguments
    /// * `size` - The size of the hash table.
    /// # Returns
    /// * `DataUnit` - The new instance of `DataUnit`.
    pub fn new(size: usize) -> Self {
        DataUnit(StaticHashTable::new(size))
    }

    /// Inserts a new data to the hash table.
    /// # Arguments
    /// * `key` - The name of the column.
    /// * `value` - The data to insert.
    pub fn insert(&mut self, key: String, value: StorageData) {
        self.0.insert(key, value);
    }

    /// Returns the data with the given key.
    /// # Arguments
    /// * `key` - The name of the column.
    /// # Returns
    /// * `Option<&StorageData>` - The data with the given key.
    pub fn get(&mut self, key: &String) -> Option<StorageData> {
        self.0.get(key)
    }

    /// Returns the values of the data unit.
    /// # Returns
    /// * `Vec<StorageData>` - The values of the data unit.
    pub fn get_values(&mut self) -> Vec<StorageData> {
        self.0.get_values()
    }

    /// Returns length of data unit.
    /// # Returns
    /// * `usize` - The length of data unit.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::{data::DataUnit, schema::r#type::r#enum::StorageData};

    #[test]
    fn test_insert() {
        let mut data_unit = DataUnit::new(10);
        data_unit.insert("test".to_string(), StorageData::Integer(25.into()));

        assert_eq!(data_unit.len(), 1);
    }

    #[test]
    fn test_get() {
        let mut data_unit = DataUnit::new(10);
        data_unit.insert("test".to_string(), StorageData::Integer(25.into()));

        let res = data_unit.get(&"test".to_string());
        assert!(res.is_some());
        assert_eq!(res.unwrap(), StorageData::Integer(25.into()));
    }

    #[test]
    fn test_get_values() {
        let mut data_unit = DataUnit::new(10);
        data_unit.insert("test".to_string(), StorageData::Integer(25.into()));
        data_unit.insert("test2".to_string(), StorageData::Integer(25.into()));

        let res = data_unit.get_values();
        assert_eq!(res.len(), 2);
    }

    #[test]
    fn test_len() {
        let mut data_unit = DataUnit::new(10);
        assert_eq!(data_unit.len(), 0);

        data_unit.insert("test".to_string(), StorageData::Integer(25.into()));
        assert_eq!(data_unit.len(), 1);
    }
}
