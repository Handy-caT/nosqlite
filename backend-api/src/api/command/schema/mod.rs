mod create_table;
mod drop_table;

use derive_more::Display;
use backend::{controller, schema, schema::database};

use crate::{
    api::command::{Command, ContextReceiver, OptionalBy},
    Context,
};

use crate::api::CommandResultString;
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

impl OptionalBy<(database::Name, schema::Name)> for TableCommand {
    fn by(&self) -> Option<(database::Name, schema::Name)> {
        match self {
            TableCommand::Create(command) => command.by(),
            TableCommand::Drop(command) => command.by(),
        }
    }
}

impl ContextReceiver for TableCommand {
    fn receive(&mut self, context: &Context) {
        match self {
            TableCommand::Create(command) => command.receive(context),
            TableCommand::Drop(command) => command.receive(context),
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
