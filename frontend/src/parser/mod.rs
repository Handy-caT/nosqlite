pub mod ast;
mod parsers;
pub mod statement;

pub use statement::Statement;

use crate::{
    lexer::{
        token::{Delimiter, Token},
        Lexer,
    },
    parser::{
        ast::Ast,
        parsers::{DmlParseError, DmlParser},
    },
};

/// Represents a parser.
#[derive(Debug, PartialEq, Clone)]
pub struct Parser {
    /// Represents the lexer.
    lexer: Lexer,

    /// Represents the state of the parser.
    state: Vec<Token>,

    /// Represents the AST.
    ast: Ast,
}

impl Parser {
    /// Creates a new parser.
    pub fn new(lexer: Lexer) -> Self {
        Self {
            lexer,
            state: Vec::new(),
            ast: Ast::new(),
        }
    }

    pub fn parse_statement(&mut self) -> Result<Option<Statement>, ParseError> {
        let token = self.lexer.next();
        if let Some(token) = token {
            self.state.push(token.clone());
            match token {
                Token::Delimiter(Delimiter::Semicolon) => {
                    self.parse_statement()
                }
                Token::DML(_) => {
                    let mut dml_parser =
                        DmlParser::new(&mut self.lexer, &mut self.state);
                    let statement = dml_parser
                        .parse()
                        .map_err(ParseError::DmlParseError)?;
                    Ok(Some(statement))
                }
                _ => {
                    todo!()
                }
            }
        } else {
            Ok(None)
        }
    }

    pub fn parse_dml(&mut self) {}
}

/// Represents a parse error.
#[derive(Debug, PartialEq, Clone)]
pub enum ParseError {
    /// Represents an DML parser fails.
    DmlParseError(DmlParseError),
}

#[cfg(test)]
mod test {
    use crate::{
        lexer::Lexer,
        parser::statement::dml::{CreateDatabase, CreateSchema},
    };

    use super::Parser;

    #[test]
    fn parse_create_database_statement() {
        let input = "CREATE DATABASE test;";
        let lexer = Lexer::new(input);

        let mut parser = Parser::new(lexer);
        let statement = parser.parse_statement();

        assert!(statement.is_ok());
        let statement = statement.unwrap();
        assert!(statement.is_some());
        let statement = statement.unwrap();

        assert_eq!(
            statement,
            CreateDatabase::new_statement("test".to_string().into())
        );

        let statement = parser.parse_statement();
        assert!(statement.is_ok());
        let statement = statement.unwrap();
        assert!(statement.is_none());
    }

    #[test]
    fn parse_create_schema_statement() {
        let input = "CREATE SCHEMA test;";
        let lexer = Lexer::new(input);

        let mut parser = Parser::new(lexer);
        let statement = parser.parse_statement();

        assert!(statement.is_ok());
        let statement = statement.unwrap();
        assert!(statement.is_some());
        let statement = statement.unwrap();

        assert_eq!(
            statement,
            CreateSchema::new_statement("test".to_string().into())
        );

        let statement = parser.parse_statement();
        assert!(statement.is_ok());
        let statement = statement.unwrap();
        assert!(statement.is_none());
    }
}
