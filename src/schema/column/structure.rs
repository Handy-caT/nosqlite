use crate::core::structs::hash_table::{HashTable as _, VecFunctions};
use crate::core::structs::hash_table::static_hash_table::StaticHashTable;
use crate::schema::r#type::r#enum::{StorageData, StorageDataType};

/// Type that represents a structure of the column.
pub struct Structure {
    /// Hash table that stores the column name and its data type.
    map: StaticHashTable<String, StorageDataType>
}

impl Structure {
    /// Creates a new instance of the structure.
    pub fn new(size: usize) -> Self {
        Structure {
            map: StaticHashTable::new(size)
        }
    }
    
    /// Inserts a new column into the structure.
    pub fn insert(&mut self, name: String, data_type: StorageDataType) {
        self.map.insert(name, data_type);
    }
    
    /// Gets the data type of the column.
    pub fn get(&mut self, name: &String) -> Option<StorageDataType> {
        self.map.get(name)
    }
    
    /// Removes the column from the structure.
    pub fn remove(&mut self, name: &String) -> Option<StorageDataType> {
        self.map.remove(name)
    }
    
    /// Gets the data types of the column as a string divided by `|` symbol.
    pub fn get_types_string(&mut self) -> String {
        let types = self.map.get_values();
        let mut types_string = String::new();
        
        for t in types {
            types_string.push_str(&format!("{:?}|", t));
        }
        types_string.remove(types_string.len() - 1);
        
        types_string
    }
}

#[cfg(test)]
mod tests {
    use crate::core::structs::hash_table::{HashTable, VecFunctions};
    use crate::schema::column::structure::Structure;
    use crate::schema::r#type::r#enum::StorageDataType;

    #[test]
    fn test_structure_insert() {
        let mut structure = Structure::new(10);
        structure.insert("name".to_string(), StorageDataType::VarChar);
        structure.insert("age".to_string(), StorageDataType::UInteger);
        structure.insert("height".to_string(), StorageDataType::Float);
        
        let values = structure.map.get_values();
        assert_eq!(values.len(), 3);
        assert!(values.contains(&StorageDataType::VarChar));
        assert!(values.contains(&StorageDataType::UInteger));
        assert!(values.contains(&StorageDataType::Float));
        
        let keys = structure.map.get_keys();
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&"name".to_string()));
        assert!(keys.contains(&"age".to_string()));
        assert!(keys.contains(&"height".to_string()));
    }
    
    #[test]
    fn test_structure_get() {
        let mut structure = Structure::new(10);
        structure.insert("name".to_string(), StorageDataType::VarChar);
        structure.insert("age".to_string(), StorageDataType::UInteger);
        structure.insert("height".to_string(), StorageDataType::Float);
        
        assert_eq!(structure.get(&"name".to_string()), Some(StorageDataType::VarChar));
        assert_eq!(structure.get(&"age".to_string()), Some(StorageDataType::UInteger));
        assert_eq!(structure.get(&"height".to_string()), Some(StorageDataType::Float));
    }
    
    #[test]
    fn test_structure_remove() {
        let mut structure = Structure::new(10);
        structure.insert("name".to_string(), StorageDataType::VarChar);
        structure.insert("age".to_string(), StorageDataType::UInteger);
        structure.insert("height".to_string(), StorageDataType::Float);
        
        assert_eq!(structure.remove(&"name".to_string()), Some(StorageDataType::VarChar));
        assert_eq!(structure.remove(&"age".to_string()), Some(StorageDataType::UInteger));
        assert_eq!(structure.remove(&"height".to_string()), Some(StorageDataType::Float));
        
        assert_eq!(structure.map.len(), 0);
    }
    
    #[test]
    fn test_structure_get_types_string() {
        let mut structure = Structure::new(10);
        structure.insert("name".to_string(), StorageDataType::VarChar);
        structure.insert("age".to_string(), StorageDataType::UInteger);
        structure.insert("height".to_string(), StorageDataType::Float);
        
        let str = structure.get_types_string();
        assert!(str.contains("VarChar"));
        assert!(str.contains("UInteger"));
        assert!(str.contains("Float"));
    }
}