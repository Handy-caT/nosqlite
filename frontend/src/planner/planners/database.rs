use crate::{create_database_statement_variant, drop_database_statement_variant, parser::ast, planner::{adapter::PlannerCommand, PlannerError}, use_database_statement_variant, use_schema_statement_variant};
use backend_api::api::command::{
    backend_api::DatabaseCommand, r#enum::BackendCommand,
};
use backend_api::api::command::database::SchemaCommand;

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
                    node.try_into().expect("is create database"),
                ))
                .into())
            }
            drop_database_statement_variant!(_) => {
                Ok(BackendCommand::Database(DatabaseCommand::Drop(
                    node.try_into().expect("is drop database"),
                ))
                .into())
            }
            use_database_statement_variant!(_) => Ok(BackendCommand::Database(
                DatabaseCommand::Use(node.try_into().expect("is use database")),
            )
            .into()),
            use_schema_statement_variant!(_) => Ok(BackendCommand::Database(
                DatabaseCommand::UseSchema(node.try_into().expect("is use schema")),
            )
                .into()),
            _ => Err(PlannerError::UnexpectedStatement(node.statement)),
        }
    }
}
