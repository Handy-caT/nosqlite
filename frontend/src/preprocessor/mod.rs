use crate::parser::ast::Statement;

/// Represents an AST leaf node.
pub trait LeafNode {}

/// Represents an AST node.
pub trait Node {
    /// Determines if the node can be followed by another statement.
    /// # Arguments
    /// * `other` - The statement to check.
    /// # Returns
    /// `true` if the node can be followed by the statement, otherwise `false`.
    fn can_be_followed(&self, other: Statement) -> bool;
}

impl<T> Node for T
where
    T: LeafNode,
{
    fn can_be_followed(&self, _: Statement) -> bool {
        false
    }
}