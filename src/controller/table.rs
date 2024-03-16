use crate::{
    core::structs::tree::object::{tree::Tree as _, BTree},
    data::id::NumericId,
    schema,
};

/// Controller for a single table.
/// Is used to change the table's schema and data.
#[derive(Debug, Default)]
pub struct TableController<const NODE_SIZE: u8> {
    /// Information about the table.
    info: schema::Table,

    /// B-Tree to store primary key indexes.
    index: BTree<NumericId, NODE_SIZE>,
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
    pub fn add_data(&mut self, data: NumericId) {
        self.index.push(data);
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use crate::controller::table::TableController;
    use crate::core::structs::tree::object::tree::Tree;
    use crate::data::id::NumericId;
    use crate::schema;
    use crate::schema::r#type::r#enum::StorageDataType;

    #[test]
    fn test_new() {
        let name = "table".to_string();
        let table = TableController::<16>::new(name.clone());

        assert_eq!(table.get_name(), &name);
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
    fn test_add_data() {
        let name = "table".to_string();
        let mut table = TableController::<16>::new(name.clone());

        table.add_data(NumericId::new(0));

        assert!(table.index.find(&NumericId::new(0)).is_some());
    }
}