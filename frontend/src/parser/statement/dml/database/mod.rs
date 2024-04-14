mod create_database;
mod drop_database;

use crate::{parser::Statement, preprocessor::Node};

pub use create_database::CreateDatabase;
pub use drop_database::DropDatabase;

/// Represents an AST node for a database operation.
#[derive(Debug, PartialEq, Clone)]
pub enum DatabaseNode {
    /// Represents a `DROP DATABASE ...` statement.
    Drop(DropDatabase),

    /// Represents a `CREATE DATABASE ...` statement.
    Create(CreateDatabase),
}

impl Node for DatabaseNode {
    fn can_be_followed(&self, next: &Statement) -> bool {
        match self {
            DatabaseNode::Drop(stmnt) => stmnt.can_be_followed(next),
            DatabaseNode::Create(stmnt) => stmnt.can_be_followed(next),
        }
    }
}
