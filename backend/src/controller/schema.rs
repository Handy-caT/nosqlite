use crate::{controller, schema as info, schema, schema::table};
use common::structs::hash_table::{
    scalable::ScalableHashTable, HashTable as _, MutHashTable,
    VecFunctions as _,
};

/// Controller for a single table.
/// Is used to change the table's schema and data.
#[derive(Debug, Clone)]
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
    pub fn new(name: schema::Name) -> Self {
        Schema {
            info: info::Schema::new(name),
            tables: ScalableHashTable::default(),
        }
    }

    /// Returns the name of the schema.
    /// # Returns
    /// * `&schema::Name` - The name of the schema.
    pub fn get_name(&self) -> &schema::Name {
        &self.info.name
    }

    /// Returns the names of the tables in the schema.
    /// # Returns
    /// * `Vec<String>` - The names of the tables in the schema.
    pub fn get_table_names(&mut self) -> Vec<table::Name> {
        self.tables.get_keys()
    }

    /// Returns the mutable schema information.
    /// # Returns
    /// * `&mut info::Schema` - The schema information.
    pub fn get_mut_info(&mut self) -> &mut info::Schema {
        &mut self.info
    }

    /// Adds a table to the schema.
    /// # Arguments
    /// * `controller` - The table controller to add.
    /// # Returns
    /// * `bool` - Whether the table was added.
    pub fn add_table(
        &mut self,
        controller: controller::Table<NODE_SIZE>,
    ) -> bool {
        if self.tables.contains_key(controller.get_name()) {
            false
        } else {
            self.tables
                .insert(controller.get_name().clone(), controller);
            true
        }
    }

    /// Removes a table from the schema.
    /// # Arguments
    /// * `name` - The name of the table to remove.
    /// # Returns
    /// * `Option<controller::Table<NODE_SIZE>>` - The table that was removed.
    pub fn remove_table(
        &mut self,
        name: &table::Name,
    ) -> Option<controller::Table<NODE_SIZE>> {
        self.tables.remove(name)
    }

    /// Gets a table from the schema.
    /// # Arguments
    /// * `name` - The name of the table to get.
    /// # Returns
    /// * `Option<&mut controller::Table<NODE_SIZE>>` - The table with the given
    ///   name.
    pub fn get_mut_table(
        &mut self,
        name: &table::Name,
    ) -> Option<&mut controller::Table<NODE_SIZE>> {
        self.tables.get_mut_value(name)
    }

    /// Checks if the schema has a table with the given name.
    /// # Arguments
    /// * `name` - The name of the table to check for.
    /// # Returns
    /// * `bool` - Whether the schema has a table with the given name.
    pub fn has_table(&mut self, name: &table::Name) -> bool {
        self.tables.contains_key(name)
    }
}

impl<const NODE_SIZE: u8> PartialEq for Schema<NODE_SIZE> {
    fn eq(&self, other: &Self) -> bool {
        self.info == other.info
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

    #[test]
    fn test_schema_new() {
        let schema = Schema::<4>::new("test".into());
        assert_eq!(schema.info.name, "test".into());
        assert_eq!(schema.tables.len(), 0);
    }

    #[test]
    fn test_schema_get_name() {
        let schema = Schema::<4>::new("test".into());
        assert_eq!(schema.get_name(), &"test".into());
    }

    #[test]
    fn test_schema_add_table() {
        let mut schema = Schema::<4>::new("test".into());

        let table = controller::Table::<4>::new("table".into());
        schema.add_table(table);
        assert_eq!(schema.tables.len(), 1);
    }

    #[test]
    fn test_schema_get_table_names() {
        let mut schema = Schema::<4>::new("test".into());

        let table = controller::Table::<4>::new("table".into());
        schema.add_table(table);

        let table_names = schema.get_table_names();
        assert_eq!(table_names.len(), 1);
        assert_eq!(table_names[0], "table".into());
    }

    #[test]
    fn test_schema_get_mut_table() {
        let mut schema = Schema::<4>::new("test".into());

        let table = controller::Table::<4>::new("table".into());
        schema.add_table(table);

        let table = schema.get_mut_table(&"table".into());
        assert!(table.is_some());

        let table = table.unwrap();
        assert_eq!(table.get_name(), &"table".into());

        table.add_page(1);

        let table = schema.get_mut_table(&"table".into());
        assert!(table.is_some());

        let table = table.unwrap();
        assert_eq!(table.get_pages().len(), 1);
    }
}
