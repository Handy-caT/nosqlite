mod create_database;
mod drop_database;

use clap::Parser;
pub use create_database::CreateDatabase;
pub use drop_database::DropDatabase;

/// Commands that can be executed on the database.
#[derive(Debug, Clone, PartialEq)]
pub enum DatabaseCommand {
    /// Command to create a new database.
    Create(CreateDatabase),

    /// Command to drop a database.
    Drop(DropDatabase),
}
