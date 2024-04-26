mod create_schema;
mod drop_schema;
mod rename_schema;
pub mod use_schema;

use crate::api::{command::Command, facade::BackendFacade};

pub use create_schema::CreateSchema;
pub use drop_schema::DropSchema;
pub use rename_schema::RenameSchema;
pub use use_schema::UseSchema;

/// Commands that can be executed on the database.
#[derive(Debug, Clone, PartialEq)]
pub enum SchemaCommand {
    /// Command to create a new schema.
    Create(CreateSchema),

    /// Command to drop a schema.
    Drop(DropSchema),

    /// Command to rename a schema.
    Rename(RenameSchema),
    
    /// Command to use a schema.
    Use(UseSchema),
}

impl AsRef<()> for SchemaCommand {
    fn as_ref(&self) -> &() {
        &()
    }
}

impl<const NODE_SIZE: u8> Command<BackendFacade<NODE_SIZE>> for SchemaCommand {
    type Ok = ();
    type Err = ExecutionError;

    fn execute(
        self,
        facade: &mut BackendFacade<NODE_SIZE>,
    ) -> Result<Self::Ok, Self::Err> {
        match self {
            SchemaCommand::Create(command) => command
                .execute(facade)
                .map_err(ExecutionError::CreateSchema),
            SchemaCommand::Use(command) => command
                .execute(facade)
                .map_err(ExecutionError::UseSchema),
            SchemaCommand::Drop(command) => command
                .execute(facade)
                .map_err(ExecutionError::DropSchema),
            SchemaCommand::Rename(command) => command
                .execute(facade)
                .map_err(ExecutionError::RenameSchema),
        }
    }
}

/// Errors that can occur during the execution of [`SchemaCommand`].
#[derive(Debug)]
pub enum ExecutionError {
    /// Create schema error.
    CreateSchema(create_schema::ExecutionError),

    /// Drop schema error.
    DropSchema(drop_schema::ExecutionError),

    /// Rename schema error.
    RenameSchema(rename_schema::ExecutionError),
    
    /// Use schema error.
    UseSchema(use_schema::ExecutionError),
}
