use crate::{
    alter_schema_statement_variant, create_schema_statement_variant,
    drop_schema_statement_variant, rename_to_statement_variant,
    use_schema_statement_variant,
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
    /// Returns an error if the statement is not a database statement.
    pub fn parse_command(self) -> Result<PlannerCommand, PlannerError> {
        let node = self.node;

        match &node.statement {
            create_schema_statement_variant!(_) => {
                Ok(BackendCommand::Schema(SchemaCommand::Create(
                    node.try_into().expect("is create schema"),
                ))
                .into())
            }
            drop_schema_statement_variant!(_) => Ok(BackendCommand::Schema(
                SchemaCommand::Drop(node.try_into().expect("is drop schema")),
            )
            .into()),
            alter_schema_statement_variant!(_) => {
                let child = &node.next;
                if let Some(child) = child {
                    match child.statement {
                        rename_to_statement_variant!(_) => {
                            Ok(BackendCommand::Schema(SchemaCommand::Rename(
                                node.try_into().expect("is drop schema"),
                            ))
                            .into())
                        }
                        _ => Err(PlannerError::UnexpectedStatement(
                            child.statement.clone(),
                        )),
                    }
                } else {
                    Err(PlannerError::UnexpectedStatement(node.statement))
                }
            }
            _ => Err(PlannerError::UnexpectedStatement(node.statement)),
        }
    }
}
