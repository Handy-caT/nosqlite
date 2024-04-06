//! Delimiter token module of the lexer.

/// Delimiter is a token that represents a delimiter in the source code.
#[derive(Debug, PartialEq)]
pub enum Delimiter {
    /// Token for `,` delimiter.
    Comma,
    
    /// Token for `;` delimiter.
    Semicolon,
    
    /// Token for `.` delimiter.
    Dot,
    
    /// Token for `(` delimiter.
    LeftParenthesis,
    
    /// Token for `)` delimiter.
    RightParenthesis,
}