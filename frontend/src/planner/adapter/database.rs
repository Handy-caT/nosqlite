use backend_api::api::command::backend_api::{CreateDatabase, DropDatabase};

use crate::{
    create_database_statement_variant, drop_database_statement_variant,
    lexer::token, parser::ast, planner::adapter::parse_identifier,
};

impl TryFrom<ast::Node> for CreateDatabase {
    type Error = ParseError;

    fn try_from(node: ast::Node) -> Result<Self, Self::Error> {
        if let create_database_statement_variant!(statement) = node.statement {
            let names = parse_identifier(statement.identifier.clone());
            if names.len() != 1 {
                return Err(ParseError::WrongIdentifier(
                    WrongIdentifierError {
                        got: statement.identifier,
                        expected_type: "`db_name`",
                    },
                ));
            }
            let name = names.into_iter().next().expect("names is not empty");

            Ok(CreateDatabase { name: name.into() })
        } else {
            Err(ParseError::UnexpectedStatement)
        }
    }
}

impl TryFrom<ast::Node> for DropDatabase {
    type Error = ParseError;

    fn try_from(node: ast::Node) -> Result<Self, Self::Error> {
        if let drop_database_statement_variant!(statement) = node.statement {
            let names = parse_identifier(statement.identifier.clone());
            if names.len() != 1 {
                return Err(ParseError::WrongIdentifier(
                    WrongIdentifierError {
                        got: statement.identifier,
                        expected_type: "`db_name`",
                    },
                ));
            }
            let name = names.into_iter().next().expect("names is not empty");

            Ok(DropDatabase { name: name.into() })
        } else {
            Err(ParseError::UnexpectedStatement)
        }
    }
}

/// Error that can occur during parsing.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// Error of wrong identifier type.
    WrongIdentifier(WrongIdentifierError),

    /// Error of unexpected statement.
    UnexpectedStatement,
}

/// Error of wrong identifier type.
#[derive(Debug, Clone, PartialEq)]
pub struct WrongIdentifierError {
    /// Provided token.
    pub got: token::Identifier,

    /// Expected type.
    pub expected_type: &'static str,
}
