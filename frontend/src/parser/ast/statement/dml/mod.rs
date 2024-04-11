mod database;
mod schema;

pub use database::DatabaseNode;
pub use schema::SchemaNode;

/// Represents an AST node for a DML operation.
#[derive(Debug, PartialEq, Clone)]
pub enum DML {
    /// Represents a database operation.
    Database(DatabaseNode),

    /// Represents a schema operation.
    Schema(SchemaNode),
}
