use crate::{
    create_database_statement_variant, drop_database_statement_variant,
    parser::ast,
    planner::{adapter::PlannerCommand, PlannerError},
    show_databases_statement_variant, use_database_statement_variant,
    use_schema_statement_variant,
};
use backend_api::api::command::{
    backend_api::DatabaseCommand, database::SchemaCommand,
    r#enum::BackendCommand,
};

/// DatabasePlanner is a planner for database operations.
#[derive(Debug, PartialEq)]
pub struct DatabasePlanner {
    /// The current node.
    node: ast::Node,
}

impl DatabasePlanner {
    /// Creates a new database planner.
    /// # Arguments
    /// * `node` - The current node.
    pub fn new(node: ast::Node) -> Self {
        Self { node }
    }

    /// Parses the command.
    /// # Errors
    /// Returns an error if the statement is not a database statement.
    pub fn parse_command(self) -> Result<PlannerCommand, PlannerError> {
        let node = self.node;

        match &node.statement {
            create_database_statement_variant!(_) => {
                Ok(BackendCommand::Database(DatabaseCommand::Create(
                    node.try_into().map_err(PlannerError::ParseError)?,
                ))
                .into())
            }
            drop_database_statement_variant!(_) => {
                Ok(BackendCommand::Database(DatabaseCommand::Drop(
                    node.try_into().map_err(PlannerError::ParseError)?,
                ))
                .into())
            }
            use_database_statement_variant!(_) => {
                Ok(BackendCommand::Database(DatabaseCommand::Use(
                    node.try_into().map_err(PlannerError::ParseError)?,
                ))
                .into())
            }
            use_schema_statement_variant!(_) => {
                Ok(BackendCommand::Database(DatabaseCommand::UseSchema(
                    node.try_into().map_err(PlannerError::ParseError)?,
                ))
                .into())
            }
            show_databases_statement_variant!(_) => {
                Ok(BackendCommand::Database(DatabaseCommand::ShowDatabases(
                    node.try_into().map_err(PlannerError::ParseError)?,
                ))
                .into())
            }
            _ => Err(PlannerError::UnexpectedStatement(node.statement)),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        create_database_statement_variant, drop_database_statement_variant,
        parser::{
            ast,
            statement::dml::{
                CreateDatabase, DropDatabase, UseDatabase, UseSchema,
            },
        },
        planner::{
            adapter::{ParseError, WrongIdentifierError},
            PlannerError,
        },
        use_database_statement_variant, use_schema_statement_variant,
    };

    use super::DatabasePlanner;

    #[test]
    fn test_create_database_wrong_identifier() {
        let node = ast::Node {
            statement: create_database_statement_variant!(CreateDatabase {
                identifier: "db_name.schema_name.table_name".to_string().into(),
            }),
            next: None,
        };

        let result = DatabasePlanner::new(node).parse_command();

        assert_eq!(
            result,
            Err(PlannerError::ParseError(ParseError::WrongIdentifier(
                WrongIdentifierError {
                    got: "db_name.schema_name.table_name".to_string().into(),
                    expected_type: "db_name",
                }
            )))
        );
    }

    #[test]
    fn test_drop_database_wrong_identifier() {
        let node = ast::Node {
            statement: drop_database_statement_variant!(DropDatabase {
                identifier: "db_name.schema_name.table_name".to_string().into(),
            }),
            next: None,
        };

        let result = DatabasePlanner::new(node).parse_command();

        assert_eq!(
            result,
            Err(PlannerError::ParseError(ParseError::WrongIdentifier(
                WrongIdentifierError {
                    got: "db_name.schema_name.table_name".to_string().into(),
                    expected_type: "db_name",
                }
            )))
        );
    }

    #[test]
    fn test_use_database_wrong_identifier() {
        let node = ast::Node {
            statement: use_database_statement_variant!(UseDatabase {
                identifier: "db_name.schema_name.table_name".to_string().into(),
            }),
            next: None,
        };

        let result = DatabasePlanner::new(node).parse_command();

        assert_eq!(
            result,
            Err(PlannerError::ParseError(ParseError::WrongIdentifier(
                WrongIdentifierError {
                    got: "db_name.schema_name.table_name".to_string().into(),
                    expected_type: "db_name",
                }
            )))
        );
    }

    #[test]
    fn test_use_schema_wrong_identifier() {
        let node = ast::Node {
            statement: use_schema_statement_variant!(UseSchema {
                identifier: "db_name.schema_name.table_name".to_string().into(),
            }),
            next: None,
        };

        let result = DatabasePlanner::new(node).parse_command();

        assert_eq!(
            result,
            Err(PlannerError::ParseError(ParseError::WrongIdentifier(
                WrongIdentifierError {
                    got: "db_name.schema_name.table_name".to_string().into(),
                    expected_type: "db_name.schema_name",
                }
            )))
        );
    }

    #[test]
    fn test_use_schema_with_db() {
        let node = ast::Node {
            statement: use_schema_statement_variant!(UseSchema {
                identifier: "db_name.schema_name".to_string().into(),
            }),
            next: None,
        };

        let result = DatabasePlanner::new(node).parse_command();

        assert!(result.is_ok());
    }
}
