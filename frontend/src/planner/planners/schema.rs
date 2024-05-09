use crate::{
    alter_schema_statement_variant, create_schema_statement_variant,
    drop_schema_statement_variant, show_schemas_statement_variant,
};
use backend_api::api::command::{
    database::SchemaCommand, r#enum::BackendCommand,
};

use crate::{
    parser::ast,
    planner::{adapter::PlannerCommand, PlannerError},
};

/// SchemaPlanner is a planner for database operations.
#[derive(Debug, PartialEq)]
pub struct SchemaPlanner {
    /// The current node.
    node: ast::Node,
}

impl SchemaPlanner {
    /// Creates a new database planner.
    /// # Arguments
    /// * `node` - The current node.
    pub fn new(node: ast::Node) -> Self {
        Self { node }
    }

    /// Parses the command.
    /// # Errors
    /// Returns an error if the statement is not a schema statement.
    pub fn parse_command(self) -> Result<PlannerCommand, PlannerError> {
        let node = self.node;

        match &node.statement {
            create_schema_statement_variant!(_) => {
                Ok(BackendCommand::Schema(SchemaCommand::Create(
                    node.try_into().map_err(PlannerError::ParseError)?,
                ))
                .into())
            }
            drop_schema_statement_variant!(_) => {
                Ok(BackendCommand::Schema(SchemaCommand::Drop(
                    node.try_into().map_err(PlannerError::ParseError)?,
                ))
                .into())
            }
            alter_schema_statement_variant!(_) => {
                Ok(BackendCommand::Schema(SchemaCommand::Rename(
                    node.try_into().map_err(PlannerError::ParseError)?,
                ))
                .into())
            }
            show_schemas_statement_variant!(_) => {
                Ok(BackendCommand::Schema(SchemaCommand::Show(
                    node.try_into().map_err(PlannerError::ParseError)?,
                ))
                .into())
            }
            _ => Err(PlannerError::UnexpectedStatement(node.statement)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        alter_schema_statement_variant, create_schema_statement_variant,
        drop_schema_statement_variant,
        parser::{
            ast,
            statement::{
                common::RenameTo,
                dml::{AlterSchema, CreateSchema, DropSchema},
            },
        },
        planner::{
            adapter::{
                IdentifierMismatchError, ParseError, WrongIdentifierError,
            },
            PlannerError,
        },
        rename_to_statement_variant,
    };

    use super::SchemaPlanner;

    #[test]
    fn test_create_schema_wrong_identifier() {
        let node = ast::Node {
            statement: create_schema_statement_variant!(CreateSchema {
                identifier: "db_name.schema_name.table_name".to_string().into(),
            }),
            next: None,
        };

        let result = SchemaPlanner::new(node).parse_command();

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
    fn test_drop_schema_wrong_identifier() {
        let node = ast::Node {
            statement: drop_schema_statement_variant!(DropSchema {
                identifier: "db_name.schema_name.table_name".to_string().into(),
            }),
            next: None,
        };

        let result = SchemaPlanner::new(node).parse_command();

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
    fn test_rename_schema_wrong_identifier() {
        let node = ast::Node {
            statement: alter_schema_statement_variant!(AlterSchema {
                identifier: "db_name.schema_name.table_name".to_string().into(),
            }),
            next: Some(Box::new(ast::Node {
                statement: rename_to_statement_variant!(RenameTo {
                    identifier: "new_db_name.new_schema_name"
                        .to_string()
                        .into(),
                }),
                next: None,
            })),
        };

        let result = SchemaPlanner::new(node).parse_command();

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
    fn test_rename_schema_mismatch_identifier() {
        let node = ast::Node {
            statement: alter_schema_statement_variant!(AlterSchema {
                identifier: "db_name.schema_name".to_string().into(),
            }),
            next: Some(Box::new(ast::Node {
                statement: rename_to_statement_variant!(RenameTo {
                    identifier: "new_db_name.new_schema_name"
                        .to_string()
                        .into(),
                }),
                next: None,
            })),
        };

        let result = SchemaPlanner::new(node).parse_command();

        assert_eq!(
            result,
            Err(PlannerError::ParseError(ParseError::IdentifierMismatch(
                IdentifierMismatchError {
                    got: "new_db_name".to_string(),
                    expected: "db_name".to_string(),
                }
            )))
        );
    }
}
