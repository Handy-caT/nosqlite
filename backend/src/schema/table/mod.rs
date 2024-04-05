use common::structs::hash_table::{scalable::ScalableHashTable, HashTable};

use crate::{
    gen_name,
    schema::{
        column,
        column::{primary_key::PrimaryKey, Column},
    },
};

gen_name!();

/// Represents a database table.
#[derive(Debug, Default, Clone)]
pub struct Table {
    /// The name of the table.
    name: Name,

    /// The columns of the table.
    columns: ScalableHashTable<column::Name, Column>,

    /// The primary key of the table.
    primary_key: Option<PrimaryKey>,
}

impl Table {
    /// Creates a new [`Table`] with the given parameters.
    /// # Arguments
    /// * `name` - The name of the table.
    /// * `columns` - The columns of the table.
    /// * `primary_key` - The primary key of the table.
    /// # Returns
    /// A new [`Table`] with the given parameters.
    pub fn new(name: Name) -> Self {
        Table {
            name,
            columns: ScalableHashTable::default(),
            primary_key: None,
        }
    }

    /// Returns the name of the table.
    /// # Returns
    /// * `&Name` - The name of the table.
    pub fn get_name(&self) -> &Name {
        &self.name
    }

    /// Adds a column to the table.
    /// # Arguments
    /// * `name` - The name of the column.
    /// * `column` - The column to add.
    pub fn add_column(&mut self, name: column::Name, column: Column) {
        self.columns.insert(name, column);
    }

    /// Checks if the table has a column with the given name.
    /// # Arguments
    /// * `name` - The name of the column.
    /// # Returns
    /// * `bool` - True if the table has a column with the given name,
    ///   false otherwise.
    pub fn has_column(&mut self, name: &column::Name) -> bool {
        self.columns.get(name).is_some()
    }

    /// Checks if the table has columns with the given names.
    /// # Arguments
    /// * `name` - The names of the columns.
    /// # Returns
    /// * `bool` - True if the table has columns with the given names,
    ///   false otherwise.
    pub fn has_columns(&mut self, name: &Vec<column::Name>) -> bool {
        for column in name {
            if !self.columns.get(column).is_some() {
                return false;
            }
        }
        true
    }

    /// Returns the column with the given name.
    /// # Arguments
    /// * `name` - The name of the column.
    /// # Returns
    /// * `Option<&Column>` - The column with the given name.
    pub fn get_column(&mut self, name: &column::Name) -> Option<Column> {
        self.columns.get(name)
    }

    /// Returns the primary key of the table.
    /// # Returns
    /// * `&PrimaryKey` - The primary key of the table.
    pub fn get_primary_key(&self) -> &Option<PrimaryKey> {
        &self.primary_key
    }

    /// Sets the primary key of the table.
    /// # Arguments
    /// * `primary_key` - The primary key of the table.
    pub fn set_primary_key(&mut self, primary_key: PrimaryKey) {
        self.primary_key = Some(primary_key);
    }
}

impl PartialEq for Table {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[cfg(test)]
mod tests {
    use common::structs::hash_table::HashTable;

    use crate::schema::{
        column::{primary_key::PrimaryKey, Column},
        r#type::r#enum::StorageDataType,
        table::{Name, Table},
    };

    #[test]
    fn test_table_new() {
        let table = Table::new("table".into());
        assert_eq!(table.get_name(), &("table".into()));
        assert_eq!(table.columns.len(), 0);
        assert_eq!(table.get_primary_key(), &None);
    }

    #[test]
    fn test_table_add_column() {
        let mut table = Table::new(Name("table".to_string()));
        let column = Column::new(StorageDataType::Integer);
        table.add_column("column".to_string().into(), column.clone());
        assert_eq!(table.columns.len(), 1);
        assert_eq!(table.get_column(&("column".into())), Some(column));
    }

    #[test]
    fn test_table_get_column() {
        let mut table = Table::new("table".into());
        let column = Column::new(StorageDataType::Integer);
        table.add_column("column".into(), column.clone());
        assert_eq!(table.get_column(&("column".into())), Some(column));
    }

    #[test]
    fn test_table_has_column() {
        let mut table = Table::new("table".into());
        let column = Column::new(StorageDataType::Integer);
        table.add_column("column".into(), column.clone());
        assert!(table.has_column(&("column".into())));
        assert!(!table.has_column(&("column2".into())));
    }

    #[test]
    fn test_table_get_primary_key() {
        let table = Table::new("table".into());
        assert_eq!(table.get_primary_key(), &None);
    }

    #[test]
    fn test_table_set_primary_key() {
        let mut table = Table::new("table".into());
        let primary_key =
            PrimaryKey::new("primary_key".into(), "column".into());
        table.set_primary_key(primary_key.clone());
        assert_eq!(table.get_primary_key(), &Some(primary_key));
    }
}
