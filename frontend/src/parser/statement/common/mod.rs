mod rename_to;

pub use rename_to::RenameTo;

/// Represents an AST node for a common operation.
#[derive(Debug, PartialEq, Clone)]
pub enum Common {
    /// Represents a `RENAME TO ...` statement.
    RenameTo(RenameTo),
}
