mod create_table;
mod drop_table;

use backend::{controller, schema, schema::database};
use derive_more::Display;

use crate::{
    api::{
        command::{
            Command, ContextReceiver, DatabaseCommand, OptionalBy,
            SchemaCommand,
        },
        CommandResultString,
    },
    Context,
};

pub use create_table::CreateTable;
pub use drop_table::DropTable;

/// Commands that can be executed on the schema.
#[derive(Debug, Clone, PartialEq)]
pub enum TableCommand {
    /// Command to create a new table.
    Create(CreateTable),

    /// Command to drop a table.
    Drop(DropTable),
}

impl DatabaseCommand for TableCommand {
    fn get_db_name(&self) -> Option<database::Name> {
        match self {
            TableCommand::Create(command) => command.get_db_name(),
            TableCommand::Drop(command) => command.get_db_name(),
        }
    }

    fn get_db_name_mut(&mut self) -> &mut Option<database::Name> {
        match self {
            TableCommand::Create(command) => command.get_db_name_mut(),
            TableCommand::Drop(command) => command.get_db_name_mut(),
        }
    }
}

impl SchemaCommand for TableCommand {
    fn get_schema_name(&self) -> Option<schema::Name> {
        match self {
            TableCommand::Create(command) => command.get_schema_name(),
            TableCommand::Drop(command) => command.get_schema_name(),
        }
    }

    fn get_schema_name_mut(&mut self) -> &mut Option<schema::Name> {
        match self {
            TableCommand::Create(command) => command.get_schema_name_mut(),
            TableCommand::Drop(command) => command.get_schema_name_mut(),
        }
    }
}

impl<const NODE_SIZE: u8> Command<controller::Schema<NODE_SIZE>>
    for TableCommand
{
    type Ok = CommandResultString;
    type Err = ExecutionError;

    fn execute(
        self,
        schema_controller: &mut controller::Schema<NODE_SIZE>,
    ) -> Result<Self::Ok, Self::Err> {
        match self {
            TableCommand::Create(command) => command
                .execute(schema_controller)
                .map_err(ExecutionError::CreateTable),
            TableCommand::Drop(command) => command
                .execute(schema_controller)
                .map_err(ExecutionError::DropTable),
        }
    }
}

/// Errors that can occur during the execution of [`SchemaCommand`].
#[derive(Debug, Display)]
pub enum ExecutionError {
    /// Create table error.
    CreateTable(create_table::ExecutionError),

    /// Drop table error.
    DropTable(drop_table::ExecutionError),
}

/// Errors that can occur when executing the [`SchemaCommand`].
#[derive(Debug, Display)]
pub enum ProvideError {
    /// The database was not provided.
    #[display(fmt = "Database not provided in the `Context`.\n\
                     Use the `USE DATABASE` command to set the current \
                     database or use `db_name`.`schema_name`.`table_name` to \
                     specify the database and schema names.")]
    DatabaseNotProvided,

    /// The schema was not provided.
    #[display(fmt = "Schema not provided in the `Context`\n\
                     Use the `USE SCHEMA` command to set the current schema \
                     or use `schema_name`.`table_name` to specify the schema \
                     name.")]
    SchemaNotProvided,
}
