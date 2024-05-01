use backend_api::api::command::backend_api::{
    CreateDatabase, DropDatabase, ShowDatabases, UseDatabase, UseSchema,
};

use crate::{
    create_database_statement_variant, drop_database_statement_variant,
    parser::ast,
    planner::adapter::{parse_identifier, ParseError, WrongIdentifierError},
    show_databases_statement_variant, use_database_statement_variant,
    use_schema_statement_variant,
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
                        expected_type: "db_name",
                    },
                ));
            }
            let name = names.into_iter().next().expect("names is not empty");

            Ok(CreateDatabase { name: name.into() })
        } else {
            Err(ParseError::UnexpectedStatement(node.statement))
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
                        expected_type: "db_name",
                    },
                ));
            }
            let name = names.into_iter().next().expect("names is not empty");

            Ok(DropDatabase { name: name.into() })
        } else {
            Err(ParseError::UnexpectedStatement(node.statement))
        }
    }
}

impl TryFrom<ast::Node> for UseDatabase {
    type Error = ParseError;

    fn try_from(node: ast::Node) -> Result<Self, Self::Error> {
        if let use_database_statement_variant!(statement) = node.statement {
            let names = parse_identifier(statement.identifier.clone());
            if names.len() != 1 {
                return Err(ParseError::WrongIdentifier(
                    WrongIdentifierError {
                        got: statement.identifier,
                        expected_type: "db_name",
                    },
                ));
            }
            let name = names.into_iter().next().expect("names is not empty");

            Ok(UseDatabase { name: name.into() })
        } else {
            Err(ParseError::UnexpectedStatement(node.statement))
        }
    }
}

impl TryFrom<ast::Node> for UseSchema {
    type Error = ParseError;

    fn try_from(node: ast::Node) -> Result<Self, Self::Error> {
        if let use_schema_statement_variant!(statement) = node.statement {
            let mut names =
                parse_identifier(statement.identifier.clone()).into_iter();
            let name = names
                .next()
                .ok_or(ParseError::WrongIdentifier(WrongIdentifierError {
                    got: statement.identifier.clone(),
                    expected_type: "schema_name",
                }))?
                .into();
            let db_name = names.next().map(|name| name.into());
            if names.next().is_some() {
                return Err(ParseError::WrongIdentifier(
                    WrongIdentifierError {
                        got: statement.identifier,
                        expected_type: "db_name.schema_name",
                    },
                ));
            }

            Ok(UseSchema {
                database_name: db_name,
                name,
            })
        } else {
            Err(ParseError::UnexpectedStatement(node.statement))
        }
    }
}

impl TryFrom<ast::Node> for ShowDatabases {
    type Error = ParseError;

    fn try_from(node: ast::Node) -> Result<Self, Self::Error> {
        if let show_databases_statement_variant!(_) = node.statement {
            Ok(ShowDatabases {})
        } else {
            Err(ParseError::UnexpectedStatement(node.statement))
        }
    }
}
