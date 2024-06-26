pub mod column;
pub mod database;
pub mod name;
pub mod table;
pub mod r#type;

use derive_more::Display;

use crate::gen_name;

pub use column::Column;
pub use database::Database;
pub use table::Table;

/// Represents a database schema.
#[derive(Debug, Display, PartialEq, Clone)]
pub struct Schema {
    /// The name of the schema.
    pub name: Name,
}

gen_name!();

impl Schema {
    /// Creates a new [`Schema`] with the given parameters.
    /// # Arguments
    /// * `name` - The name of the schema.
    /// # Returns
    /// A new [`Schema`] with the given parameters.
    pub fn new(name: Name) -> Self {
        Schema { name }
    }
}

#[cfg(test)]
mod tests {
    use common::structs::hash_table::HashTable;

    use crate::schema::{table::Table, Schema};

    #[test]
    fn test_schema_new() {
        let schema = Schema::new("test".into());
        assert_eq!(schema.name, "test".into());
    }
}
