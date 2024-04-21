pub mod ast;
mod parsers;
pub mod statement;

pub use statement::Statement;

use crate::{
    lexer::{
        token::{Delimiter, Token},
        Lexer,
    },
    parser::parsers::{DmlParseError, DmlParser, ShortcutParseError},
};

/// Represents a parser.
#[derive(Debug, PartialEq, Clone)]
pub struct Parser {
    /// Represents the lexer.
    lexer: Lexer,

    /// Represents the state of the parser.
    state: Vec<Token>,

    /// Represents the peek token.
    peek_token: Option<Token>,
}

impl Parser {
    /// Creates a new parser.
    pub fn new<T>(input: T) -> Self
    where
        T: AsRef<str>,
    {
        Self {
            lexer: Lexer::new(input),
            state: Vec::new(),
            peek_token: None,
        }
    }

    pub fn parse_statement(&mut self) -> Option<Result<Statement, ParseError>> {
        let token = if self.peek_token.is_some() {
            self.peek_token.clone()
        } else {
            self.lexer.next()
        };

        if let Some(mut token) = token {
            match token {
                Token::Delimiter(Delimiter::Semicolon) => {
                    let mut tokens_finished = false;
                    while matches!(
                        token,
                        Token::Delimiter(Delimiter::Semicolon)
                    ) && !tokens_finished
                    {
                        if let Some(nex_token) = self.lexer.next() {
                            token = nex_token;
                        } else {
                            tokens_finished = true;
                            self.peek_token = None;
                        }
                    }
                    self.peek_token = Some(token);

                    self.state.clear();
                    Some(Ok(Statement::Semicolon))
                }
                Token::DML(_) => {
                    self.state.push(token);
                    let mut dml_parser =
                        DmlParser::new(&mut self.lexer, &mut self.state);
                    let statement =
                        dml_parser.parse().map_err(ParseError::DmlParseError);

                    self.state.clear();
                    Some(statement)
                }
                Token::Shortcut(_) => {
                    self.state.push(token);
                    let mut shortcut_parser = parsers::ShortcutParser::new(
                        &mut self.lexer,
                        &mut self.state,
                    );
                    let statement = shortcut_parser
                        .parse()
                        .map_err(ParseError::ShortcutParseError);

                    self.state.clear();
                    Some(statement)
                }
                _ => {
                    todo!()
                }
            }
        } else {
            None
        }
    }

    pub fn parse_dml(&mut self) {}
}

impl Iterator for Parser {
    type Item = Result<Statement, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_statement()
    }
}

/// Represents a parse error.
#[derive(Debug, PartialEq, Clone)]
pub enum ParseError {
    /// Represents a DML parser fails.
    DmlParseError(DmlParseError),

    /// Represents a Shortcut parser fails.
    ShortcutParseError(ShortcutParseError),
}

#[cfg(test)]
mod test {
    use crate::{
        lexer::Lexer,
        parser::statement::{
            common::RenameTo,
            dml::{
                AlterSchema, CreateDatabase, CreateSchema, DropDatabase,
                DropSchema,
            },
        },
    };

    use super::Parser;

    #[test]
    fn parse_many_semicolons_statement() {
        let input = "CREATE DATABASE test;;;;;;;";

        let mut parser = Parser::new(input);
        let statement = parser.next();

        assert!(statement.is_some());
        let statement = statement.unwrap();
        assert!(statement.is_ok());
        let statement = statement.unwrap();

        assert_eq!(
            statement,
            CreateDatabase::new_statement("test".to_string().into())
        );

        let statement = parser.next();
        assert!(statement.is_some());
        let statement = statement.unwrap();
        assert!(statement.is_ok());
        let statement = statement.unwrap();

        assert_eq!(statement, super::Statement::Semicolon);
    }

    #[test]
    fn parse_create_database_statement() {
        let input = "CREATE DATABASE test";

        let mut parser = Parser::new(input);
        let statement = parser.next();

        assert!(statement.is_some());
        let statement = statement.unwrap();
        assert!(statement.is_ok());
        let statement = statement.unwrap();

        assert_eq!(
            statement,
            CreateDatabase::new_statement("test".to_string().into())
        );

        let statement = parser.next();
        assert!(statement.is_none());
    }

    #[test]
    fn parse_create_schema_statement() {
        let input = "CREATE SCHEMA test";

        let mut parser = Parser::new(input);
        let statement = parser.next();

        assert!(statement.is_some());
        let statement = statement.unwrap();
        assert!(statement.is_ok());
        let statement = statement.unwrap();

        assert_eq!(
            statement,
            CreateSchema::new_statement("test".to_string().into())
        );

        let statement = parser.next();
        assert!(statement.is_none());
    }

    #[test]
    fn parse_drop_database_statement() {
        let input = "DROP DATABASE test";

        let mut parser = Parser::new(input);
        let statement = parser.next();

        assert!(statement.is_some());
        let statement = statement.unwrap();
        assert!(statement.is_ok());
        let statement = statement.unwrap();

        assert_eq!(
            statement,
            DropDatabase::new_statement("test".to_string().into())
        );

        let statement = parser.next();
        assert!(statement.is_none());
    }

    #[test]
    fn parse_drop_schema_statement() {
        let input = "DROP SCHEMA test";

        let mut parser = Parser::new(input);
        let statement = parser.next();

        assert!(statement.is_some());
        let statement = statement.unwrap();
        assert!(statement.is_ok());
        let statement = statement.unwrap();

        assert_eq!(
            statement,
            DropSchema::new_statement("test".to_string().into())
        );

        let statement = parser.next();
        assert!(statement.is_none());
    }

    #[test]
    fn parse_alter_schema_statement() {
        let input = "ALTER SCHEMA test";

        let mut parser = Parser::new(input);
        let statement = parser.next();

        assert!(statement.is_some());
        let statement = statement.unwrap();
        assert!(statement.is_ok());
        let statement = statement.unwrap();

        assert_eq!(
            statement,
            AlterSchema::new_statement("test".to_string().into())
        );

        let statement = parser.next();
        assert!(statement.is_none());
    }

    #[test]
    fn parse_rename_to_statement() {
        let input = "RENAME TO test";

        let mut parser = Parser::new(input);
        let statement = parser.next();

        assert!(statement.is_some());
        let statement = statement.unwrap();
        assert!(statement.is_ok());
        let statement = statement.unwrap();

        assert_eq!(
            statement,
            RenameTo::new_statement("test".to_string().into())
        );

        let statement = parser.next();
        assert!(statement.is_none());
    }

    #[test]
    fn parse_alter_schema_rename_to_statement() {
        let input = "ALTER SCHEMA test RENAME TO test1";

        let mut parser = Parser::new(input);
        let statement = parser.next();

        assert!(statement.is_some());
        let statement = statement.unwrap();
        assert!(statement.is_ok());
        let statement = statement.unwrap();

        assert_eq!(
            statement,
            AlterSchema::new_statement("test".to_string().into())
        );

        let statement = parser.next();

        assert!(statement.is_some());
        let statement = statement.unwrap();
        assert!(statement.is_ok());
        let statement = statement.unwrap();

        assert_eq!(
            statement,
            RenameTo::new_statement("test1".to_string().into())
        );

        let statement = parser.next();
        assert!(statement.is_none());
    }
}
