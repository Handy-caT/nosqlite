mod create_database;
mod drop_database;
mod use_database;
mod use_schema;

use std::fmt::Debug;

use crate::api::{
    command::{Command, Execute},
    facade::BackendFacade,
};

pub use create_database::CreateDatabase;
pub use drop_database::DropDatabase;
pub use use_database::UseDatabase;

/// Commands that can be executed on the database.
#[derive(Debug, Clone, PartialEq)]
pub enum DatabaseCommand {
    /// Command to create a new database.
    Create(CreateDatabase),

    /// Command to drop a database.
    Drop(DropDatabase),

    Use(UseDatabase),
}

impl Command for DatabaseCommand {}

impl<const NODE_SIZE: u8> Execute<DatabaseCommand, Self>
    for BackendFacade<NODE_SIZE>
{
    type Ok = ();
    type Err = ExecutionError;

    fn execute(
        cmd: DatabaseCommand,
        backend: &mut Self,
    ) -> Result<Self::Ok, Self::Err> {
        match cmd {
            DatabaseCommand::Create(cmd) => {
                <Self as Execute<CreateDatabase, Self>>::execute(cmd, backend)
                    .map_err(ExecutionError::CreateDatabase)
            }
            DatabaseCommand::Drop(cmd) => {
                <Self as Execute<DropDatabase, Self>>::execute(cmd, backend)
                    .map_err(ExecutionError::DropDatabase)
            }
            DatabaseCommand::Use(cmd) => {
                <Self as Execute<UseDatabase, Self>>::execute(cmd, backend)
                    .map_err(ExecutionError::UseDatabase)
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

    UseDatabase(use_database::ExecutionError),
}
