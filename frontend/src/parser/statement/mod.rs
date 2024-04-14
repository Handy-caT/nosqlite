pub mod common;
pub mod dml;

pub use common::Common;
pub use dml::DML;

/// Represents an AST node for a statement.
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    /// Represents a DML operation.
    Dml(DML),

    /// Represents a common operation.
    Common(Common),
}
