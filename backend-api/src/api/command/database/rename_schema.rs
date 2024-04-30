use backend::{controller, schema, schema::database};
use derive_more::Display;

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

/// [`Command`] which is used to rename a [`controller::Schema`].
#[derive(Debug, Clone, PartialEq)]
pub struct RenameSchema {
    /// The name of the database where the schema is located.
    pub database_name: Option<database::Name>,

    /// The old name of the schema.
    pub old_name: schema::Name,

    /// The new name of the schema.
    pub new_name: schema::Name,
}

impl OptionalBy<database::Name> for RenameSchema {
    type Err = ProvideError;

    fn by(&self) -> Result<database::Name, Self::Err> {
        self.database_name
            .clone()
            .ok_or(ProvideError::DatabaseNotProvided)
    }
}

impl ContextReceiver for RenameSchema {
    fn receive(&mut self, context: &Context) {
        if self.database_name.is_none() {
            self.database_name = context.current_db().cloned();
        }
    }
}

impl<const NODE_SIZE: u8> Command<controller::Database<NODE_SIZE>>
    for RenameSchema
{
    type Ok = CommandResultString;
    type Err = ExecutionError;

    fn execute(
        self,
        db_controller: &mut controller::Database<NODE_SIZE>,
    ) -> Result<Self::Ok, Self::Err> {
        let database_name =
            self.database_name.clone().expect("database_name is set");

        if !db_controller.has_schema(&self.old_name) {
            return Err(ExecutionError::SchemaNotFound(
                database_name,
                self.old_name,
            ));
        }
        if db_controller.has_schema(&self.new_name) {
            return Err(ExecutionError::SchemaAlreadyExists(
                database_name,
                self.new_name,
            ));
        }

        let schema = db_controller.remove_schema(&self.old_name);
        if let Some(mut schema) = schema {
            let info = schema.get_mut_info();
            info.name = self.new_name.clone();
            db_controller.add_schema(schema);

            Ok(CommandResultString {
                result: format!(
                    "Schema `{}`.`{}` renamed to `{}`.`{}`",
                    database_name.clone(),
                    self.old_name,
                    database_name,
                    self.new_name.clone()
                ),
            })
        } else {
            Err(ExecutionError::SchemaNotFound(database_name, self.old_name))
        }
    }
}

/// Errors that can occur during the execution of the [`RenameSchema`] command.
#[derive(Debug, Display)]
pub enum ExecutionError {
    /// The schema with the old name was not found.
    #[display(fmt = "Schema `{}`.`{}` not found", _0, _1)]
    SchemaNotFound(database::Name, schema::Name),

    /// The schema with the new name already exists.
    #[display(fmt = "Schema `{}`.`{}` already exists", _0, _1)]
    SchemaAlreadyExists(database::Name, schema::Name),
}

#[cfg(test)]
mod tests {
    use backend::{schema, schema::database};
    use common::structs::hash_table::MutHashTable as _;

    use crate::api::command::{
        database::rename_schema::{ExecutionError, RenameSchema},
        extract::DatabaseExtractionError,
        gateway::{test::TestBackendFacade, GatewayError},
        Gateway as _,
    };

    #[test]
    fn renames_schema_when_exists() {
        let database_name = database::Name::from("test");
        let schema_name = schema::Name::from("schema");
        let new_schema_name = schema::Name::from("new_schema");

        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .with_schema(database_name.clone(), schema_name.clone())
            .build();
        let cmd = RenameSchema {
            database_name: Some(database_name.clone()),
            old_name: schema_name.clone(),
            new_name: new_schema_name.clone(),
        };
        let result = facade.send(cmd);
        assert!(result.is_ok());

        let db = facade
            .database_controllers
            .get_mut_value(&database_name)
            .unwrap();

        let schema = db.get_mut_schema(&new_schema_name);
        assert!(schema.is_some());

        let schema = db.get_mut_schema(&schema_name);
        assert!(schema.is_none());
    }

    #[test]
    fn renames_schema_with_db_in_context() {
        let database_name = database::Name::from("test");
        let schema_name = schema::Name::from("schema");
        let new_schema_name = schema::Name::from("new_schema");

        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .with_schema(database_name.clone(), schema_name.clone())
            .with_db_in_context(database_name.clone())
            .build();
        let cmd = RenameSchema {
            database_name: None,
            old_name: schema_name.clone(),
            new_name: new_schema_name.clone(),
        };
        let result = facade.send(cmd);
        assert!(result.is_ok());

        let db = facade
            .database_controllers
            .get_mut_value(&database_name)
            .unwrap();

        let schema = db.get_mut_schema(&new_schema_name);
        assert!(schema.is_some());

        let schema = db.get_mut_schema(&schema_name);
        assert!(schema.is_none());
    }

    #[test]
    fn returns_error_when_db_not_provided() {
        let database_name = database::Name::from("test");
        let schema_name = schema::Name::from("schema");
        let new_schema_name = schema::Name::from("new_schema");

        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .build();
        let cmd = RenameSchema {
            database_name: None,
            old_name: schema_name.clone(),
            new_name: new_schema_name.clone(),
        };
        let result = facade.send(cmd);
        assert!(result.is_err());

        match result {
            Err(GatewayError::ByNotProvided(_)) => {}
            _ => panic!("Expected `ByNotProvided` found {:?}", result),
        }
    }

    #[test]
    fn returns_error_when_schema_with_new_name_exists() {
        let database_name = database::Name::from("test");
        let schema_name = schema::Name::from("schema");
        let new_schema_name = schema::Name::from("new_schema");

        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .with_schema(database_name.clone(), schema_name.clone())
            .with_schema(database_name.clone(), new_schema_name.clone())
            .build();
        let cmd = RenameSchema {
            database_name: Some(database_name.clone()),
            old_name: schema_name.clone(),
            new_name: new_schema_name.clone(),
        };
        let result = facade.send(cmd);
        assert!(result.is_err());

        match result {
            Err(GatewayError::CommandError(
                ExecutionError::SchemaAlreadyExists(db_name, name),
            )) => {
                assert_eq!(db_name, database_name);
                assert_eq!(name, new_schema_name);
            }
            _ => panic!("Expected `SchemaAlreadyExists` found {:?}", result),
        }
    }

    #[test]
    fn returns_error_when_schema_with_old_name_not_exists() {
        let database_name = database::Name::from("test");
        let schema_name = schema::Name::from("schema");
        let new_schema_name = schema::Name::from("new_schema");

        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .build();
        let cmd = RenameSchema {
            database_name: Some(database_name.clone()),
            old_name: schema_name.clone(),
            new_name: new_schema_name.clone(),
        };
        let result = facade.send(cmd);
        assert!(result.is_err());

        match result {
            Err(GatewayError::CommandError(
                ExecutionError::SchemaNotFound(db_name, name),
            )) => {
                assert_eq!(db_name, database_name);
                assert_eq!(name, schema_name);
            }
            _ => panic!("Expected `SchemaNotFound` found {:?}", result),
        }
    }

    #[test]
    fn returns_error_when_db_not_exists() {
        let database_name = database::Name::from("test");
        let schema_name = schema::Name::from("schema");
        let new_schema_name = schema::Name::from("new_schema");

        let mut facade = TestBackendFacade::<4>::new().build();
        let cmd = RenameSchema {
            database_name: Some(database_name.clone()),
            old_name: schema_name.clone(),
            new_name: new_schema_name.clone(),
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
