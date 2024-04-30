mod database;
mod schema;
mod table;

use derive_more::Display;

use crate::{parser::Statement, preprocessor::Node};

pub use database::*;
pub use schema::*;
pub use table::*;

/// Represents an AST node for a DML operation.
#[derive(Debug, Display, PartialEq, Clone)]
pub enum DML {
    /// Represents a database operation.
    Database(DatabaseNode),

    /// Represents a schema operation.
    Schema(SchemaNode),

    /// Represents a table operation.
    Table(TableNode),
}

impl Node for DML {
    fn can_be_followed(&self, next: &Statement) -> bool {
        match self {
            DML::Database(stmnt) => stmnt.can_be_followed(next),
            DML::Schema(stmnt) => stmnt.can_be_followed(next),
            DML::Table(stmnt) => stmnt.can_be_followed(next),
        }
    }
}
