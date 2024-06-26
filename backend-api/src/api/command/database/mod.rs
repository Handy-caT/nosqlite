mod create_schema;
mod drop_schema;
mod rename_schema;
mod show_schemas;

use backend::{controller, schema::database};
use derive_more::Display;

use crate::api::{
    command::{Command, DatabaseCommand},
    CommandResultString,
};

pub use create_schema::CreateSchema;
pub use drop_schema::DropSchema;
pub use rename_schema::RenameSchema;
pub use show_schemas::ShowSchemas;

/// Commands that can be executed on the database.
#[derive(Debug, Clone, PartialEq)]
pub enum SchemaCommand {
    /// Command to create a new schema.
    Create(CreateSchema),

    /// Command to drop a schema.
    Drop(DropSchema),

    /// Command to rename a schema.
    Rename(RenameSchema),

    /// Command to show schemas.
    Show(ShowSchemas),
}

impl DatabaseCommand for SchemaCommand {
    fn get_db_name(&self) -> Option<database::Name> {
        match self {
            SchemaCommand::Create(command) => command.get_db_name(),
            SchemaCommand::Drop(command) => command.get_db_name(),
            SchemaCommand::Rename(command) => command.get_db_name(),
            SchemaCommand::Show(command) => command.get_db_name(),
        }
    }

    fn get_db_name_mut(&mut self) -> &mut Option<database::Name> {
        match self {
            SchemaCommand::Create(command) => command.get_db_name_mut(),
            SchemaCommand::Drop(command) => command.get_db_name_mut(),
            SchemaCommand::Rename(command) => command.get_db_name_mut(),
            SchemaCommand::Show(command) => command.get_db_name_mut(),
        }
    }
}

impl<const NODE_SIZE: u8> Command<controller::Database<NODE_SIZE>>
    for SchemaCommand
{
    type Ok = CommandResultString;
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
            SchemaCommand::Show(command) => command
                .execute(db_controller)
                .map_err(ExecutionError::ShowSchemas),
        }
    }
}

/// Errors that can occur during the execution of [`SchemaCommand`].
#[derive(Debug, Display)]
pub enum ExecutionError {
    /// Create schema error.
    CreateSchema(create_schema::ExecutionError),

    /// Drop schema error.
    DropSchema(drop_schema::ExecutionError),

    /// Rename schema error.
    RenameSchema(rename_schema::ExecutionError),

    /// Show schemas error.
    ShowSchemas(show_schemas::ExecutionError),
}

/// Errors that can occur when executing the [`SchemaCommand`].
#[derive(Debug, Display)]
pub enum ProvideError {
    /// Database not provided.
    #[display(fmt = "Database not provided in the `Context`.\n\
                     Use the `USE DATABASE` command to set the database \
                     or use `db_name`.`schema_name` to specify the database.")]
    DatabaseNotProvided,
}
