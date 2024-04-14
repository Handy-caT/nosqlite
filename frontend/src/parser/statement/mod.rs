pub mod common;
pub mod dml;

use crate::preprocessor::Node;

pub use common::Common;
pub use dml::DML;

/// Represents an AST node for a statement.
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    /// Represents a DML operation.
    Dml(DML),

    /// Represents a common operation.
    Common(Common),

    /// Represents a semicolon to separate statements.
    Semicolon,
}

impl Node for Statement {
    fn can_be_followed(&self, next: &Statement) -> bool {
        match self {
            Statement::Dml(stmnt) => stmnt.can_be_followed(next),
            Statement::Common(stmnt) => stmnt.can_be_followed(next),
            Statement::Semicolon => true,
        }
    }
}
