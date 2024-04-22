use derive_more::Display;

use crate::gen_name;

/// Represents a database info.
#[derive(Debug, Clone, Display, PartialEq, Hash)]
pub struct Database {
    /// The name of the database.
    name: Name,
}

gen_name!();

impl Database {
    /// Creates a new [`Database`] with the given parameters.
    /// # Arguments
    /// * `name` - The name of the database.
    /// # Returns
    /// A new [`Database`] with the given parameters.
    pub fn new(name: Name) -> Self {
        Database { name }
    }

    /// Returns the name of the database.
    /// # Returns
    /// * `&Name` - The name of the database.
    pub fn get_name(&self) -> &Name {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use crate::schema::database::Database;

    #[test]
    fn test_database_new() {
        let database = Database::new("test".into());
        assert_eq!(database.name, "test".into());
    }

    #[test]
    fn test_database_get_name() {
        let database = Database::new("test".into());
        assert_eq!(database.get_name(), &"test".into());
    }
}
