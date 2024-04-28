mod create_schema;
mod drop_schema;
mod rename_schema;

use crate::api::{command::Command, facade::BackendFacade};
use backend::{controller, schema};

pub use create_schema::CreateSchema;
pub use drop_schema::DropSchema;
pub use rename_schema::RenameSchema;
use crate::api::command::{ContextReceiver, OptionalRef};
use crate::Context;

/// Commands that can be executed on the database.
#[derive(Debug, Clone, PartialEq)]
pub enum SchemaCommand {
    /// Command to create a new schema.
    Create(CreateSchema),

    /// Command to drop a schema.
    Drop(DropSchema),

    /// Command to rename a schema.
    Rename(RenameSchema),
}

impl OptionalRef<schema::database::Name> for SchemaCommand {
    fn as_ref(&self) -> Option<&schema::database::Name> {
        match self {
            SchemaCommand::Create(command) => command.as_ref(),
            SchemaCommand::Drop(command) => command.as_ref(),
            SchemaCommand::Rename(command) => command.as_ref(),
        }
    }
}

impl ContextReceiver for SchemaCommand {
    fn receive(&mut self, context: &Context) {
        match self {
            SchemaCommand::Create(command) => command.receive(context),
            SchemaCommand::Drop(command) => command.receive(context),
            SchemaCommand::Rename(command) => command.receive(context),
        }
    }

}

impl<const NODE_SIZE: u8> Command<controller::Database<NODE_SIZE>>
    for SchemaCommand
{
    type Ok = ();
    type Err = ExecutionError;

    fn execute(
        self,
        db_controller: &mut controller::Database<NODE_SIZE>,
    ) -> Result<Self::Ok, Self::Err> {
        match self {
            SchemaCommand::Create(command) => command
                .execute(db_controller)
                .map_err(ExecutionError::CreateSchema),
            SchemaCommand::Drop(command) => command
                .execute(db_controller)
                .map_err(ExecutionError::DropSchema),
            SchemaCommand::Rename(command) => command
                .execute(db_controller)
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
}
