//! DML operator token module of the lexer.

/// DML operator token.
#[derive(Debug, PartialEq)]
pub enum DMLOperator {
    /// Token for `CREATE` statement.
    Create,
    
    /// Token for `ALTER` statement.
    Alter,

    /// Token for `RENAME` statement.
    Rename,
    
    /// Token for `DROP` statement.
    Drop,
}