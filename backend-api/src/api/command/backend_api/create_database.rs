use backend::{controller, schema::database};
use common::structs::hash_table::HashTable;

use crate::api::{
    command::{Command, ContextReceiver},
    facade::BackendFacade,
};

/// Command to create a new database.
#[derive(Debug, Clone, PartialEq)]
pub struct CreateDatabase {
    /// The name of the database to create.
    pub name: database::Name,
}

impl ContextReceiver for CreateDatabase {}

impl<const NODE_SIZE: u8> Command<BackendFacade<NODE_SIZE>> for CreateDatabase {
    type Ok = ();
    type Err = ExecutionError;

    fn execute(
        self,
        backend: &mut BackendFacade<NODE_SIZE>,
    ) -> Result<Self::Ok, Self::Err> {
        if backend.database_controllers.contains_key(&self.name) {
            return Err(ExecutionError::DatabaseAlreadyExists(self.name));
        }
        let db = controller::Database::new(self.name.clone());
        backend.database_controllers.insert(self.name, db);
        Ok(())
    }
}

/// Errors that can occur when executing the [`CreateDatabase`] command.
#[derive(Debug)]
pub enum ExecutionError {
    /// The database already exists.
    DatabaseAlreadyExists(database::Name),
}

#[cfg(test)]
mod tests {
    use backend::schema::database;
    use common::structs::hash_table::MutHashTable;

    use crate::api::command::{
        backend_api::create_database::{CreateDatabase, ExecutionError},
        gateway::{test::TestBackendFacade, GatewayError},
        Gateway,
    };

    #[test]
    fn creates_db_when_not_exists() {
        let name = database::Name::from("test");
        let mut facade = TestBackendFacade::<4>::new().build();
        let cmd = CreateDatabase { name: name.clone() };
        let result = facade.send(cmd);
        assert!(result.is_ok());

        let db = facade.database_controllers.get_mut_value(&name);
        assert!(db.is_some());
    }

    #[test]
    fn returns_error_when_db_exists() {
        let db_name = database::Name::from("test");
        let mut facade = TestBackendFacade::<4>::new()
            .with_database(db_name.clone())
            .build();
        let cmd = CreateDatabase {
            name: db_name.clone(),
        };
        let result = facade.send(cmd);
        assert!(result.is_err());

        match result {
            Err(GatewayError::CommandError(
                ExecutionError::DatabaseAlreadyExists(name),
            )) => {
                assert_eq!(name, db_name)
            }
            _ => panic!(
                "Expected `DatabaseAlreadyExists` error, found {:?}",
                result
            ),
        }
    }
}
