use backend::schema::database;
use common::structs::hash_table::HashTable;

use crate::api::{
    command::{Command, ContextReceiver},
    facade::BackendFacade,
};

/// Command to use a database.
#[derive(Debug, Clone, PartialEq)]
pub struct UseDatabase {
    /// The name of the database to use.
    pub name: database::Name,
}

impl ContextReceiver for UseDatabase {}

impl<const NODE_SIZE: u8> Command<BackendFacade<NODE_SIZE>> for UseDatabase {
    type Ok = ();
    type Err = ExecutionError;

    fn execute(
        self,
        backend: &mut BackendFacade<NODE_SIZE>,
    ) -> Result<Self::Ok, Self::Err> {
        if !backend.database_controllers.contains_key(&self.name) {
            return Err(ExecutionError::DatabaseNotExists(self.name));
        }
        backend.context.set_current_db(self.name);
        Ok(())
    }
}

/// Errors that can occur when executing the [`UseDatabase`] command.
#[derive(Debug)]
pub enum ExecutionError {
    /// The database not exists.
    DatabaseNotExists(database::Name),
}

#[cfg(test)]
mod tests {
    use backend::schema::database;

    use crate::api::command::{
        gateway::{test::TestBackendFacade, GatewayError},
        Gateway,
    };

    use super::{ExecutionError, UseDatabase};

    #[test]
    fn use_db_when_exists() {
        let name = database::Name::from("test");
        let mut facade = TestBackendFacade::<4>::new()
            .with_database(name.clone())
            .build();
        let cmd = UseDatabase { name: name.clone() };
        let result = facade.send(cmd);
        assert!(result.is_ok());

        assert!(facade.context.current_db().is_some());
        assert_eq!(facade.context.current_db().unwrap(), &name)
    }

    #[test]
    fn use_db_when_not_exists() {
        let name = database::Name::from("test");
        let mut facade = TestBackendFacade::<4>::new().build();
        let cmd = UseDatabase { name: name.clone() };
        let result = facade.send(cmd);
        assert!(result.is_err());

        match result {
            Err(GatewayError::CommandError(
                ExecutionError::DatabaseNotExists(db_name),
            )) => {
                assert_eq!(name, db_name)
            }
            _ => {
                panic!("Expected `DatabaseNotExists` error, found {:?}", result)
            }
        }

        assert!(facade.context.current_db().is_none());
    }
}
