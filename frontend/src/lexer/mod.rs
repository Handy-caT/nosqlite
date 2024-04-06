mod token;

/// [`Lexer`] is a struct that represents a lexer for the source code.
#[derive(Debug, PartialEq, Clone)]
pub struct Lexer {
    /// The input source code string.
    input: String,

    /// The current position of the lexer.
    current_position: usize,

    /// The current character of the lexer.
    ch: char,
}
