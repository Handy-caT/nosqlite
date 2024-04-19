use crate::api::command::{
    backend_api::DatabaseCommand, database::SchemaCommand,
};

/// Commands that can be executed on the whole backend.
#[derive(Debug, Clone, PartialEq)]
pub enum BackendCommand {
    /// Command to operate on a database.
    Database(DatabaseCommand),

    /// Command to operate on a schema.
    Schema(SchemaCommand),
}
