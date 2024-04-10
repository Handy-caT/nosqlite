mod ddl_operator;
mod delimiter;
mod dml_operator;
mod keyword;

pub use ddl_operator::DDLOperator;
pub use delimiter::Delimiter;
pub use dml_operator::DMLOperator;
pub use keyword::{DBObject, Keyword};

/// Identifier is a token that represents an identifier in the source code.
#[derive(Debug, PartialEq, Clone)]
pub struct Identifier(pub String);

/// Token is a part of lexer that represents a single unit of the source code.
#[derive(Debug, Clone, PartialEq)]
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
}
