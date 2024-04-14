mod alter_schema;
mod create_schema;
mod drop_schema;

use crate::{parser::Statement, preprocessor::Node};

pub use alter_schema::AlterSchema;
pub use create_schema::CreateSchema;
pub use drop_schema::DropSchema;

/// Represents an AST node for a schema operation.
#[derive(Debug, PartialEq, Clone)]
pub enum SchemaNode {
    /// Represents a `DROP SCHEMA ...` statement.
    Drop(DropSchema),

    /// Represents a `CREATE SCHEMA ...` statement.
    Create(CreateSchema),

    /// Represents a `ALTER SCHEMA ...` statement.
    Alter(AlterSchema),
}

impl Node for SchemaNode {
    fn can_be_followed(&self, next: &Statement) -> bool {
        match self {
            SchemaNode::Drop(stmnt) => stmnt.can_be_followed(next),
            SchemaNode::Create(stmnt) => stmnt.can_be_followed(next),
            SchemaNode::Alter(stmnt) => stmnt.can_be_followed(next),
        }
    }
}
