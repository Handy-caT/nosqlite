pub mod column;
pub mod name;
pub mod table;
pub mod r#type;

use common::structs::hash_table::{
    scalable::ScalableHashTable, HashTable as _,
};

pub use column::Column;
pub use table::Table;

/// Represents a database schema.
#[derive(Debug)]
pub struct Schema {
    /// The name of the schema.
    name: String,
}

impl Schema {
    /// Creates a new [`Schema`] with the given parameters.
    /// # Arguments
    /// * `name` - The name of the schema.
    /// # Returns
    /// A new [`Schema`] with the given parameters.
    pub fn new(name: String) -> Self {
        Schema { name }
    }

    /// Returns the name of the schema.
    /// # Returns
    /// * `&String` - The name of the schema.
    pub fn get_name(&self) -> &String {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use common::structs::hash_table::HashTable;

    use crate::schema::{table::Table, Schema};

    #[test]
    fn test_schema_new() {
        let schema = Schema::new("test".to_string());
        assert_eq!(schema.name, "test");
    }
}
