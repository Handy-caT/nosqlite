pub mod advisors;
pub mod data_allocator;
pub mod data_storage;
pub mod id;
pub mod row_type;

use crate::schema::{column, r#type::DataRow};
use common::structs::hash_table::{
    r#static::StaticHashTable, HashTable, VecFunctions,
};

use crate::schema::r#type::r#enum::StorageData;

/// Type of data unit. It represents a column row data using a hash table.
/// Key is the name of the column and value is the data.
#[derive(Debug)]
pub struct DataUnit {
    /// Hash table to map column name to data column index.
    indexes: StaticHashTable<column::Name, usize>,

    /// Data storage.
    data: Vec<DataRow>,
}

impl PartialEq for DataUnit {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl DataUnit {
    /// Creates a new instance of `DataUnit`.
    /// # Arguments
    /// * `size` - The size of the hash table.
    /// # Returns
    /// * `DataUnit` - The new instance of `DataUnit`.
    pub fn new(names: Vec<column::Name>) -> Self {
        let mut indexes = StaticHashTable::new(names.len());
        for (i, name) in names.iter().enumerate() {
            indexes.insert(name.clone(), i);
        }

        Self {
            indexes,
            data: vec![],
        }
    }

    /// Inserts a new data row to the hash table.
    /// # Arguments
    /// * `row` - The data row to insert.
    pub fn insert(&mut self, row: DataRow) {
        self.data.push(row);
    }

    /// Returns the index of the [`Column`] in a [`DataRow`].
    /// # Arguments
    /// * `key` - The name of the [`Column`].
    /// # Returns
    /// * `Option<usize>` - The index of the [`Column`] in a [`DataRow`].
    ///
    /// [`Column`]: column::Column
    pub fn get_index(&mut self, key: &column::Name) -> Option<usize> {
        self.indexes.get(key)
    }

    /// Returns the values of the data unit.
    /// # Returns
    /// * `Vec<DataRow>` - The values of the data unit.
    pub fn get_values(self) -> Vec<DataRow> {
        self.data
    }

    /// Returns length of data unit.
    /// # Returns
    /// * `usize` - The length of data unit.
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::{data::DataUnit, schema::r#type::r#enum::StorageData};

    #[test]
    fn test_insert() {
        let data_unit = DataUnit::new(vec!["test".into()]);

        assert_eq!(data_unit.len(), 0);
    }

    #[test]
    fn test_get() {
        let mut data_unit = DataUnit::new(vec!["test".into()]);

        data_unit.insert(vec![StorageData::Integer(25.into())].into());

        let res = data_unit.get_index(&"test".into());
        assert!(res.is_some());
        assert_eq!(res.unwrap(), 0);

        let res = data_unit.get_index(&"test2".into());
        assert!(res.is_none());

        let vals = data_unit.get_values();
        assert_eq!(vals.len(), 1);

        let row = vals.get(0).unwrap();
        assert_eq!(row.0.get(0).unwrap(), &StorageData::Integer(25.into()));
    }

    #[test]
    fn test_len() {
        let mut data_unit = DataUnit::new(vec!["test".into()]);
        assert_eq!(data_unit.len(), 0);

        data_unit.insert(vec![StorageData::Integer(25.into())].into());
        assert_eq!(data_unit.len(), 1);
    }
}
