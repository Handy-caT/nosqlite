use crate::{
    alter_schema_statement_variant, create_schema_statement_variant,
    drop_schema_statement_variant,
    parser::ast,
    planner::adapter::{
        parse_identifier, IdentifierMismatchError, ParseError,
        WrongIdentifierError,
    },
    rename_to_statement_variant, show_schemas_statement_variant,
};
use backend::schema;
use backend_api::api::command::database::{
    CreateSchema, DropSchema, RenameSchema, ShowSchemas,
};

impl TryFrom<ast::Node> for CreateSchema {
    type Error = ParseError;

    fn try_from(node: ast::Node) -> Result<Self, Self::Error> {
        if let create_schema_statement_variant!(statement) = node.statement {
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

            Ok(CreateSchema {
                database_name: db_name,
                name,
            })
        } else {
            Err(ParseError::UnexpectedStatement(node.statement))
        }
    }
}

impl TryFrom<ast::Node> for DropSchema {
    type Error = ParseError;

    fn try_from(node: ast::Node) -> Result<Self, Self::Error> {
        if let drop_schema_statement_variant!(statement) = node.statement {
            let mut names =
                parse_identifier(statement.identifier.clone()).into_iter();
            let name = names
                .next()
                .ok_or(ParseError::WrongIdentifier(WrongIdentifierError {
                    got: statement.identifier.clone(),
                    expected_type: "`schema_name`",
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

            Ok(DropSchema {
                database_name: db_name,
                name,
            })
        } else {
            Err(ParseError::UnexpectedStatement(node.statement))
        }
    }
}

impl TryFrom<ast::Node> for RenameSchema {
    type Error = ParseError;

    fn try_from(node: ast::Node) -> Result<Self, Self::Error> {
        let (schema_name_from, db_name) =
            if let alter_schema_statement_variant!(statement) = &node.statement
            {
                let mut names =
                    parse_identifier(statement.identifier.clone()).into_iter();
                let name: schema::Name = names
                    .next()
                    .ok_or(ParseError::WrongIdentifier(WrongIdentifierError {
                        got: statement.identifier.clone(),
                        expected_type: "`schema_name`",
                    }))?
                    .into();
                let db_name = names.next();
                if names.next().is_some() {
                    return Err(ParseError::WrongIdentifier(
                        WrongIdentifierError {
                            got: statement.identifier.clone(),
                            expected_type: "db_name.schema_name",
                        },
                    ));
                }

                (name, db_name)
            } else {
                return Err(ParseError::UnexpectedStatement(node.statement));
            };

        let child = &node.next.expect("is rename to statement and exist");
        let (schema_name_to, db_name_to) =
            if let rename_to_statement_variant!(statement) = &child.statement {
                let mut names =
                    parse_identifier(statement.identifier.clone()).into_iter();
                let name: schema::Name = names
                    .next()
                    .ok_or(ParseError::WrongIdentifier(WrongIdentifierError {
                        got: statement.identifier.clone(),
                        expected_type: "`schema_name`",
                    }))?
                    .into();
                let db_name = names.next();
                if names.next().is_some() {
                    return Err(ParseError::WrongIdentifier(
                        WrongIdentifierError {
                            got: statement.identifier.clone(),
                            expected_type: "db_name.schema_name",
                        },
                    ));
                }

                (name, db_name)
            } else {
                return Err(ParseError::UnexpectedStatement(node.statement));
            };

        match (db_name, db_name_to) {
            (Some(db_name), Some(db_name_to)) => {
                if db_name != db_name_to {
                    Err(ParseError::IdentifierMismatch(
                        IdentifierMismatchError {
                            got: db_name_to,
                            expected: db_name,
                        },
                    ))
                } else {
                    Ok(RenameSchema {
                        database_name: Some(db_name.into()),
                        old_name: schema_name_from,
                        new_name: schema_name_to,
                    })
                }
            }
            (Some(db_name), None) => Ok(RenameSchema {
                database_name: Some(db_name.into()),
                old_name: schema_name_from,
                new_name: schema_name_to,
            }),
            (None, None) => Ok(RenameSchema {
                database_name: None,
                old_name: schema_name_from,
                new_name: schema_name_to,
            }),
            _ => Err(ParseError::UnexpectedStatement(node.statement)),
        }
    }
}

impl TryFrom<ast::Node> for ShowSchemas {
    type Error = ParseError;

    fn try_from(node: ast::Node) -> Result<Self, Self::Error> {
        if let show_schemas_statement_variant!(statement) = node.statement {
            let names =
                parse_identifier(statement.identifier.clone()).into_iter();
            if names.len() != 1 {
                return Err(ParseError::WrongIdentifier(
                    WrongIdentifierError {
                        got: statement.identifier,
                        expected_type: "db_name",
                    },
                ));
            }
            let name = names.into_iter().next().expect("names is not empty");

            Ok(ShowSchemas {
                database_name: Some(name.into()),
            })
        } else {
            Err(ParseError::UnexpectedStatement(node.statement))
        }
    }
}
