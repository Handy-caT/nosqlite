mod alter;
mod create;
mod drop;
mod r#use;

use derive_more::Display;

use crate::{parser::Statement, preprocessor::Node};

pub use alter::AlterSchema;
pub use create::CreateSchema;
pub use drop::DropSchema;
pub use r#use::UseSchema;

/// Represents an AST node for a schema operation.
#[derive(Debug, Display, PartialEq, Clone)]
pub enum SchemaNode {
    /// Represents a `DROP SCHEMA ...` statement.
    Drop(DropSchema),

    /// Represents a `CREATE SCHEMA ...` statement.
    Create(CreateSchema),

    /// Represents a `ALTER SCHEMA ...` statement.
    Alter(AlterSchema),

    /// Represents a `USE SCHEMA ...` statement.
    Use(UseSchema),
}

impl Node for SchemaNode {
    fn can_be_followed(&self, next: &Statement) -> bool {
        match self {
            SchemaNode::Drop(stmnt) => stmnt.can_be_followed(next),
            SchemaNode::Create(stmnt) => stmnt.can_be_followed(next),
            SchemaNode::Alter(stmnt) => stmnt.can_be_followed(next),
            SchemaNode::Use(stmnt) => stmnt.can_be_followed(next),
        }
    }
}

/// Shortcut for a [`SchemaNode`] variant of [`Statement`].
#[macro_export]
macro_rules! schema_statement_variant {
    ($($arg:tt)*) => {
        $crate::parser::Statement::Dml(
            $crate::parser::statement::DML::Schema(
                    $($arg)*
            )
        )
    };
}
