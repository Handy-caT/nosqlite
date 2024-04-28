use crate::{parser::Statement, preprocessor::Node};

mod create;
mod drop;

pub use create::CreateTable;
pub use drop::DropTable;

/// Represents an AST node for a table operation.
#[derive(Debug, PartialEq, Clone)]
pub enum TableNode {
    /// Represents a `CREATE TABLE ...` statement.
    Create(CreateTable),

    /// Represents a `DROP TABLE ...` statement.
    Drop(DropTable),
}

impl Node for TableNode {
    fn can_be_followed(&self, next: &Statement) -> bool {
        match self {
            TableNode::Create(stmnt) => stmnt.can_be_followed(next),
            TableNode::Drop(stmnt) => stmnt.can_be_followed(next),
        }
    }
}
