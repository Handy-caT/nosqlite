pub mod ast;
mod parsers;
pub mod statement;

pub use statement::Statement;

use crate::{
    lexer::{
        token::{Delimiter, Token},
        Lexer,
    },
    parser::parsers::{
        DmlParseError, DmlParser, ParenthesisParseError, ParenthesisParser,
        ShortcutParseError, ShortcutParser,
    },
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

    /// Represents the parenthesis state.
    parenthesis_state: ParenthesisState,
}

/// Represents a parenthesis state.
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ParenthesisState {
    /// Represents the parenthesis opened.
    pub opened: Vec<()>,

    /// Indicates whether the last token is a comma.
    pub is_last_comma: bool,
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
            parenthesis_state: ParenthesisState::default(),
        }
    }

    pub fn parse_statement(&mut self) -> Option<Result<Statement, ParseError>> {
        let token = if self.peek_token.is_some() {
            self.peek_token.take()
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
                Token::Delimiter(Delimiter::LeftParenthesis)
                | Token::Delimiter(Delimiter::Comma) => {
                    self.state.push(token);
                    let mut parenthesis_parser = ParenthesisParser::new(
                        &mut self.lexer,
                        &mut self.state,
                        &mut self.peek_token,
                        &mut self.parenthesis_state,
                    );
                    let statement = parenthesis_parser.parse();

                    if statement.is_none() {
                        self.state.clear();
                        self.parse_statement()
                    } else {
                        let result = statement
                            .expect("exist")
                            .map_err(ParseError::ParenthesisParseError);
                        self.state.clear();
                        Some(result)
                    }
                }
                Token::Delimiter(Delimiter::RightParenthesis) => {
                    self.parenthesis_state.opened.pop();
                    self.parse_statement()
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
                    let mut shortcut_parser =
                        ShortcutParser::new(&mut self.lexer, &mut self.state);
                    let statement = shortcut_parser
                        .parse()
                        .map_err(ParseError::ShortcutParseError);

                    self.state.clear();
                    Some(statement)
                }
                _ => Some(Err(ParseError::UnexpectedToken(token))),
            }
        } else if self.parenthesis_state.opened.is_empty() {
            None
        } else {
            Some(Err(ParseError::ParenthesisNotClosed))
        }
    }
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
    /// Represents a parenthesis not closed.
    ParenthesisNotClosed,

    /// Represents a wrong token provided.
    UnexpectedToken(Token),

    /// Represents a DML parser fails.
    DmlParseError(DmlParseError),

    /// Represents a Shortcut parser fails.
    ShortcutParseError(ShortcutParseError),

    /// Represents a Parenthesis parser fails.
    ParenthesisParseError(ParenthesisParseError),
}

#[cfg(test)]
mod test {
    use crate::{
        lexer::token::{DataType, Token},
        parser::statement::{
            common::{Column, RenameTo},
            dml::{
                AlterSchema, CreateDatabase, CreateSchema, CreateTable,
                DropDatabase, DropSchema,
            },
        },
    };

    use super::{ParseError, Parser};

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

    #[test]
    fn parse_create_table_statement() {
        let input = "CREATE TABLE test (id INTEGER PRIMARY KEY)";

        let mut parser = Parser::new(input);
        let statement = parser.next();

        assert!(statement.is_some());
        let statement = statement.unwrap();
        assert!(statement.is_ok());
        let statement = statement.unwrap();

        assert_eq!(
            statement,
            CreateTable::new_statement("test".to_string().into())
        );

        let statement = parser.next();

        assert!(statement.is_some());
        let statement = statement.unwrap();
        assert!(statement.is_ok());
        let statement = statement.unwrap();

        assert_eq!(
            statement,
            Column::new_statement(Column {
                identifier: "id".to_string().into(),
                data_type: DataType::Integer,
                is_primary_key: true,
            })
        );

        let statement = parser.next();
        assert!(statement.is_none());
    }

