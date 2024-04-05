use common::structs::hash_table::MutHashTable;
use std::fmt::Debug;

use crate::{
    api::{
        command::{
            Command, Execute, ExecuteBackend, ExecuteDatabase, Gateway,
            GatewayError,
        },
        facade::BackendFacade,
    },
    controller,
    schema::database,
};

impl<Cmd, const NODE_SIZE: u8> Gateway<Cmd, controller::Database<NODE_SIZE>>
    for BackendFacade<NODE_SIZE>
where
    Self: ExecuteDatabase<Cmd, NODE_SIZE>,
    <Self as Execute<Cmd, controller::Database<NODE_SIZE>>>::Err: Debug,
    Cmd: Command + AsRef<database::Name>,
{
    type Ok = <Self as Execute<Cmd, controller::Database<NODE_SIZE>>>::Ok;
    type Err = GatewayError<
        <Self as Execute<Cmd, controller::Database<NODE_SIZE>>>::Err,
        DatabaseGatewayError,
    >;

    fn send(
        &mut self,
        cmd: Cmd,
    ) -> Result<
        <Self as Gateway<Cmd, controller::Database<NODE_SIZE>>>::Ok,
        <Self as Gateway<Cmd, controller::Database<NODE_SIZE>>>::Err,
    > {
        let database_name = cmd.as_ref();
        let database = self.database_controllers.get_mut_value(database_name);
        if let Some(database) = database {
            <Self as Execute<Cmd, controller::Database<NODE_SIZE>>>::execute(
                cmd, database,
            )
            .map_err(GatewayError::Cmd)
        } else {
            Err(GatewayError::Gateway(
                DatabaseGatewayError::DatabaseNotFound,
            ))
        }
    }
}

#[derive(Debug)]
pub enum DatabaseGatewayError {
    DatabaseNotFound,
}

impl<Cmd, const NODE_SIZE: u8> Gateway<Cmd, Self> for BackendFacade<NODE_SIZE>
where
    Self: ExecuteBackend<Cmd, NODE_SIZE>,
    <Self as Execute<Cmd, Self>>::Err: Debug,
    Cmd: Command,
{
    type Ok = <Self as Execute<Cmd, Self>>::Ok;
    type Err = GatewayError<<Self as Execute<Cmd, Self>>::Err, ()>;

    fn send(
        &mut self,
        cmd: Cmd,
    ) -> Result<
        <Self as Gateway<Cmd, Self>>::Ok,
        <Self as Gateway<Cmd, Self>>::Err,
    > {
        <Self as Execute<Cmd, Self>>::execute(cmd, self)
            .map_err(GatewayError::Cmd)
    }
}

#[cfg(test)]
pub mod test {
    use common::structs::hash_table::{HashTable, MutHashTable};
    use std::sync::{Arc, Mutex};

    use crate::{
        api::facade::BackendFacade, controller, data::id,
        page::page_controller::PageController, schema, schema::database,
    };

    /// Creates a new instance of `BackendFacade` for testing.
    pub struct TestBackendFacade<const NODE_SIZE: u8>(BackendFacade<NODE_SIZE>);

    impl<const NODE_SIZE: u8> TestBackendFacade<NODE_SIZE> {
        /// Creates a new instance of `BackendFacade` for testing.
        pub fn new() -> Self {
            let id_registry = Arc::new(Mutex::new(id::Registry::new()));
            let page_controller = Arc::new(Mutex::new(PageController::new()));

            TestBackendFacade(BackendFacade::new(page_controller, id_registry))
        }

        /// Adds a database to the `BackendFacade`.
        pub fn with_database(mut self, name: database::Name) -> Self {
            let database = controller::Database::new(name.clone());
            self.0.database_controllers.insert(name, database);
            self
        }

        pub fn with_schema(
            mut self,
            database_name: database::Name,
            schema_name: schema::Name,
        ) -> Self {
            let database = self
                .0
                .database_controllers
                .get_mut_value(&database_name)
                .expect("database exists");
            let schema = controller::Schema::new(schema_name);
            database.add_schema(schema);
            self
        }

        /// Builds the `BackendFacade`.
        pub fn build(self) -> BackendFacade<NODE_SIZE> {
            self.0
        }
    }
}
