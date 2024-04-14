mod rename_to;

use crate::{parser::Statement, preprocessor::Node};

pub use rename_to::RenameTo;

/// Represents an AST node for a common operation.
#[derive(Debug, PartialEq, Clone)]
pub enum Common {
    /// Represents a `RENAME TO ...` statement.
    RenameTo(RenameTo),
}

impl Node for Common {
    fn can_be_followed(&self, next: &Statement) -> bool {
        match self {
            Common::RenameTo(stmnt) => stmnt.can_be_followed(next),
        }
    }
}
