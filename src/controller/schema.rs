use crate::{controller, schema as info, schema::table};
use common::structs::hash_table::{
    scalable::ScalableHashTable, HashTable as _, MutHashTable,
    VecFunctions as _,
};

/// Controller for a single table.
/// Is used to change the table's schema and data.
#[derive(Debug)]
pub struct Schema<const NODE_SIZE: u8> {
    /// The schema information.
    info: info::Schema,

    /// The tables in the schema.
    tables: ScalableHashTable<table::Name, controller::Table<NODE_SIZE>>,
}

impl<const NODE_SIZE: u8> Schema<NODE_SIZE> {
    /// Creates a new [`Schema`] with the given parameters.
    /// # Arguments
    /// * `name` - The name of the schema.
    /// # Returns
    /// A new [`Schema`] with the given parameters.
    pub fn new(name: String) -> Self {
        Schema {
            info: info::Schema::new(name),
            tables: ScalableHashTable::default(),
        }
    }

    /// Returns the name of the schema.
    /// # Returns
    /// * `&String` - The name of the schema.
    pub fn get_name(&self) -> &String {
        self.info.get_name()
    }

    /// Returns the names of the tables in the schema.
    /// # Returns
    /// * `Vec<String>` - The names of the tables in the schema.
    pub fn get_table_names(&mut self) -> Vec<table::Name> {
        self.tables.get_keys()
    }

    /// Adds a table to the schema.
    /// # Arguments
    /// * `controller` - The table controller to add.
    pub fn add_table(&mut self, controller: controller::Table<NODE_SIZE>) {
        self.tables
            .insert(controller.get_name().clone(), controller);
    }

    /// Gets a table from the schema.
    /// # Arguments
    /// * `name` - The name of the table to get.
    /// # Returns
    /// * `Option<controller::Table<NODE_SIZE>>` - The table with the given
    ///   name.
    pub fn get_table(
        &mut self,
        name: &table::Name,
    ) -> Option<&mut controller::Table<NODE_SIZE>> {
        self.tables.get_mut_value(name)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use common::structs::hash_table::HashTable as _;

    use crate::{
        controller,
        controller::schema::Schema,
        data::{data_storage::DataStorage, id},
        page::page_controller::PageController,
    };

    /// Creates a new instance of `DataStorage`.
    fn data_storage_factory() -> DataStorage {
        let mut controller = PageController::new();
        controller.add_page();
        let controller = Arc::new(Mutex::new(controller));
        let registry = Arc::new(Mutex::new(id::Registry::new()));

        DataStorage::new(controller, registry)
    }

    #[test]
    fn test_schema_new() {
        let schema = Schema::<4>::new("test".to_string());
        assert_eq!(schema.info.get_name(), "test");
        assert_eq!(schema.tables.len(), 0);
    }

    #[test]
    fn test_schema_get_name() {
        let schema = Schema::<4>::new("test".to_string());
        assert_eq!(schema.get_name(), "test");
    }

    #[test]
    fn test_schema_add_table() {
        let mut schema = Schema::<4>::new("test".to_string());

        let data_storage = data_storage_factory();
        let table = controller::Table::<4>::new("table".into(), data_storage);
        schema.add_table(table);
        assert_eq!(schema.tables.len(), 1);
    }

    #[test]
    fn test_schema_get_table_names() {
        let mut schema = Schema::<4>::new("test".to_string());

        let data_storage = data_storage_factory();
        let table = controller::Table::<4>::new("table".into(), data_storage);
        schema.add_table(table);

        let table_names = schema.get_table_names();
        assert_eq!(table_names.len(), 1);
        assert_eq!(table_names[0], "table".into());
    }

    #[test]
    fn test_schema_get_table() {
        let mut schema = Schema::<4>::new("test".to_string());

        let data_storage = data_storage_factory();
        let table = controller::Table::<4>::new("table".into(), data_storage);
        schema.add_table(table);

        let table = schema.get_table(&"table".into());
        assert!(table.is_some());

        let table = table.unwrap();
        assert_eq!(table.get_name(), &"table".into());

        table.add_page(1);

        let table = schema.get_table(&"table".into());
        assert!(table.is_some());

        let table = table.unwrap();
        assert_eq!(table.get_pages().len(), 1);
    }
}
