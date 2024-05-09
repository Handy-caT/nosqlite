use crate::{
    lexer::{token, token::Token},
    parser::Statement,
    planner::command::FrontendCommand,
};
use backend::schema::database as db;
use backend_api::api::command::r#enum::BackendCommand;
use derive_more::{Display, From};
use std::fmt::Display;

mod data_type;
mod database;
mod schema;
mod table;

/// Parses an identifier into a vector of names.
/// # Arguments
/// * `identifier` - An identifier.
/// # Returns
/// A vector of names.
pub fn parse_identifier(identifier: token::Identifier) -> Vec<String> {
    identifier.0.split('.').map(String::from).rev().collect()
}

/// Represents a planner command.
#[derive(Debug, Clone, From, PartialEq)]
pub enum PlannerCommand {
    /// Represents a backend command.
    Backend(BackendCommand),

    /// Represents a frontend command.
    Frontend(FrontendCommand),
}

/// Error that can occur during parsing.
#[derive(Debug, Clone, From, PartialEq)]
pub enum ParseError {
    /// Error of wrong identifier type.
    WrongIdentifier(WrongIdentifierError),

    /// Error of identifier mismatch.
    IdentifierMismatch(IdentifierMismatchError),

    /// Error of expected [`Token`]s.
    ExpectedTokens(Vec<Token>),

    /// Error of expected statement.
    #[from(ignore)]
    ExpectedStatement(Statement),

    /// Error of unexpected statement.
    UnexpectedStatement(Statement),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::WrongIdentifier(error) => write!(f, "{}", error),
            ParseError::IdentifierMismatch(error) => write!(f, "{}", error),
            ParseError::ExpectedTokens(tokens) => {
                write!(f, "Expected the following tokens: ")?;
                for token in tokens {
                    write!(f, "{} ", token)?;
                }
                Ok(())
            }
            ParseError::UnexpectedStatement(statement) => {
                write!(f, "Unexpected statement: `{}`", statement)
            }
            ParseError::ExpectedStatement(statement) => {
                write!(f, "Expected statement: `{}`", statement)
            }
        }
    }
}

/// Error of wrong identifier type.
#[derive(Debug, Clone, PartialEq)]
pub struct WrongIdentifierError {
    /// Provided token.
    pub got: token::Identifier,

    /// Expected type.
    pub expected_type: &'static str,
}

impl Display for WrongIdentifierError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Wrong identifier: Expected `{}`, but got `{}`",
            self.expected_type, self.got
        )
    }
}

/// Error of identifier mismatch.
#[derive(Debug, Clone, PartialEq)]
pub struct IdentifierMismatchError {
    /// Provided identifier.
    pub got: String,

    /// Expected identifier.
    pub expected: String,
}

impl Display for IdentifierMismatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Identifier mismatch: Expected `{}`, but got `{}`",
            self.expected, self.got
        )
    }
}
