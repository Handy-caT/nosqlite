use crate::{
    column_statement_variant,
    lexer::{
        token::{Delimiter, Key, Keyword, Token},
        Lexer,
    },
    parser::{ParenthesisState, Statement},
};
use std::mem;

/// Represents a Parenthesis parser.
#[derive(Debug, PartialEq)]
pub struct ParenthesisParser<'a> {
    /// Represents the lexer.
    lexer: &'a mut Lexer,

    /// Represents the state of the parser.
    state: &'a mut Vec<Token>,

    /// Represents the [`Token`] peeked.
    peek_token: &'a mut Option<Token>,

    /// Represents the parenthesis state.
    parenthesis_state: &'a mut ParenthesisState,
}

impl<'a> ParenthesisParser<'a> {
    /// Creates a new Parenthesis parser.
    pub fn new(
        lexer: &'a mut Lexer,
        state: &'a mut Vec<Token>,
        peek_token: &'a mut Option<Token>,
        parenthesis_state: &'a mut ParenthesisState,
    ) -> Self {
        Self {
            lexer,
            state,
            peek_token,
            parenthesis_state,
        }
    }

    /// Parses an operation in parentheses.
    pub fn parse(&mut self) -> Option<Result<Statement, ParseError>> {
        let token = self
            .state
            .pop()
            .expect("exist because passed from `Parser`");

        if let Token::Delimiter(Delimiter::LeftParenthesis) = token {
            self.parenthesis_state.opened.push(());
        }

        let mut maybe_identifier_token = self.lexer.next();
        match maybe_identifier_token.clone() {
            Some(Token::Identifier(_)) => {
                mem::swap(self.peek_token, &mut maybe_identifier_token);
            }
            Some(delimeter) => {
                return match delimeter {
                    Token::Delimiter(Delimiter::RightParenthesis) => {
                        self.parenthesis_state.opened.pop();
                        if let Token::Delimiter(Delimiter::LeftParenthesis) =
                            token
                        {
                            return Some(Err(ParseError::UnexpectedToken(
                                delimeter,
                            )));
                        }
                        None
                    }
                    _ => Some(Err(ParseError::UnexpectedToken(delimeter))),
                }
            }
            None => {
                return Some(Err(ParseError::NotEnoughTokens));
            }
        }

        Some(self.parse_column())
    }

    /// Parses a column statement.
    pub fn parse_column(&mut self) -> Result<Statement, ParseError> {
        let identifier_token = if self.peek_token.is_some() {
            self.peek_token.take().expect("exist because checked")
        } else {
            self.lexer.next().ok_or(ParseError::NotEnoughTokens)?
        };

        let datatype_token =
            self.lexer.next().ok_or(ParseError::NotEnoughTokens)?;

        self.state.push(identifier_token);
        self.state.push(datatype_token);

        let mut primary_key_token = self.lexer.next();
        match primary_key_token {
            Some(Token::Keyword(Keyword::Key(Key::Primary))) => {
                let key_token =
                    self.lexer.next().ok_or(ParseError::NotEnoughTokens)?;
                if let Token::Keyword(Keyword::Key(Key::Key)) = key_token {
                    self.state.push(
                        primary_key_token.expect("exist because checked"),
                    );
                    self.state.push(key_token);
                } else {
                    return Err(ParseError::WrongTokenProvided {
                        got: key_token,
                        expected: "`key`".to_string(),
                    });
                }
            }
            Some(_) => {
                mem::swap(self.peek_token, &mut primary_key_token);
            }
            _ => {}
        };

        Ok(column_statement_variant!(self
            .state
            .as_slice()
            .try_into()
            .expect("valid tokens")))
    }
}

/// Error of [`DmlParser`] execution.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// Wrong token provided to the [`DmlParser`].
    WrongTokenProvided { got: Token, expected: String },

    /// Not enough tokens got from the [`Lexer`].
    NotEnoughTokens,

    /// Unexpected token.
    UnexpectedToken(Token),

    /// Expected identifier token but got something else.
    ExpectedIdentifier(Token),
}

#[cfg(test)]
mod test {
    use crate::{
        lexer::{
            token::{DataType, Delimiter, Identifier, Token},
            Lexer,
        },
        parser::{statement::common::Column, ParenthesisState},
    };

    use super::{ParenthesisParser, ParseError};

    #[test]
    #[should_panic]
    fn test_no_tokens_in_state() {
        let mut lexer = Lexer::new("CREATE DATABASE test");
        let mut state = vec![];
        let mut peek_token = None;
        let mut parenthesis_state = ParenthesisState::default();

        let mut parser = ParenthesisParser::new(
            &mut lexer,
            &mut state,
            &mut peek_token,
            &mut parenthesis_state,
        );

        let _ = parser.parse();
    }

