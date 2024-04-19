use backend::{controller, schema};
use derive_more::AsRef;

use crate::api::{
    command::{Command, Execute},
    facade::BackendFacade,
};

/// [`Command`] to create a new schema in a database.
#[derive(Debug, AsRef, Clone, PartialEq)]
pub struct CreateSchema {
    /// The name of the database where the schema will be created.
    #[as_ref]
    pub database_name: schema::database::Name,

    /// The name of the schema to create.
    pub name: schema::Name,
}

impl Command for CreateSchema {}

impl<const NODE_SIZE: u8> Execute<CreateSchema, controller::Database<NODE_SIZE>>
    for BackendFacade<NODE_SIZE>
{
    type Ok = ();
    type Err = ExecutionError;

    fn execute(
        cmd: CreateSchema,
        db_controller: &mut controller::Database<NODE_SIZE>,
    ) -> Result<Self::Ok, Self::Err> {
        if db_controller.has_schema(&cmd.name) {
            return Err(ExecutionError::SchemaAlreadyExists(cmd.name));
        }

        let schema = controller::Schema::new(cmd.name.clone());
        if db_controller.add_schema(schema) {
            Ok(())
        } else {
            Err(ExecutionError::SchemaAlreadyExists(cmd.name))
        }
    }
}

/// Errors that can occur during the execution of [`CreateSchema`].
#[derive(Debug)]
pub enum ExecutionError {
    /// The schema already exists in the database.
    SchemaAlreadyExists(schema::Name),
}

#[cfg(test)]
mod tests {
    use backend::{schema, schema::database};
    use common::structs::hash_table::MutHashTable;

    use crate::api::command::{
        database::create_schema::{CreateSchema, ExecutionError},
        gateway::{test::TestBackendFacade, DatabaseGatewayError},
        Gateway, GatewayError,
    };

    #[test]
    fn creates_schema_when_not_exists() {
        let database_name = database::Name::from("test");
        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .build();
        let cmd = CreateSchema {
            database_name: database_name.clone(),
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
    fn returns_error_when_schema_exists() {
        let database_name = database::Name::from("test");
        let schema_name = schema::Name::from("schema");

        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .with_schema(database_name.clone(), schema_name.clone())
            .build();
        let cmd = CreateSchema {
            database_name: database_name.clone(),
            name: schema_name.clone(),
        };
        let result = facade.send(cmd);
        assert!(result.is_err());

        match result {
            Err(GatewayError::Cmd(ExecutionError::SchemaAlreadyExists(
                name,
            ))) => {
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
            database_name: database_name.clone(),
            name: schema_name.clone(),
        };
        let result = facade.send(cmd);
        assert!(result.is_err());

        match result {
            Err(GatewayError::Gateway(
                DatabaseGatewayError::DatabaseNotFound(name),
            )) => {
                assert_eq!(name, database_name);
            }
            _ => panic!("Expected `DatabaseNotFound` found {:?}", result),
        }
    }
}
