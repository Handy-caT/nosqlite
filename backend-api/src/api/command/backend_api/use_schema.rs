use backend::{controller, schema, schema::database};
use common::structs::hash_table::{HashTable, MutHashTable};
use derive_more::AsRef;

use crate::api::{
    command::{Command, Execute},
    facade::BackendFacade,
};

/// [`Command`] to use a schema in a database.
#[derive(Debug, AsRef, Clone, PartialEq)]
pub struct UseSchema {
    /// The name of the database where the schema will be used.
    #[as_ref]
    pub database_name: database::Name,

    /// The name of the schema to use.
    pub name: schema::Name,
}

impl Command for UseSchema {}

impl<const NODE_SIZE: u8> Execute<UseSchema, Self>
    for BackendFacade<NODE_SIZE>
{
    type Ok = ();
    type Err = ExecutionError;

    fn execute(
        cmd: UseSchema,
        backend: &mut Self,
    ) -> Result<Self::Ok, Self::Err> {
        if !backend
            .database_controllers
            .contains_key(&cmd.database_name)
        {
            return Err(ExecutionError::DatabaseNotExists(cmd.database_name));
        }

        let db_controller = backend
            .database_controllers
            .get_mut_value(&cmd.database_name)
            .expect("exitst because of the check above");
        if !db_controller.has_schema(&cmd.name) {
            return Err(ExecutionError::SchemaNotExists(cmd.name));
        }

        backend.context.set_current_schema(cmd.name);
        Ok(())
    }
}

/// Errors that can occur during the execution of [`UseSchema`].
#[derive(Debug)]
pub enum ExecutionError {
    /// The schema not exists in the database.
    DatabaseNotExists(database::Name),

    /// The schema not exists in the database.
    SchemaNotExists(schema::Name),
}

#[cfg(test)]
mod tests {
    use backend::{schema, schema::database};

    use crate::api::command::{
        gateway::test::TestBackendFacade, Gateway, GatewayError,
    };

    use super::{ExecutionError, UseSchema};

    #[test]
    fn use_schema_when_exists() {
        let database_name = database::Name::from("test");
        let schema_name = schema::Name::from("schema");

        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .with_schema(database_name.clone(), schema_name.clone())
            .build();
        let cmd = UseSchema {
            database_name: database_name.clone(),
            name: schema_name.clone(),
        };
        let result = facade.send(cmd);
        assert!(result.is_ok());

        assert!(facade.context.current_schema().is_some());
        assert_eq!(facade.context.current_schema().unwrap(), &schema_name)
    }

    #[test]
    fn returns_error_when_schema_not_exists() {
        let database_name = database::Name::from("test");
        let schema_name = schema::Name::from("schema");

        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .build();
        let cmd = UseSchema {
            database_name: database_name.clone(),
            name: schema_name.clone(),
        };
        let result = facade.send(cmd);
        assert!(result.is_err());

        match result {
            Err(GatewayError::Cmd(ExecutionError::SchemaNotExists(name))) => {
                assert_eq!(name, schema_name);
            }
            _ => panic!("Expected `SchemaNotExists` found {:?}", result),
        }
    }

    #[test]
    fn returns_error_when_db_not_exists() {
        let database_name = database::Name::from("test");
        let schema_name = schema::Name::from("schema");

        let mut facade = TestBackendFacade::<4>::new().build();
        let cmd = UseSchema {
            database_name: database_name.clone(),
            name: schema_name.clone(),
        };
        let result = facade.send(cmd);
        assert!(result.is_err());

        match result {
            Err(GatewayError::Cmd(ExecutionError::DatabaseNotExists(name))) => {
                assert_eq!(name, database_name);
            }
            _ => panic!("Expected `DatabaseNotExists` found {:?}", result),
        }
    }
}
