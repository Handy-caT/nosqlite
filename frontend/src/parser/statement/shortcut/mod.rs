mod quit;

use crate::{parser::Statement, preprocessor::Node};

pub use quit::Quit;

/// Represents an AST node for a DML operation.
#[derive(Debug, PartialEq, Clone)]
pub enum Shortcut {
    /// Represents a database operation.
    Quit(Quit),
}

impl Node for Shortcut {
    fn can_be_followed(&self, next: &Statement) -> bool {
        match self {
            Shortcut::Quit(stmnt) => stmnt.can_be_followed(next),
        }
    }
}
