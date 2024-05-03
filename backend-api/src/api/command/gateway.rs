use derive_more::Display;
use std::fmt::{Debug, Display};

use crate::api::{
    command::{Command, ContextReceiver, Gateway, OptionalBy, TryExtract},
    facade::BackendFacade,
};

impl<Cmd, Ctx, By, const NODE_SIZE: u8> Gateway<Cmd, Ctx>
    for BackendFacade<NODE_SIZE>
where
    Cmd: Command<Ctx> + OptionalBy<By> + ContextReceiver<By>,
    <Cmd as Command<Ctx>>::Err: Display,
    <Cmd as OptionalBy<By>>::Err: Display,
    Self: TryExtract<Ctx, By = By>,
    <Self as TryExtract<Ctx>>::Err: Debug,
{
    type Ok = <Cmd as Command<Ctx>>::Ok;
    type Err = GatewayError<
        <Cmd as Command<Ctx>>::Err,
        <Self as TryExtract<Ctx>>::Err,
        <Cmd as OptionalBy<By>>::Err,
    >;

    #[rustfmt::skip]
    fn send(
        &mut self,
        mut cmd: Cmd,
    ) -> Result<
        <Self as Gateway<Cmd, Ctx>>::Ok,
        <Self as Gateway<Cmd, Ctx>>::Err,
    > {
        cmd.receive(&self.context);

        let by = cmd.by().map_err(GatewayError::ByNotProvided)?;

        let ctx = self
            .try_extract_mut(by)
            .map_err(GatewayError::ExtractionError)?;
        <Cmd as Command<Ctx>>::execute(cmd, ctx)
            .map_err(GatewayError::CommandError)
    }
}

/// Represents an error that occurred during the execution of a command.
#[derive(Debug, Display)]
pub enum GatewayError<CmdErr, ExtractErr, ByError>
where
    CmdErr: Display,
    ExtractErr: Debug,
    ByError: Display,
{
    /// An error occurred during the execution of the command.
    #[display(fmt = "{}", _0)]
    CommandError(CmdErr),

    /// An error occurred during the extraction of the context.
    #[display(fmt = "{:?}", _0)]
    ExtractionError(ExtractErr),

    /// Can't extract the context because the command doesn't provide the
    /// necessary information.
    #[display(fmt = "{}", _0)]
    ByNotProvided(ByError),
}

#[cfg(test)]
pub mod test {
    use std::sync::{Arc, Mutex};

    use backend::{
        controller,
        data::id,
        page::page_controller::PageController,
        schema,
        schema::{database, table},
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

        /// Adds a table to the `BackendFacade`.
        pub fn with_table(
            mut self,
            database_name: database::Name,
            schema_name: schema::Name,
            table_name: table::Name,
        ) -> Self {
            let database = self
                .0
                .database_controllers
                .get_mut_value(&database_name)
                .expect("database exists");
            let schema = database
                .get_mut_schema(&schema_name)
                .expect("schema exists");
            let table = controller::Table::new(table_name);
            schema.add_table(table);
            self
        }

        /// Sets the current database in the `BackendFacade`'s context.
        pub fn with_db_in_context(mut self, name: database::Name) -> Self {
            self.0.context.set_current_db(name);
            self
        }

        /// Sets the current schema in the `BackendFacade`'s context.
        pub fn with_schema_in_context(mut self, name: schema::Name) -> Self {
            self.0.context.set_current_schema(name);
            self
        }

        /// Builds the `BackendFacade`.
        pub fn build(self) -> BackendFacade<NODE_SIZE> {
            self.0
        }
    }
}
