mod get_context;
mod quit;

use derive_more::Display;

use crate::{parser::Statement, preprocessor::Node};

pub use get_context::GetContext;
pub use quit::Quit;

/// Represents an AST node for a DML operation.
#[derive(Debug, Display, PartialEq, Clone)]
pub enum Shortcut {
    /// Represents a `quit` operation.
    Quit(Quit),

    /// Represents a `get_context` operation.
    GetContext(GetContext),
}

impl Node for Shortcut {
    fn can_be_followed(&self, next: &Statement) -> bool {
        match self {
            Shortcut::Quit(stmnt) => stmnt.can_be_followed(next),
            Shortcut::GetContext(stmnt) => stmnt.can_be_followed(next),
        }
    }
}
