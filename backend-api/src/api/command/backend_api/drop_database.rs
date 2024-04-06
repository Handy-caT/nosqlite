use std::convert::Infallible;

use backend::schema::database;
use common::structs::hash_table::HashTable;

use crate::api::{
    command::{Command, Execute},
    facade::BackendFacade,
};

/// Command to drop a database.
#[derive(Debug, Clone)]
pub struct DropDatabase {
    /// The name of the database to drop.
    pub name: database::Name,
}

impl Command for DropDatabase {}

impl<const NODE_SIZE: u8> Execute<DropDatabase, Self>
    for BackendFacade<NODE_SIZE>
{
    type Ok = ();
    type Err = Infallible;

    fn execute(
        cmd: DropDatabase,
        backend: &mut Self,
    ) -> Result<Self::Ok, Self::Err> {
        if !backend.database_controllers.contains_key(&cmd.name) {
            return Ok(());
        }
        backend.database_controllers.remove(&cmd.name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use backend::schema::database;
    use common::structs::hash_table::MutHashTable as _;

    use crate::api::command::{
        backend_api::drop_database::DropDatabase,
        gateway::test::TestBackendFacade, Gateway as _,
    };

    #[test]
    fn drops_db_when_exists() {
        let name = database::Name::from("test");
        let mut facade = TestBackendFacade::<4>::new()
            .with_database(name.clone())
            .build();
        let cmd = DropDatabase { name: name.clone() };
        let result = facade.send(cmd);
        assert!(result.is_ok());

        let db = facade.database_controllers.get_mut_value(&name);
        assert!(db.is_none());
    }

    #[test]
    fn not_error_when_db_not_exists() {
        let name = database::Name::from("test");
        let mut facade = TestBackendFacade::<4>::new().build();
        let cmd = DropDatabase { name: name.clone() };
        let result = facade.send(cmd);
        assert!(result.is_ok());

        let db = facade.database_controllers.get_mut_value(&name);
        assert!(db.is_none());
    }
}
