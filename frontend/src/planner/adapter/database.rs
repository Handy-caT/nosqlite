use backend_api::api::command::backend_api::{
    CreateDatabase, DropDatabase, UseDatabase, UseSchema,
};

use crate::{
    create_database_statement_variant, drop_database_statement_variant,
    parser::ast,
    planner::adapter::{parse_identifier, ParseError, WrongIdentifierError},
    use_database_statement_variant, use_schema_statement_variant,
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

impl TryFrom<ast::Node> for UseDatabase {
    type Error = ParseError;

    fn try_from(node: ast::Node) -> Result<Self, Self::Error> {
        if let use_database_statement_variant!(statement) = node.statement {
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

            Ok(UseDatabase { name: name.into() })
        } else {
            Err(ParseError::UnexpectedStatement)
        }
    }
}

impl TryFrom<ast::Node> for UseSchema {
    type Error = ParseError;

    fn try_from(node: ast::Node) -> Result<Self, Self::Error> {
        if let use_schema_statement_variant!(statement) = node.statement {
            let names = parse_identifier(statement.identifier.clone());
            let name = names
                .first()
                .ok_or(ParseError::WrongIdentifier(WrongIdentifierError {
                    got: statement.identifier,
                    expected_type: "`schema_name`",
                }))?
                .to_string();
            let (schema_name, db_name) = if let Some(schema_name) = names.get(1)
            {
                (schema_name.clone(), Some(name.into()))
            } else {
                (name.clone(), None)
            };

            Ok(UseSchema {
                database_name: db_name,
                name: schema_name.into(),
            })
        } else {
            Err(ParseError::UnexpectedStatement)
        }
    }
}
