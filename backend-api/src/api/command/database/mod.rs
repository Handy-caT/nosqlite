mod create_schema;
mod drop_schema;
mod rename_schema;

use crate::api::{command::Execute, facade::BackendFacade};
use backend::{controller, schema::database};

use crate::api::command::Command;
pub use create_schema::CreateSchema;
pub use drop_schema::DropSchema;
pub use rename_schema::RenameSchema;

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

impl AsRef<database::Name> for SchemaCommand {
    fn as_ref(&self) -> &database::Name {
        match self {
            SchemaCommand::Create(cmd) => cmd.as_ref(),
            SchemaCommand::Drop(cmd) => cmd.as_ref(),
            SchemaCommand::Rename(cmd) => cmd.as_ref(),
        }
    }
}

impl Command for SchemaCommand {}

impl<const NODE_SIZE: u8>
    Execute<SchemaCommand, controller::Database<NODE_SIZE>>
    for BackendFacade<NODE_SIZE>
{
    type Ok = ();
    type Err = ExecutionError;

    fn execute(
        cmd: SchemaCommand,
        db_controller: &mut controller::Database<NODE_SIZE>,
    ) -> Result<Self::Ok, Self::Err> {
        match cmd {
            SchemaCommand::Create(cmd) => {
                <Self as Execute<
                    CreateSchema,
                    controller::Database<NODE_SIZE>,
                >>::execute(cmd, db_controller)
                .map_err(ExecutionError::CreateSchema)
            }
            SchemaCommand::Drop(cmd) => <Self as Execute<
                DropSchema,
                controller::Database<NODE_SIZE>,
            >>::execute(
                cmd, db_controller
            )
            .map_err(ExecutionError::DropSchema),
            SchemaCommand::Rename(cmd) => {
                <Self as Execute<
                    RenameSchema,
                    controller::Database<NODE_SIZE>,
                >>::execute(cmd, db_controller)
                .map_err(ExecutionError::RenameSchema)
            }
        }
    }
}

/// Errors that can occur when executing the [`SchemaCommand`].
#[derive(Debug)]
pub enum ExecutionError {
    /// Create schema error.
    CreateSchema(create_schema::ExecutionError),

    /// Drop schema error.
    DropSchema(drop_schema::ExecutionError),

    /// Rename schema error.
    RenameSchema(rename_schema::ExecutionError),
}
