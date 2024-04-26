use backend::{controller, schema};
use derive_more::AsRef;
use common::structs::hash_table::MutHashTable;

use crate::api::{command::Command, facade::BackendFacade};

/// [`Command`] which is used to rename a [`controller::Schema`].
#[derive(Debug, Clone, PartialEq)]
pub struct RenameSchema {
    /// The name of the database where the schema is located.
    pub database_name: Option<schema::database::Name>,

    /// The old name of the schema.
    pub old_name: schema::Name,

    /// The new name of the schema.
    pub new_name: schema::Name,
}

impl AsRef<()> for RenameSchema {
    fn as_ref(&self) -> &() {
        &()
    }
}

impl<const NODE_SIZE: u8> Command<BackendFacade<NODE_SIZE>>
    for RenameSchema
{
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
        
        if !db_controller.has_schema(&self.old_name) {
            return Err(ExecutionError::SchemaNotFound(self.old_name));
        }
        if db_controller.has_schema(&self.new_name) {
            return Err(ExecutionError::SchemaAlreadyExists(self.new_name));
        }

        let schema = db_controller.remove_schema(&self.old_name);
        if let Some(mut schema) = schema {
            let info = schema.get_mut_info();
            info.name = self.new_name;
            db_controller.add_schema(schema);

            Ok(())
        } else {
            Err(ExecutionError::SchemaNotFound(self.old_name))
        }
    }
}

/// Errors that can occur during the execution of the [`RenameSchema`] command.
#[derive(Debug)]
pub enum ExecutionError {
    /// The database was not provided.
    DatabaseNotProvided,

    /// Provided database does not exist.
    DatabaseNotExists(schema::database::Name),
    
    /// The schema with the old name was not found.
    SchemaNotFound(schema::Name),

    /// The schema with the new name already exists.
    SchemaAlreadyExists(schema::Name),
}

#[cfg(test)]
mod tests {
    use backend::{schema, schema::database};
    use common::structs::hash_table::MutHashTable as _;

    use crate::api::command::{
        database::rename_schema::{ExecutionError, RenameSchema},
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
            Err(GatewayError::CommandError(
                    ExecutionError::DatabaseNotProvided,
                )) => {}
            _ => panic!("Expected `DatabaseNotProvided` found {:?}", result),
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
                ExecutionError::SchemaAlreadyExists(name),
            )) => {
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
                ExecutionError::SchemaNotFound(name),
            )) => {
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
            Err(GatewayError::CommandError(
                ExecutionError::DatabaseNotExists(name),
            )) => {
                assert_eq!(name, database_name);
            }
            _ => panic!("Expected `DatabaseNotFound` found {:?}", result),
        }
    }
}
