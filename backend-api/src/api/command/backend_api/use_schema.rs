use backend::{schema, schema::database};
use common::structs::hash_table::{HashTable, MutHashTable};

use crate::api::{
    command::{Command, ContextReceiver},
    facade::BackendFacade,
};

/// [`Command`] to use a schema in a database.
#[derive(Debug, Clone, PartialEq)]
pub struct UseSchema {
    /// The name of the database where the schema will be used.
    pub database_name: Option<database::Name>,

    /// The name of the schema to use.
    pub name: schema::Name,
}

impl ContextReceiver for UseSchema {}

impl<const NODE_SIZE: u8> Command<BackendFacade<NODE_SIZE>> for UseSchema {
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

        if !backend.database_controllers.contains_key(database_name) {
            return Err(ExecutionError::DatabaseNotExists(
                database_name.clone(),
            ));
        }

        let db_controller = backend
            .database_controllers
            .get_mut_value(database_name)
            .expect("exist because of the check above");
        if !db_controller.has_schema(&self.name) {
            return Err(ExecutionError::SchemaNotExists(self.name));
        }

        backend.context.set_current_db(database_name.clone());
        backend.context.set_current_schema(self.name);
        Ok(())
    }
}

/// Errors that can occur during the execution of [`UseSchema`].
#[derive(Debug)]
pub enum ExecutionError {
    /// The database was not provided.
    DatabaseNotProvided,

    /// The schema not exists in the database.
    DatabaseNotExists(database::Name),

    /// The schema not exists in the database.
    SchemaNotExists(schema::Name),
}

#[cfg(test)]
mod tests {
    use backend::{schema, schema::database};

    use crate::api::command::{
        gateway::{test::TestBackendFacade, GatewayError},
        Gateway,
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
            database_name: Some(database_name.clone()),
            name: schema_name.clone(),
        };
        let result = facade.send(cmd);
        assert!(result.is_ok());

        assert!(facade.context.current_schema().is_some());
        assert_eq!(facade.context.current_schema().unwrap(), &schema_name)
    }

    #[test]
    fn sets_bd_name_too() {
        let database_name = database::Name::from("test");
        let schema_name = schema::Name::from("schema");

        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .with_schema(database_name.clone(), schema_name.clone())
            .build();
        let cmd = UseSchema {
            database_name: Some(database_name.clone()),
            name: schema_name.clone(),
        };
        let result = facade.send(cmd);
        assert!(result.is_ok());

        assert!(facade.context.current_schema().is_some());
        assert_eq!(facade.context.current_schema().unwrap(), &schema_name);

        assert!(facade.context.current_db().is_some());
        assert_eq!(facade.context.current_db().unwrap(), &database_name)
    }

    #[test]
    fn use_schema_with_db_in_context() {
        let database_name = database::Name::from("test");
        let schema_name = schema::Name::from("schema");

        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .with_schema(database_name.clone(), schema_name.clone())
            .with_db_in_context(database_name.clone())
            .build();
        let cmd = UseSchema {
            database_name: None,
            name: schema_name.clone(),
        };
        let result = facade.send(cmd);
        assert!(result.is_ok());

        assert!(facade.context.current_schema().is_some());
        assert_eq!(facade.context.current_schema().unwrap(), &schema_name)
    }

    #[test]
    fn returns_error_when_db_not_provided() {
        let database_name = database::Name::from("test");
        let schema_name = schema::Name::from("schema");

        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .with_schema(database_name.clone(), schema_name.clone())
            .build();
        let cmd = UseSchema {
            database_name: None,
            name: schema_name.clone(),
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
    fn returns_error_when_schema_not_exists() {
        let database_name = database::Name::from("test");
        let schema_name = schema::Name::from("schema");

        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .build();
        let cmd = UseSchema {
            database_name: Some(database_name.clone()),
            name: schema_name.clone(),
        };
        let result = facade.send(cmd);
        assert!(result.is_err());

        match result {
            Err(GatewayError::CommandError(
                ExecutionError::SchemaNotExists(name),
            )) => {
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
