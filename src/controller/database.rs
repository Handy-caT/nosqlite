use common::structs::hash_table::{
    scalable::ScalableHashTable, HashTable as _, MutHashTable as _,
    VecFunctions as _,
};

use crate::{controller, schema, schema as info, schema::database};

/// Controller for a single table.
/// Is used to change the table's schema and data.
#[derive(Debug, Clone)]
pub struct Database<const NODE_SIZE: u8> {
    /// The database information.
    info: info::Database,

    /// The schemas in the database.
    schemas: ScalableHashTable<schema::Name, controller::Schema<NODE_SIZE>>,
}

impl<const NODE_SIZE: u8> Database<NODE_SIZE> {
    /// Creates a new [`Database`] with the given parameters.
    /// # Arguments
    /// * `name` - The name of the database.
    /// # Returns
    /// A new [`Database`] with the given parameters.
    pub fn new(name: database::Name) -> Self {
        Database {
            info: info::Database::new(name),
            schemas: ScalableHashTable::default(),
        }
    }

    /// Returns the name of the database.
    /// # Returns
    /// * `&database::Name` - The name of the database.
    pub fn get_name(&self) -> &database::Name {
        self.info.get_name()
    }

    /// Returns the names of the schemas in the database.
    /// # Returns
    /// * `Vec<schema::Name>` - The names of the schemas in the database.
    pub fn get_schema_names(&mut self) -> Vec<schema::Name> {
        self.schemas.get_keys()
    }

    /// Adds a schema to the database.
    /// # Arguments
    /// * `controller` - The schema controller to add.
    pub fn add_schema(&mut self, controller: controller::Schema<NODE_SIZE>) -> bool {
        if self.schemas.contains_key(controller.get_name()) {
            false
        } else {
            self.schemas
                .insert(controller.get_name().clone(), controller);
            true
        }
    }

    /// Gets a schema from the database.
    /// # Arguments
    /// * `name` - The name of the schema to get.
    /// # Returns
    /// * `Option<controller::Schema<NODE_SIZE>>` - The schema with the given
    ///   name.
    pub fn get_mut_schema(
        &mut self,
        name: &schema::Name,
    ) -> Option<&mut controller::Schema<NODE_SIZE>> {
        self.schemas.get_mut_value(name)
    }
}

impl<const NODE_SIZE: u8> PartialEq for Database<NODE_SIZE> {
    fn eq(&self, other: &Self) -> bool {
        self.info == other.info
    }
}

#[cfg(test)]
mod tests {
    use common::structs::hash_table::HashTable as _;

    use super::Database;
    use crate::{controller::Schema, schema};

    #[test]
    fn test_database_new() {
        let database = Database::<4>::new("test".into());
        assert_eq!(database.get_name(), &"test".into());
        assert_eq!(database.schemas.len(), 0)
    }

    #[test]
    fn test_database_get_name() {
        let database = Database::<4>::new("test".into());
        assert_eq!(database.get_name(), &"test".into());
    }

    #[test]
    fn test_database_get_schema_names() {
        let mut database = Database::<4>::new("test".into());
        assert_eq!(database.get_schema_names(), Vec::<schema::Name>::new());

        let schema = Schema::<4>::new("schema".into());
        database.add_schema(schema);

        assert_eq!(database.get_schema_names(), vec!["schema".into()]);
    }

    #[test]
    fn test_database_add_schema() {
        let mut database = Database::<4>::new("test".into());
        let schema = Schema::<4>::new("schema".into());
        assert!(database.add_schema(schema.clone()));

        assert_eq!(database.schemas.len(), 1);
    }

    #[test]
    fn test_database_get_schema() {
        let mut database = Database::<4>::new("test".into());
        let mut schema = Schema::<4>::new("schema".into());
        database.add_schema(schema.clone());

        assert_eq!(
            database.get_mut_schema(&"schema".into()),
            Some(&mut schema)
        );
    }
}
