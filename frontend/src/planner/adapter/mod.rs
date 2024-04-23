use crate::{lexer::token, planner::command::FrontendCommand};
use backend::schema::database as db;
use backend_api::api::command::r#enum::BackendCommand;
use derive_more::From;

mod database;

/// Parses an identifier into a vector of names.
/// # Arguments
/// * `identifier` - An identifier.
/// # Returns
/// A vector of names.
pub fn parse_identifier(identifier: token::Identifier) -> Vec<String> {
    identifier.0.split('.').map(String::from).collect()
}

/// Represents a planner command.
#[derive(Debug, Clone, From, PartialEq)]
pub enum PlannerCommand {
    /// Represents a backend command.
    Backend(BackendCommand),

    /// Represents a frontend command.
    Frontend(FrontendCommand),
}
