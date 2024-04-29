use backend::{controller, schema, schema::database};
use derive_more::Display;

use crate::{
    api::{
        command::{Command, ContextReceiver, OptionalBy},
        CommandResultString,
    },
    Context,
};

/// [`Command`] to create a new schema in a database.
#[derive(Debug, Clone, PartialEq)]
pub struct CreateSchema {
    /// The name of the database where the schema will be created.
    pub database_name: Option<database::Name>,

    /// The name of the schema to create.
    pub name: schema::Name,
}

impl OptionalBy<database::Name> for CreateSchema {
    fn by(&self) -> Option<database::Name> {
        self.database_name.clone()
    }
}

impl ContextReceiver for CreateSchema {
    fn receive(&mut self, context: &Context) {
        if self.database_name.is_none() {
            self.database_name = context.current_db().cloned();
        }
    }
}

impl<const NODE_SIZE: u8> Command<controller::Database<NODE_SIZE>>
    for CreateSchema
{
    type Ok = CommandResultString;
    type Err = ExecutionError;

    fn execute(
        self,
        db_controller: &mut controller::Database<NODE_SIZE>,
    ) -> Result<Self::Ok, Self::Err> {
        let database_name = self
            .database_name
            .clone()
            .expect("database_name is provided");

        if db_controller.has_schema(&self.name) {
            return Err(ExecutionError::SchemaAlreadyExists(
                database_name,
                self.name,
            ));
        }

        let schema = controller::Schema::new(self.name.clone());
        if db_controller.add_schema(schema) {
            Ok(CommandResultString {
                result: format!(
                    "Schema `{}`.`{}` created",
                    database_name, self.name
                ),
            })
        } else {
            Err(ExecutionError::SchemaAlreadyExists(
                database_name,
                self.name,
            ))
        }
    }
}

/// Errors that can occur during the execution of [`CreateSchema`].
#[derive(Debug, Display)]
pub enum ExecutionError {
    /// The schema already exists in the database.
    #[display(fmt = "Schema `{}`.`{}` already exists", _0, _1)]
    SchemaAlreadyExists(database::Name, schema::Name),
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
            Err(GatewayError::ByNotProvided) => {}
            _ => panic!("Expected `ByNotProvided` found {:?}", result),
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
                ExecutionError::SchemaAlreadyExists(db_name, name),
            )) => {
                assert_eq!(db_name, database_name);
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
            Err(GatewayError::ExtractionError(
                DatabaseExtractionError::DatabaseNotFound(name),
            )) => {
                assert_eq!(name, database_name);
            }
            _ => panic!("Expected `DatabaseNotFound` found {:?}", result),
        }
    }
}
