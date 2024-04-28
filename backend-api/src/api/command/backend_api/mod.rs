mod create_database;
mod drop_database;
mod use_database;
pub mod use_schema;

use std::fmt::Debug;

use crate::api::{command::Command, facade::BackendFacade};

pub use create_database::CreateDatabase;
pub use drop_database::DropDatabase;
pub use use_database::UseDatabase;
pub use use_schema::UseSchema;
use crate::api::command::{ContextReceiver, OptionalRef};

/// Commands that can be executed on the database.
#[derive(Debug, Clone, PartialEq)]
pub enum DatabaseCommand {
    /// Command to create a new database.
    Create(CreateDatabase),

    /// Command to drop a database.
    Drop(DropDatabase),

    /// Command to use a database.
    Use(UseDatabase),

    // Command to use a schema in a database.
    UseSchema(UseSchema),
}

impl ContextReceiver for DatabaseCommand {
    fn receive(&mut self, context: &crate::Context) {
        match self {
            DatabaseCommand::Create(command) => command.receive(context),
            DatabaseCommand::Drop(command) => command.receive(context),
            DatabaseCommand::Use(command) => command.receive(context),
            DatabaseCommand::UseSchema(command) => command.receive(context),
        }
    }
}

impl<const NODE_SIZE: u8> Command<BackendFacade<NODE_SIZE>>
    for DatabaseCommand
{
    type Ok = ();
    type Err = ExecutionError;

    fn execute(
        self,
        facade: &mut BackendFacade<NODE_SIZE>,
    ) -> Result<Self::Ok, Self::Err> {
        match self {
            DatabaseCommand::Create(command) => command
                .execute(facade)
                .map_err(ExecutionError::CreateDatabase),
            DatabaseCommand::Drop(command) => command
                .execute(facade)
                .map_err(ExecutionError::DropDatabase),
            DatabaseCommand::Use(command) => {
                command.execute(facade).map_err(ExecutionError::UseDatabase)
            }
            DatabaseCommand::UseSchema(command) => {
                command.execute(facade).map_err(ExecutionError::UseSchema)
            }
        }
    }
}

/// Errors that can occur when executing the [`DatabaseCommand`].
#[derive(Debug)]
pub enum ExecutionError {
    /// Create database error.
    CreateDatabase(create_database::ExecutionError),

    /// Drop database error.
    DropDatabase(drop_database::ExecutionError),

    /// Use database error.
    UseDatabase(use_database::ExecutionError),

    /// Use schema error.
    UseSchema(use_schema::ExecutionError),
}
