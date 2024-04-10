use crate::parser::ast::statement::dml::database::{
    create_database::CreateDatabase, drop_database::DropDatabase,
};

mod create_database;
mod drop_database;

/// Represents an AST node for a database operation.
#[derive(Debug, PartialEq, Clone)]
pub enum DatabaseNode {
    /// Represents a `DROP DATABASE ...` statement.
    DropDatabase(DropDatabase),
    
    /// Represents a `CREATE DATABASE ...` statement.
    CreateDatabase(CreateDatabase),
}
