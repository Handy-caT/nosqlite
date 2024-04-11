mod create_database;
mod drop_database;

pub use create_database::CreateDatabase;
pub use drop_database::DropDatabase;

/// Represents an AST node for a database operation.
#[derive(Debug, PartialEq, Clone)]
pub enum DatabaseNode {
    /// Represents a `DROP DATABASE ...` statement.
    DropDatabase(DropDatabase),

    /// Represents a `CREATE DATABASE ...` statement.
    CreateDatabase(CreateDatabase),
}
