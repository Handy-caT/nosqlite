mod database;
mod schema;

use crate::{parser::Statement, preprocessor::Node};

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

impl Node for DML {
    fn can_be_followed(&self, next: &Statement) -> bool {
        match self {
            DML::Database(stmnt) => stmnt.can_be_followed(next),
            DML::Schema(stmnt) => stmnt.can_be_followed(next),
        }
    }
}
