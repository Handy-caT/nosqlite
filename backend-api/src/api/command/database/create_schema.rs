use backend::{controller, schema};
use common::structs::hash_table::MutHashTable;

use crate::api::{command::Command, facade::BackendFacade};

/// [`Command`] to create a new schema in a database.
#[derive(Debug, Clone, PartialEq)]
pub struct CreateSchema {
    /// The name of the database where the schema will be created.
    pub database_name: Option<schema::database::Name>,

    /// The name of the schema to create.
    pub name: schema::Name,
}

impl AsRef<()> for CreateSchema {
    fn as_ref(&self) -> &() {
        &()
    }
}

impl<const NODE_SIZE: u8> Command<BackendFacade<NODE_SIZE>> for CreateSchema {
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

        if db_controller.has_schema(&self.name) {
            return Err(ExecutionError::SchemaAlreadyExists(self.name));
        }

        let schema = controller::Schema::new(self.name.clone());
        if db_controller.add_schema(schema) {
            Ok(())
        } else {
            Err(ExecutionError::SchemaAlreadyExists(self.name))
        }
    }
}

/// Errors that can occur during the execution of [`CreateSchema`].
#[derive(Debug)]
pub enum ExecutionError {
    /// The database was not provided.
    DatabaseNotProvided,

    /// Provided database does not exist.
    DatabaseNotExists(schema::database::Name),

    /// The schema already exists in the database.
    SchemaAlreadyExists(schema::Name),
}

#[cfg(test)]
mod tests {
    use backend::{schema, schema::database};
    use common::structs::hash_table::MutHashTable;

    use crate::api::command::{
        database::create_schema::{CreateSchema, ExecutionError},
        extract::DatabaseExtractionError,
        gateway::{test::TestBackendFacade, GatewayError},
        Gateway,
    };

    #[test]
    fn creates_schema_when_not_exists() {
        let database_name = database::Name::from("test");
        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .build();
        let cmd = CreateSchema {
            database_name: Some(database_name.clone()),
            name: "schema".into(),
        };
        let result = facade.send(cmd);
        assert!(result.is_ok());

        let db = facade
            .database_controllers
            .get_mut_value(&database_name)
            .unwrap();
        let schema = db.get_mut_schema(&"schema".into());
        assert!(schema.is_some());
    }

    #[test]
    fn creates_schema_with_db_in_context() {
        let database_name = database::Name::from("test");
        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .with_db_in_context(database_name.clone())
            .build();
        let cmd = CreateSchema {
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
        assert!(schema.is_some());
    }

    #[test]
    fn returns_error_when_db_not_provided() {
        let database_name = database::Name::from("test");
        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .build();
        let cmd = CreateSchema {
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
    fn returns_error_when_schema_exists() {
        let database_name = database::Name::from("test");
        let schema_name = schema::Name::from("schema");

        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .with_schema(database_name.clone(), schema_name.clone())
            .build();
        let cmd = CreateSchema {
            database_name: Some(database_name.clone()),
            name: schema_name.clone(),
        };
        let result = facade.send(cmd);
        assert!(result.is_err());

        match result {
            Err(GatewayError::CommandError(
                ExecutionError::SchemaAlreadyExists(name),
            )) => {
                assert_eq!(name, schema_name);
            }
            _ => panic!("Expected `SchemaAlreadyExists` found {:?}", result),
        }
    }

    #[test]
    fn returns_error_when_db_not_exists() {
        let database_name = database::Name::from("test");
        let schema_name = schema::Name::from("schema");

        let mut facade = TestBackendFacade::<4>::new().build();
        let cmd = CreateSchema {
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
            _ => panic!("Expected `DatabaseNotExists` found {:?}", result),
        }
    }
}
