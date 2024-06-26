use backend::schema::database;
use common::structs::hash_table::HashTable;
use derive_more::Display;

use crate::api::{
    command::Command, facade::BackendFacade, CommandResultString,
};

/// Command to use a database.
#[derive(Debug, Clone, PartialEq)]
pub struct UseDatabase {
    /// The name of the database to use.
    pub name: database::Name,
}

impl<const NODE_SIZE: u8> Command<BackendFacade<NODE_SIZE>> for UseDatabase {
    type Ok = CommandResultString;
    type Err = ExecutionError;

    fn execute(
        self,
        backend: &mut BackendFacade<NODE_SIZE>,
    ) -> Result<Self::Ok, Self::Err> {
        if !backend.database_controllers.contains_key(&self.name) {
            return Err(ExecutionError::DatabaseNotExists(self.name));
        }
        backend.context.set_current_db(self.name.clone());

        Ok(CommandResultString {
            result: format!("Database `{}` selected", self.name),
        })
    }
}

/// Errors that can occur when executing the [`UseDatabase`] command.
#[derive(Debug, Display)]
pub enum ExecutionError {
    /// The database not exists.
    #[display(fmt = "Database `{}` not exists", _0)]
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
