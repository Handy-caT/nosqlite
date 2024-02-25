use crate::{
    core::structs::hash_table::{
        scalable_hash_table::ScalableHashTable, HashTable,
    },
    schema::column::{primary_key::PrimaryKey, Column},
};

/// Represents a database table.
#[derive(Debug, Clone)]
pub struct Table {
    /// The name of the table.
    name: String,

    /// The columns of the table.
    columns: ScalableHashTable<String, Column>,

    /// The primary key of the table.
    primary_key: PrimaryKey,
}

impl Table {
    /// Creates a new [`Table`] with the given parameters.
    /// # Arguments
    /// * `name` - The name of the table.
    /// * `columns` - The columns of the table.
    /// * `primary_key` - The primary key of the table.
    /// # Returns
    /// A new [`Table`] with the given parameters.
    pub fn new(name: String) -> Self {
        Table {
            name,
            columns: ScalableHashTable::default(),
            primary_key: PrimaryKey::default(),
        }
    }

    /// Returns the name of the table.
    /// # Returns
    /// * `&String` - The name of the table.
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Adds a column to the table.
    /// # Arguments
    /// * `name` - The name of the column.
    /// * `column` - The column to add.
    pub fn add_column(&mut self, name: String, column: Column) {
        self.columns.insert(name, column);
    }

    /// Returns the column with the given name.
    /// # Arguments
    /// * `name` - The name of the column.
    /// # Returns
    /// * `Option<&Column>` - The column with the given name.
    pub fn get_column(&mut self, name: &String) -> Option<Column> {
        self.columns.get(name)
    }

    /// Returns the primary key of the table.
    /// # Returns
    /// * `&PrimaryKey` - The primary key of the table.
    pub fn get_primary_key(&self) -> &PrimaryKey {
        &self.primary_key
    }

    /// Sets the primary key of the table.
    /// # Arguments
    /// * `primary_key` - The primary key of the table.
    pub fn set_primary_key(&mut self, primary_key: PrimaryKey) {
        self.primary_key = primary_key;
    }
}

impl PartialEq for Table {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::structs::hash_table::HashTable,
        schema::{
            column::{primary_key::PrimaryKey, Column},
            table::Table,
        },
    };

    #[test]
    fn test_table_new() {
        let table = Table::new("table".to_string());
        assert_eq!(table.get_name(), "table");
        assert_eq!(table.columns.len(), 0);
        assert_eq!(table.get_primary_key().get_name(), "");
        assert_eq!(table.get_primary_key().get_columns().len(), 0);
    }

    #[test]
    fn test_table_add_column() {
        let mut table = Table::new("table".to_string());
        let column = Column::default();
        table.add_column("column".to_string(), column.clone());
        assert_eq!(table.columns.len(), 1);
        assert_eq!(table.get_column(&"column".to_string()), Some(column));
    }

    #[test]
    fn test_table_get_column() {
        let mut table = Table::new("table".to_string());
        let column = Column::default();
        table.add_column("column".to_string(), column.clone());
        assert_eq!(table.get_column(&"column".to_string()), Some(column));
    }

    #[test]
    fn test_table_get_primary_key() {
        let table = Table::new("table".to_string());
        assert_eq!(table.get_primary_key().get_name(), "");
        assert_eq!(table.get_primary_key().get_columns().len(), 0);
    }

    #[test]
    fn test_table_set_primary_key() {
        let mut table = Table::new("table".to_string());
        let primary_key = PrimaryKey::new(
            "primary_key".to_string(),
            vec!["column".to_string()],
        );
        table.set_primary_key(primary_key.clone());
        assert_eq!(table.get_primary_key(), &primary_key);
    }
}
