mod statement;

pub use statement::Statement;

/// Represents an AST node.
pub struct Node {
    /// Represents a statement.
    pub statement: Statement,
    
    /// Represents the next node in the AST.
    pub next: Option<Box<Node>>,
}