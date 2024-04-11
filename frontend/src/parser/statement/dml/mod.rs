mod database;
mod schema;

pub use database::*;
pub use schema::*;

/// Represents an AST node for a DML operation.
#[derive(Debug, PartialEq, Clone)]
pub enum DML {
    /// Represents a database operation.
    Database(DatabaseNode),

    /// Represents a schema operation.
    Schema(SchemaNode),
}
