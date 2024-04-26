use backend::schema;
use common::structs::hash_table::MutHashTable;

use crate::api::{command::Command, facade::BackendFacade};

/// [`Command`] to drop a schema from a database.
#[derive(Debug, Clone, PartialEq)]
pub struct DropSchema {
    /// The name of the database to drop the schema from.
    pub database_name: Option<schema::database::Name>,

    /// The name of the schema to drop.
    pub name: schema::Name,
}

impl AsRef<()> for DropSchema {
    fn as_ref(&self) -> &() {
        &()
    }
}

impl<const NODE_SIZE: u8> Command<BackendFacade<NODE_SIZE>> for DropSchema {
    type Ok = ();
    type Err = ExecutionError;

    fn execute(
        self,
        backend: &mut BackendFacade<NODE_SIZE>,
    ) -> Result<Self::Ok, Self::Err> {
        let database_name = self
            .database_name
            .as_ref()
            .or(backend.context.current_db())
            .ok_or(ExecutionError::DatabaseNotProvided)?;

        let db_controller = backend
            .database_controllers
            .get_mut_value(database_name)
            .ok_or(ExecutionError::DatabaseNotExists(database_name.clone()))?;

        if !db_controller.has_schema(&self.name) {
            return Ok(());
        }

        let _ = db_controller.remove_schema(&self.name);
        Ok(())
    }
}

/// Errors that can occur during the execution of [`DropSchema`].
#[derive(Debug)]
pub enum ExecutionError {
    /// The database was not provided.
    DatabaseNotProvided,

    /// Provided database does not exist.
    DatabaseNotExists(schema::database::Name),
}

#[cfg(test)]
mod tests {
    use backend::{schema, schema::database};
    use common::structs::hash_table::MutHashTable as _;

    use crate::api::command::{
        gateway::{test::TestBackendFacade, GatewayError},
        Gateway as _,
    };

    use super::{DropSchema, ExecutionError};

    #[test]
    fn drops_schema_when_exists() {
        let database_name = database::Name::from("test");
        let schema_name = schema::Name::from("schema");

        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .with_schema(database_name.clone(), schema_name.clone())
            .build();
        let cmd = DropSchema {
            database_name: Some(database_name.clone()),
            name: schema_name.clone(),
        };
        let result = facade.send(cmd);
        assert!(result.is_ok());

        let db = facade
            .database_controllers
            .get_mut_value(&database_name)
            .unwrap();
        let schema = db.get_mut_schema(&schema_name);
        assert!(schema.is_none());
    }

    #[test]
    fn drop_schema_with_db_in_context() {
        let database_name = database::Name::from("test");
        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .with_db_in_context(database_name.clone())
            .build();
        let cmd = DropSchema {
            database_name: None,
            name: "schema".into(),
        };
        let result = facade.send(cmd);
        assert!(result.is_ok());

        let db = facade
            .database_controllers
            .get_mut_value(&database_name)
            .unwrap();
        let schema = db.get_mut_schema(&"schema".into());
        assert!(schema.is_none());
    }

    #[test]
    fn returns_error_when_db_not_provided() {
        let database_name = database::Name::from("test");
        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .build();
        let cmd = DropSchema {
            database_name: None,
            name: "schema".into(),
        };
        let result = facade.send(cmd);
        assert!(result.is_err());

        match result {
            Err(GatewayError::CommandError(
                ExecutionError::DatabaseNotProvided,
            )) => {}
            _ => panic!("Expected `DatabaseNotProvided` found {:?}", result),
        }
    }

    #[test]
    fn not_errors_when_schema_not_exists() {
        let database_name = database::Name::from("test");
        let schema_name = schema::Name::from("schema");

        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .build();
        let cmd = DropSchema {
            database_name: Some(database_name.clone()),
            name: schema_name.clone(),
        };
        let result = facade.send(cmd);
        assert!(result.is_ok());
    }

    #[test]
    fn returns_error_when_db_not_exists() {
        let database_name = database::Name::from("test");
        let schema_name = schema::Name::from("schema");

        let mut facade = TestBackendFacade::<4>::new().build();
        let cmd = DropSchema {
            database_name: Some(database_name.clone()),
            name: schema_name.clone(),
        };
        let result = facade.send(cmd);
        assert!(result.is_err());

        match result {
            Err(GatewayError::CommandError(
                ExecutionError::DatabaseNotExists(name),
            )) => {
                assert_eq!(name, database_name);
            }
            _ => panic!("Expected `DatabaseNotFound` found {:?}", result),
        }
    }
}
