//! Keyword token module of the lexer.

/// Represents a keyword in the SQL language for the database objects.
#[derive(Debug, PartialEq)]
pub enum DBObject {
    /// Token for `DATABASE` object.
    Database,
    
    /// Token for `SCHEMA` object.
    Schema,
    
    /// Token for `TABLE` object.
    Table,
    
    /// Token for `COLUMN` object.
    Column,
}