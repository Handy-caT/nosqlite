use crate::{
    lexer::{
        token::{Shortcut, Token},
        Lexer,
    },
    parser::{statement::shortcut::Quit, Statement},
};

/// Represents a Shortcut parser.
#[derive(Debug, PartialEq)]
pub struct ShortcutParser<'a> {
    /// Represents the lexer.
    lexer: &'a mut Lexer,

    /// Represents the state of the parser.
    state: &'a mut Vec<Token>,
}

impl<'a> ShortcutParser<'a> {
    /// Creates a new Shortcut parser.
    pub fn new(lexer: &'a mut Lexer, state: &'a mut Vec<Token>) -> Self {
        Self { lexer, state }
    }

    /// Parses a Shortcut operation.
    pub fn parse(&mut self) -> Result<Statement, ParseError> {
        let token = self
            .state
            .last()
            .expect("exist because passed from `Parser`");
        if let Token::Shortcut(token) = token {
            match token {
                Shortcut::Quit => Ok(Quit::new_statement()),
                Shortcut::Help => {
                    todo!()
                }
                Shortcut::Clear => {
                    todo!()
                }
            }
        } else {
            Err(ParseError::WrongTokenProvided {
                got: token.clone(),
                expected: "\\quit|\\help|\\clear".to_string(),
            })
        }
    }
}

/// Error of [`DmlParser`] execution.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// Wrong token provided to the [`DmlParser`].
    WrongTokenProvided { got: Token, expected: String },
}

#[cfg(test)]
mod test {
    use crate::{
        lexer::{
            token::{Keyword, Preposition, Token},
            Lexer,
        },
        parser::statement::shortcut::Quit,
    };

    use super::{ParseError, ShortcutParser};

    #[test]
    #[should_panic]
    fn test_no_tokens_in_state() {
        let mut lexer = Lexer::new("\\quit");
        let mut state = vec![];
        let mut parser = ShortcutParser::new(&mut lexer, &mut state);

        let _ = parser.parse();
    }

    #[test]
    fn test_wrong_tokens() {
        let mut lexer = Lexer::new("TO");
        let mut state = vec![lexer.next().unwrap()];
        let mut parser = ShortcutParser::new(&mut lexer, &mut state);

        let statement = parser.parse();

        assert_eq!(
            statement,
            Err(ParseError::WrongTokenProvided {
                got: Token::Keyword(Keyword::Preposition(Preposition::To)),
                expected: "\\quit|\\help|\\clear".to_string()
            })
        );
    }

    #[test]
    fn test_quit_statement() {
        let mut lexer = Lexer::new("\\quit");
        let mut state = vec![lexer.next().unwrap()];
        let mut parser = ShortcutParser::new(&mut lexer, &mut state);

        let statement = parser.parse();

        assert_eq!(statement, Ok(Quit::new_statement()));
    }
}