    #[test]
    fn parse_create_table_statement_not_closed() {
        let input = "CREATE TABLE test (id INTEGER PRIMARY KEY";

        let mut parser = Parser::new(input);
        let statement = parser.next();

        assert!(statement.is_some());
        let statement = statement.unwrap();
        assert!(statement.is_ok());
        let statement = statement.unwrap();

        assert_eq!(
            statement,
            CreateTable::new_statement("test".to_string().into())
        );

        let statement = parser.next();

        assert!(statement.is_some());
        let statement = statement.unwrap();
        assert!(statement.is_ok());
        let statement = statement.unwrap();

        assert_eq!(
            statement,
            Column::new_statement(Column {
                identifier: "id".to_string().into(),
                data_type: DataType::Integer,
                is_primary_key: true,
            })
        );

        let statement = parser.next();
        assert!(statement.is_some());
        let statement = statement.unwrap();
        assert!(statement.is_err());
        assert_eq!(statement, Err(ParseError::ParenthesisNotClosed));
    }

    #[test]
    fn parse_create_table_statement_not_opened() {
        let input = "CREATE TABLE test id INTEGER PRIMARY KEY)";

        let mut parser = Parser::new(input);
        let statement = parser.next();

        assert!(statement.is_some());
        let statement = statement.unwrap();
        assert!(statement.is_ok());
        let statement = statement.unwrap();

        assert_eq!(
            statement,
            CreateTable::new_statement("test".to_string().into())
        );

        let statement = parser.next();

        assert!(statement.is_some());
        let statement = statement.unwrap();
        assert!(statement.is_err());
        assert_eq!(
            statement,
            Err(ParseError::UnexpectedToken(Token::Identifier(
                "id".to_string().into()
            )))
        );
    }

    #[test]
    fn parse_create_table_statement_many_columns() {
        let input = "CREATE TABLE test (id INTEGER PRIMARY KEY,\
                                             name VARCHAR10)";

        let mut parser = Parser::new(input);
        let statement = parser.next();

        assert!(statement.is_some());
        let statement = statement.unwrap();
        assert!(statement.is_ok());
        let statement = statement.unwrap();

        assert_eq!(
            statement,
            CreateTable::new_statement("test".to_string().into())
        );

        let statement = parser.next();

        assert!(statement.is_some());
        let statement = statement.unwrap();
        assert!(statement.is_ok());
        let statement = statement.unwrap();

        assert_eq!(
            statement,
            Column::new_statement(Column {
                identifier: "id".to_string().into(),
                data_type: DataType::Integer,
                is_primary_key: true,
            })
        );

        let statement = parser.next();

        assert!(statement.is_some());
        let statement = statement.unwrap();
        assert!(statement.is_ok());
        let statement = statement.unwrap();

        assert_eq!(
            statement,
            Column::new_statement(Column {
                identifier: "name".to_string().into(),
                data_type: DataType::VarChar(10),
                is_primary_key: false,
            })
        );

        let statement = parser.next();
        assert!(statement.is_none());
    }

    #[test]
    fn parse_create_table_statement_comma_before_close() {
        let input = "CREATE TABLE test (id INTEGER PRIMARY KEY,)";

        let mut parser = Parser::new(input);
        let statement = parser.next();

        assert!(statement.is_some());
        let statement = statement.unwrap();
        assert!(statement.is_ok());
        let statement = statement.unwrap();

        assert_eq!(
            statement,
            CreateTable::new_statement("test".to_string().into())
        );

        let statement = parser.next();

        assert!(statement.is_some());
        let statement = statement.unwrap();
        assert!(statement.is_ok());
        let statement = statement.unwrap();

        assert_eq!(
            statement,
            Column::new_statement(Column {
                identifier: "id".to_string().into(),
                data_type: DataType::Integer,
                is_primary_key: true,
            })
        );

        let statement = parser.next();
        assert!(statement.is_none());
    }
}
