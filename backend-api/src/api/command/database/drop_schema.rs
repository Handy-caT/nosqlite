use std::convert::Infallible;

use backend::{controller, schema};
use derive_more::AsRef;

use crate::api::{
    command::{Command, Execute},
    facade::BackendFacade,
};

/// [`Command`] to drop a schema from a database.
#[derive(Debug, AsRef, Clone, PartialEq)]
pub struct DropSchema {
    /// The name of the database to drop the schema from.
    #[as_ref]
    pub database_name: schema::database::Name,

    /// The name of the schema to drop.
    pub name: schema::Name,
}

impl Command for DropSchema {}

impl<const NODE_SIZE: u8> Execute<DropSchema, controller::Database<NODE_SIZE>>
    for BackendFacade<NODE_SIZE>
{
    type Ok = ();
    type Err = Infallible;

    fn execute(
        cmd: DropSchema,
        db_controller: &mut controller::Database<NODE_SIZE>,
    ) -> Result<Self::Ok, Self::Err> {
        if !db_controller.has_schema(&cmd.name) {
            return Ok(());
        }

        let _ = db_controller.remove_schema(&cmd.name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use backend::{schema, schema::database};
    use common::structs::hash_table::MutHashTable as _;

    use crate::api::command::{
        database::drop_schema::DropSchema,
        gateway::{test::TestBackendFacade, DatabaseGatewayError},
        Gateway as _, GatewayError,
    };

    #[test]
    fn drops_schema_when_exists() {
        let database_name = database::Name::from("test");
        let schema_name = schema::Name::from("schema");

        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .with_schema(database_name.clone(), schema_name.clone())
            .build();
        let cmd = DropSchema {
            database_name: database_name.clone(),
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
    fn not_errors_when_schema_not_exists() {
        let database_name = database::Name::from("test");
        let schema_name = schema::Name::from("schema");

        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .build();
        let cmd = DropSchema {
            database_name: database_name.clone(),
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
