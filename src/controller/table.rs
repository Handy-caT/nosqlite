use crate::{
    core::structs::tree::object::{tree::Tree as _, BTree},
    data::id::NumericId,
    schema,
    schema::column::primary_key,
};

/// Represents a mapper from a primary key to a unique identifier.
#[derive(Debug, Clone)]
pub struct KeyId {
    /// The unique identifier.
    pub id: NumericId,

    /// The primary key value.
    pub key: primary_key::Data,
}

impl PartialEq for KeyId {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for KeyId {}

impl PartialOrd for KeyId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for KeyId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

/// Controller for a single table.
/// Is used to change the table's schema and data.
#[derive(Debug, Default)]
pub struct TableController<const NODE_SIZE: u8> {
    /// Information about the table.
    info: schema::Table,

    /// B-Tree to store primary key indexes.
    index: BTree<KeyId, NODE_SIZE>,

    /// Vector of page indexes that store the table's data.
    table_pages: Vec<usize>,
}

impl<const NODE_SIZE: u8> TableController<NODE_SIZE> {
    /// Creates a new table controller.
    /// # Arguments
    /// * `name` - The name of the table.
    /// # Returns
    /// A new table controller.
    pub fn new(name: String) -> Self {
        TableController {
            info: schema::Table::new(name),
            index: BTree::default(),
            table_pages: Vec::new(),
        }
    }

    /// Returns the name of the table.
    /// # Returns
    /// * `&String` - The name of the table.
    pub fn get_name(&self) -> &String {
        self.info.get_name()
    }

    /// Adds a column to the table.
    /// # Arguments
    /// * `name` - The name of the column.
    /// * `column` - The column to add.
    pub fn add_column(&mut self, name: String, column: schema::Column) {
        self.info.add_column(name, column);
    }

    /// Returns the column with the given name.
    /// # Arguments
    /// * `name` - The name of the column.
    /// # Returns
    /// * `Option<&Column>` - The column with the given name.
    pub fn get_column(&mut self, name: &String) -> Option<schema::Column> {
        self.info.get_column(name)
    }

    /// Adds a row identified by [`NumericId`] to the table.
    /// # Arguments
    /// * `data` - The data to add.
    /// # Returns
    /// * `NumericId` - The index of the new row.
    pub fn add_data(&mut self, id: NumericId, key: primary_key::Data) {
        self.index.push(KeyId { id, key });
    }

    /// Adds a page to the table.
    /// # Arguments
    /// * `index` - The index of the page to add.
    pub fn add_page(&mut self, index: usize) {
        self.table_pages.push(index);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        controller::table::{KeyId, TableController},
        core::structs::tree::object::tree::Tree as _,
        data::id::NumericId,
        schema,
        schema::{column::primary_key, r#type::r#enum::StorageDataType},
    };

    #[test]
    fn test_new() {
        let name = "table".to_string();
        let table = TableController::<16>::new(name.clone());

        assert_eq!(table.get_name(), &name);
    }

    #[test]
    fn test_add_data() {
        let name = "table".to_string();
        let mut table = TableController::<16>::new(name.clone());

        table.add_data(NumericId::new(3), primary_key::Data::Integer(0.into()));

        let key_id = KeyId {
            id: NumericId::default(),
            key: primary_key::Data::Integer(0.into()),
        };
        assert!(table.index.find(&key_id).is_some());
    }

    #[test]
    fn test_get_name() {
        let name = "table".to_string();
        let table = TableController::<16>::new(name.clone());

        assert_eq!(table.get_name(), &name);
    }

    #[test]
    fn test_get_column() {
        let name = "table".to_string();
        let mut table = TableController::<16>::new(name.clone());

        let column = schema::Column::new(StorageDataType::Integer);
        table.add_column("column".to_string(), column.clone());

        assert_eq!(table.get_column(&"column".to_string()), Some(column));
    }

    #[test]
    fn test_add_column() {
        let name = "table".to_string();
        let mut table = TableController::<16>::new(name.clone());

        let column = schema::Column::new(StorageDataType::Integer);
        table.add_column("column".to_string(), column.clone());

        assert_eq!(table.get_column(&"column".to_string()), Some(column));
    }

    #[test]
    fn test_add_column_multiple() {
        let name = "table".to_string();
        let mut table = TableController::<16>::new(name.clone());

        let column = schema::Column::new(StorageDataType::Integer);
        table.add_column("column".to_string(), column.clone());

        let column = schema::Column::new(StorageDataType::Integer);
        table.add_column("column2".to_string(), column.clone());

        assert_eq!(
            table.get_column(&"column".to_string()),
            Some(schema::Column::new(StorageDataType::Integer))
        );
        assert_eq!(
            table.get_column(&"column2".to_string()),
            Some(schema::Column::new(StorageDataType::Integer))
        );
    }

    #[test]
    fn test_add_page() {
        let name = "table".to_string();
        let mut table = TableController::<16>::new(name.clone());

        table.add_page(0);

        assert_eq!(table.table_pages.len(), 1);
    }
}
