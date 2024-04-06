use backend::{controller, schema};
use derive_more::AsRef;

use crate::api::{
    command::{Command, Execute},
    facade::BackendFacade,
};

#[derive(Debug, AsRef, Clone)]
pub struct RenameSchema {
    #[as_ref]
    pub database_name: schema::database::Name,

    pub old_name: schema::Name,
    
    pub new_name: schema::Name,
}

impl Command for RenameSchema {}

impl<const NODE_SIZE: u8> Execute<RenameSchema, controller::Database<NODE_SIZE>>
for BackendFacade<NODE_SIZE>
{
    type Ok = ();
    type Err = ExecutionError;

    fn execute(
        cmd: RenameSchema,
        db_controller: &mut controller::Database<NODE_SIZE>,
    ) -> Result<Self::Ok, Self::Err> {
        if !db_controller.has_schema(&cmd.old_name) {
            return Err(ExecutionError::SchemaNotFound);
        }
        if db_controller.has_schema(&cmd.new_name) {
            return Err(ExecutionError::SchemaAlreadyExists);
        }

        let schema = db_controller.get_mut_schema(&cmd.old_name);
        if let Some(schema) = schema {
            let info = schema.get_mut_info();
            info.name = cmd.new_name;
            
            Ok(())
        } else {
            return Err(ExecutionError::SchemaNotFound);
        }
    }
}

#[derive(Debug)]
pub enum ExecutionError {
    SchemaNotFound,
    SchemaAlreadyExists,
}

#[cfg(test)]
mod tests {
    use backend::{schema, schema::database};
    use common::structs::hash_table::MutHashTable as _;

    use crate::api::command::{
        gateway::{test::TestBackendFacade, DatabaseGatewayError},
        Gateway as _, GatewayError,
    };
    use crate::api::command::database::rename_schema::{ExecutionError, RenameSchema};

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
            database_name: database_name.clone(),
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
            database_name: database_name.clone(),
            old_name: schema_name.clone(),
            new_name: new_schema_name.clone(),
        };
        let result = facade.send(cmd);
        assert!(result.is_err());

        match result {
            Err(GatewayError::Cmd(ExecutionError::SchemaAlreadyExists)) => {}
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
            database_name: database_name.clone(),
            old_name: schema_name.clone(),
            new_name: new_schema_name.clone(),
        };
        let result = facade.send(cmd);
        assert!(result.is_err());

        match result {
            Err(GatewayError::Cmd(ExecutionError::SchemaNotFound)) => {}
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
            database_name: database_name.clone(),
            old_name: schema_name.clone(),
            new_name: new_schema_name.clone(),
        };
        let result = facade.send(cmd);
        assert!(result.is_err());

        match result {
            Err(GatewayError::Gateway(
                    DatabaseGatewayError::DatabaseNotFound,
                )) => {}
            _ => panic!("Expected `DatabaseNotFound` found {:?}", result),
        }
    }
}
