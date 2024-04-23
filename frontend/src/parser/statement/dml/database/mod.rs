mod create;
mod drop;
mod r#use;

use crate::{parser::Statement, preprocessor::Node};

pub use create::CreateDatabase;
pub use drop::DropDatabase;
pub use r#use::UseDatabase;

/// Represents an AST node for a database operation.
#[derive(Debug, PartialEq, Clone)]
pub enum DatabaseNode {
    /// Represents a `DROP DATABASE ...` statement.
    Drop(DropDatabase),

    /// Represents a `CREATE DATABASE ...` statement.
    Create(CreateDatabase),

    /// Represents a `USE DATABASE ...` statement.
    Use(UseDatabase),
}

impl Node for DatabaseNode {
    fn can_be_followed(&self, next: &Statement) -> bool {
        match self {
            DatabaseNode::Drop(stmnt) => stmnt.can_be_followed(next),
            DatabaseNode::Create(stmnt) => stmnt.can_be_followed(next),
            DatabaseNode::Use(stmnt) => stmnt.can_be_followed(next),
        }
    }
}
