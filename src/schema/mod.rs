pub mod column;
pub mod table;
pub mod r#type;

use common::structs::hash_table::{
    scalable::ScalableHashTable, HashTable as _,
};

pub use column::Column;
pub use table::Table;

/// Represents a database schema.
pub struct Schema {
    /// The name of the schema.
    name: String,

    /// The tables of the schema.
    tables: ScalableHashTable<String, Table>,
}

impl Schema {
    /// Creates a new [`Schema`] with the given parameters.
    /// # Arguments
    /// * `name` - The name of the schema.
    /// # Returns
    /// A new [`Schema`] with the given parameters.
    pub fn new(name: String) -> Self {
        Schema {
            name,
            tables: ScalableHashTable::default(),
        }
    }

    /// Returns the name of the schema.
    /// # Returns
    /// * `&String` - The name of the schema.
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Adds a table to the schema.
    /// # Arguments
    /// * `name` - The name of the table.
    /// * `table` - The table to add.
    pub fn add_table(&mut self, name: String, table: Table) {
        self.tables.insert(name, table);
    }

    /// Returns the table with the given name.
    /// # Arguments
    /// * `name` - The name of the table.
    /// # Returns
    /// * `Option<&Table>` - The table with the given name.
    pub fn get_table(&mut self, name: &String) -> Option<Table> {
        self.tables.get(name)
    }

    /// Removes the table with the given name.
    /// # Arguments
    /// * `name` - The name of the table.
    /// # Returns
    /// * `Option<Table>` - The table with the given name.
    pub fn remove_table(&mut self, name: &String) -> Option<Table> {
        self.tables.remove(name)
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
        assert_eq!(schema.tables.len(), 0);
    }

    #[test]
    fn test_schema_add_table() {
        let mut schema = Schema::new("test".to_string());
        let table = Table::new("test_table".to_string());
        schema.add_table("test_table".to_string(), table);
        assert_eq!(schema.tables.len(), 1);
    }

    #[test]
    fn test_schema_get_table() {
        let mut schema = Schema::new("test".to_string());
        let table = Table::new("test_table".to_string());
        schema.add_table("test_table".to_string(), table);
        let table = schema.get_table(&"test_table".to_string());
        assert!(table.is_some());
        assert_eq!(table.unwrap().get_name(), "test_table");
    }

    #[test]
    fn test_schema_remove_table() {
        let mut schema = Schema::new("test".to_string());
        let table = Table::new("test_table".to_string());
        schema.add_table("test_table".to_string(), table);
        let table = schema.remove_table(&"test_table".to_string());
        assert!(table.is_some());
        assert_eq!(schema.tables.len(), 0);
    }
}
