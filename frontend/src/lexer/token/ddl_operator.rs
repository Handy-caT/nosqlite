//! DDL operator token module of the lexer.

/// DDL operator token.
#[derive(Debug, Clone)]
pub enum DDLOperator {
    /// Token for `SELECT` statement.
    Select,
    
    /// Token for `INSERT` statement.
    Insert,
    
    /// Token for `UPDATE` statement.
    Update,
    
    /// Token for `DELETE` statement.
    Delete,
}