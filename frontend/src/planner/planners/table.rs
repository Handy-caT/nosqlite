use crate::{
    create_table_statement_variant, drop_table_statement_variant,
    parser::ast,
    planner::{adapter::PlannerCommand, PlannerError},
};
use backend_api::api::command::{r#enum::BackendCommand, schema::TableCommand};

/// TablePlanner is a planner for database operations.
#[derive(Debug, PartialEq)]
pub struct TablePlanner {
    /// The current node.
    node: ast::Node,
}

impl TablePlanner {
    /// Creates a new table planner.
    /// # Arguments
    /// * `node` - The current node.
    pub fn new(node: ast::Node) -> Self {
        Self { node }
    }

    /// Parses the command.
    /// # Errors
    /// Returns an error if the statement is not a table statement.
    pub fn parse_command(self) -> Result<PlannerCommand, PlannerError> {
        let node = self.node;

        match &node.statement {
            create_table_statement_variant!(_) => Ok(BackendCommand::Table(
                TableCommand::Create(node.try_into().expect("is create table")),
            )
            .into()),
            drop_table_statement_variant!(_) => Ok(BackendCommand::Table(
                TableCommand::Drop(node.try_into().expect("is drop table")),
            )
            .into()),
            _ => Err(PlannerError::UnexpectedStatement(node.statement)),
        }
    }
}
