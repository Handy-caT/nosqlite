use common::structs::hash_table::HashTable;
use backend::{    controller,
                  schema::database,};

use crate::{
    api::{
        command::{Command, Execute},
        facade::BackendFacade,
    },
};

#[derive(Debug, Clone)]
pub struct CreateDatabase {
    pub name: database::Name,
}

impl Command for CreateDatabase {}

impl<const NODE_SIZE: u8> Execute<CreateDatabase, Self>
    for BackendFacade<NODE_SIZE>
{
    type Ok = ();
    type Err = ExecutionError;

    fn execute(
        cmd: CreateDatabase,
        backend: &mut Self,
    ) -> Result<Self::Ok, Self::Err> {
        if backend.database_controllers.contains_key(&cmd.name) {
            return Err(ExecutionError::DatabaseAlreadyExists);
        }
        let db = controller::Database::new(cmd.name.clone());
        backend.database_controllers.insert(cmd.name, db);
        Ok(())
    }
}

#[derive(Debug)]
pub enum ExecutionError {
    DatabaseAlreadyExists,
}

#[cfg(test)]
mod tests {
    use common::structs::hash_table::MutHashTable;
    use backend::schema::database;
    
    use crate::{
        api::command::{
            backend_api::create_database::{CreateDatabase, ExecutionError},
            gateway::test::TestBackendFacade,
            Gateway, GatewayError,
        },
    };

    #[test]
    fn creates_db_when_not_exists() {
        let name = database::Name::from("test");
        let mut facade = TestBackendFacade::<4>::new().build();
        let cmd = CreateDatabase { name: name.clone() };
        let result = facade.send(cmd);
        assert!(result.is_ok());

        let db = facade.database_controllers.get_mut_value(&name);
        assert!(db.is_some());
    }

    #[test]
    fn returns_error_when_db_exists() {
        let name = database::Name::from("test");
        let mut facade = TestBackendFacade::<4>::new()
            .with_database(name.clone())
            .build();
        let cmd = CreateDatabase { name: name.clone() };
        let result = facade.send(cmd);
        assert!(result.is_err());

        match result {
            Err(GatewayError::Cmd(ExecutionError::DatabaseAlreadyExists)) => (),
            _ => panic!(
                "Expected `DatabaseAlreadyExists` error, found {:?}",
                result
            ),
        }
    }
}
