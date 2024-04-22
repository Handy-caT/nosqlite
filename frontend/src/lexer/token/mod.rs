mod ddl_operator;
mod delimiter;
mod dml_operator;
mod keyword;
mod shortcut;

use derive_more::From;

pub use ddl_operator::DDLOperator;
pub use delimiter::Delimiter;
pub use dml_operator::DMLOperator;
pub use keyword::{DBObject, Keyword, Preposition};
pub use shortcut::Shortcut;

/// Identifier is a token that represents an identifier in the source code.
#[derive(Debug, PartialEq, Clone, From)]
pub struct Identifier(pub String);

impl Default for Identifier {
    fn default() -> Self {
        Self("`identifier`".to_string())
    }
}

/// Token is a part of lexer that represents a single unit of the source code.
#[derive(Debug, Clone, PartialEq, From)]
pub enum Token {
    /// Token for [`DDLOperator`].
    DDL(DDLOperator),

    /// Token for [`DMLOperator`].
    DML(DMLOperator),

    /// Token for [`Keyword`].
    Keyword(Keyword),

    /// Token for [`Delimiter`].
    Delimiter(Delimiter),

    /// Token for [`Identifier`].
    Identifier(Identifier),

    /// Token for [`Shortcut`].
    Shortcut(Shortcut),
}