    #[test]
    fn test_column_statement() {
        let mut lexer = Lexer::new("(id INTEGER PRIMARY KEY");
        let mut state = vec![lexer.next().unwrap()];
        let mut peek_token = None;
        let mut parenthesis_state = ParenthesisState::default();

        let mut parser = ParenthesisParser::new(
            &mut lexer,
            &mut state,
            &mut peek_token,
            &mut parenthesis_state,
        );

        let statement = parser.parse();

        assert_eq!(
            statement,
            Some(Ok(Column::new_statement(Column {
                identifier: Identifier("id".to_string()),
                data_type: DataType::Integer,
                is_primary_key: true,
            })))
        );
        assert_eq!(parenthesis_state.opened.len(), 1);
    }

    #[test]
    fn test_column_no_primary_key_statement() {
        let mut lexer = Lexer::new("(id INTEGER,");
        let mut state = vec![lexer.next().unwrap()];
        let mut peek_token = None;
        let mut parenthesis_state = ParenthesisState::default();

        let mut parser = ParenthesisParser::new(
            &mut lexer,
            &mut state,
            &mut peek_token,
            &mut parenthesis_state,
        );

        let statement = parser.parse();

        assert_eq!(
            statement,
            Some(Ok(Column::new_statement(Column {
                identifier: Identifier("id".to_string()),
                data_type: DataType::Integer,
                is_primary_key: false,
            })))
        );
        assert_eq!(parenthesis_state.opened.len(), 1);
        assert!(peek_token.is_some());
        assert_eq!(peek_token, Some(Token::Delimiter(Delimiter::Comma)))
    }

    #[test]
    fn test_column_no_primary_key_statement_comma() {
        let mut lexer = Lexer::new(",id INTEGER,");
        let mut state = vec![lexer.next().unwrap()];
        let mut peek_token = None;
        let mut parenthesis_state = ParenthesisState::default();

        let mut parser = ParenthesisParser::new(
            &mut lexer,
            &mut state,
            &mut peek_token,
            &mut parenthesis_state,
        );

        let statement = parser.parse();

        assert_eq!(
            statement,
            Some(Ok(Column::new_statement(Column {
                identifier: Identifier("id".to_string()),
                data_type: DataType::Integer,
                is_primary_key: false,
            })))
        );
        assert_eq!(parenthesis_state.opened.len(), 0);
        assert!(peek_token.is_some());
        assert_eq!(peek_token, Some(Token::Delimiter(Delimiter::Comma)))
    }

    #[test]
    fn test_empty_parenthesis() {
        let mut lexer = Lexer::new("()");
        let mut state = vec![lexer.next().unwrap()];
        let mut peek_token = None;
        let mut parenthesis_state = ParenthesisState::default();

        let mut parser = ParenthesisParser::new(
            &mut lexer,
            &mut state,
            &mut peek_token,
            &mut parenthesis_state,
        );

        let statement = parser.parse();

        assert_eq!(
            statement,
            Some(Err(ParseError::UnexpectedToken(Token::Delimiter(
                Delimiter::RightParenthesis
            ))))
        );
    }

    #[test]
    fn test_comma_after_left_parenthesis() {
        let mut lexer = Lexer::new("(,");
        let mut state = vec![lexer.next().unwrap()];
        let mut peek_token = None;
        let mut parenthesis_state = ParenthesisState::default();

        let mut parser = ParenthesisParser::new(
            &mut lexer,
            &mut state,
            &mut peek_token,
            &mut parenthesis_state,
        );

        let statement = parser.parse();

        assert_eq!(
            statement,
            Some(Err(ParseError::UnexpectedToken(Token::Delimiter(
                Delimiter::Comma
            ))))
        );
        assert_eq!(parenthesis_state.opened.len(), 1);
        assert_eq!(peek_token, None);
    }

    #[test]
    fn test_right_parenthesis_after_comma() {
        let mut lexer = Lexer::new(",)");
        let mut state = vec![lexer.next().unwrap()];
        let mut peek_token = None;
        let mut parenthesis_state = ParenthesisState::default();

        let mut parser = ParenthesisParser::new(
            &mut lexer,
            &mut state,
            &mut peek_token,
            &mut parenthesis_state,
        );

        let statement = parser.parse();

        assert_eq!(statement, None);
        assert_eq!(parenthesis_state.opened.len(), 0);
        assert_eq!(peek_token, None);
    }
}
