use crate::core::structs::hash_table::HashTable as _;
use crate::core::structs::hash_table::static_hash_table::StaticHashTable;
use crate::schema::r#type::r#enum::StorageDataType;

/// Type that represents a structure of the column.
pub struct Structure {
    /// Hash table that stores the column name and its data type.
    map: StaticHashTable<String, StorageDataType>
}

// impl Structure {
//     /// Creates a new instance of the structure.
//     pub fn new(size: usize) -> Self {
//         Structure {
//             map: StaticHashTable::new(size)
//         }
//     }
// }