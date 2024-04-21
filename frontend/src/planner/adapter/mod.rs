use crate::{lexer::token, planner::command::FrontendCommand};
use backend::schema::database as db;
use backend_api::api::command::r#enum::BackendCommand;
use derive_more::From;

mod database;

impl From<token::Identifier> for db::Name {
    fn from(identifier: token::Identifier) -> Self {
        db::Name(identifier.0)
    }
}

/// Represents a planner command.
#[derive(Debug, Clone, From, PartialEq)]
pub enum PlannerCommand {
    /// Represents a backend command.
    Backend(BackendCommand),

    /// Represents a frontend command.
    Frontend(FrontendCommand),
}
