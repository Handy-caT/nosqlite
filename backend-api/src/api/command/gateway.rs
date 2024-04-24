use std::fmt::Debug;

use crate::api::{
    command::{Command, Gateway, TryExtractBy},
    facade::BackendFacade,
};

impl<Cmd, Ctx, By, const NODE_SIZE: u8> Gateway<Cmd, Ctx>
    for BackendFacade<NODE_SIZE>
where
    Cmd: Command<Ctx> + AsRef<By>,
    <Cmd as Command<Ctx>>::Err: Debug,
    Self: TryExtractBy<Ctx, By = By>,
    <Self as TryExtractBy<Ctx>>::Err: Debug,
{
    type Ok = <Cmd as Command<Ctx>>::Ok;
    type Err = GatewayError<
        <Cmd as Command<Ctx>>::Err,
        <Self as TryExtractBy<Ctx>>::Err,
    >;

    #[rustfmt::skip]
    fn send(
        &mut self,
        cmd: Cmd,
    ) -> Result<
        <Self as Gateway<Cmd, Ctx>>::Ok,
        <Self as Gateway<Cmd, Ctx>>::Err,
    > {
        let ctx = self
            .try_extract(cmd.as_ref())
            .map_err(GatewayError::ExtractionError)?;
        <Cmd as Command<Ctx>>::execute(cmd, ctx)
            .map_err(GatewayError::CommandError)
    }
}

/// Represents an error that occurred during the execution of a command.
#[derive(Debug)]
pub enum GatewayError<CmdErr, ExtractErr> {
    CommandError(CmdErr),
    ExtractionError(ExtractErr),
}

#[cfg(test)]
pub mod test {
    use std::sync::{Arc, Mutex};

    use backend::{
        controller, data::id, page::page_controller::PageController, schema,
        schema::database,
    };
    use common::structs::hash_table::{HashTable, MutHashTable};

    use crate::api::facade::BackendFacade;

    /// Creates a new instance of `BackendFacade` for testing.
    pub struct TestBackendFacade<const NODE_SIZE: u8>(BackendFacade<NODE_SIZE>);

    impl<const NODE_SIZE: u8> TestBackendFacade<NODE_SIZE> {
        /// Creates a new instance of `BackendFacade` for testing.
        pub fn new() -> Self {
            let id_registry = Arc::new(Mutex::new(id::Registry::default()));
            let page_controller =
                Arc::new(Mutex::new(PageController::default()));

            TestBackendFacade(BackendFacade::new(page_controller, id_registry))
        }

        /// Adds a database to the `BackendFacade`.
        pub fn with_database(mut self, name: database::Name) -> Self {
            let database = controller::Database::new(name.clone());
            self.0.database_controllers.insert(name, database);
            self
        }

        /// Adds a schema to the `BackendFacade`.
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

        /// Sets the current database in the `BackendFacade`'s context.
        pub fn with_db_in_context(mut self, name: database::Name) -> Self {
            self.0.context.set_current_db(name);
            self
        }

        /// Builds the `BackendFacade`.
        pub fn build(self) -> BackendFacade<NODE_SIZE> {
            self.0
        }
    }
}
