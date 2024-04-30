pub mod common;
pub mod dml;
pub mod shortcut;

use derive_more::Display;

use crate::preprocessor::Node;

pub use common::Common;
pub use dml::DML;
pub use shortcut::Shortcut;

/// Represents an AST node for a statement.
#[derive(Debug, Display, PartialEq, Clone)]
pub enum Statement {
    /// Represents a DML operation.
    Dml(DML),

    /// Represents a common operation.
    Common(Common),

    /// Represents a shortcut for frontend commands.
    Shortcut(Shortcut),

    /// Represents a semicolon to separate statements.
    #[display(fmt = ";")]
    Semicolon,
}

impl Node for Statement {
    fn can_be_followed(&self, next: &Statement) -> bool {
        match self {
            Statement::Dml(stmnt) => stmnt.can_be_followed(next),
            Statement::Common(stmnt) => stmnt.can_be_followed(next),
            Statement::Shortcut(stmnt) => stmnt.can_be_followed(next),
            Statement::Semicolon => true,
        }
    }
}
