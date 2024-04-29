use backend::{controller, schema, schema::database};
use std::convert::Infallible;

use crate::{
    api::{
        command::{
            database::{CreateSchema, ProvideError},
            Command, ContextReceiver, OptionalBy,
        },
        CommandResultString,
    },
    Context,
};

/// [`Command`] to drop a schema from a database.
#[derive(Debug, Clone, PartialEq)]
pub struct DropSchema {
    /// The name of the database to drop the schema from.
    pub database_name: Option<database::Name>,

    /// The name of the schema to drop.
    pub name: schema::Name,
}

impl OptionalBy<database::Name> for DropSchema {
    type Err = ProvideError;

    fn by(&self) -> Result<database::Name, Self::Err> {
        self.database_name
            .clone()
            .ok_or(ProvideError::DatabaseNotProvided)
    }
}

impl ContextReceiver for DropSchema {
    fn receive(&mut self, context: &Context) {
        if self.database_name.is_none() {
            self.database_name = context.current_db().cloned();
        }
    }
}

impl<const NODE_SIZE: u8> Command<controller::Database<NODE_SIZE>>
    for DropSchema
{
    type Ok = CommandResultString;
    type Err = ExecutionError;

    fn execute(
        self,
        db_controller: &mut controller::Database<NODE_SIZE>,
    ) -> Result<Self::Ok, Self::Err> {
        if !db_controller.has_schema(&self.name) {
            return Ok(CommandResultString::default());
        }

        let _ = db_controller.remove_schema(&self.name);

        Ok(CommandResultString {
            result: format!(
                "Schema `{}`.`{}` dropped",
                self.database_name.expect("exists"),
                self.name
            ),
        })
    }
}

/// Errors that can occur during the execution of [`DropSchema`].
pub type ExecutionError = Infallible;

#[cfg(test)]
mod tests {
    use backend::{schema, schema::database};
    use common::structs::hash_table::MutHashTable as _;

    use crate::api::command::{
        extract::DatabaseExtractionError,
        gateway::{test::TestBackendFacade, GatewayError},
        Gateway as _,
    };

    use super::DropSchema;

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
            Err(GatewayError::ByNotProvided) => {}
            _ => panic!("Expected `ByNotProvided` found {:?}", result),
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
            Err(GatewayError::ExtractionError(
                DatabaseExtractionError::DatabaseNotFound(name),
            )) => {
                assert_eq!(name, database_name);
            }
            _ => panic!("Expected `DatabaseNotFound` found {:?}", result),
        }
    }
}
