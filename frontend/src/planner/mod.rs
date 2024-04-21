pub mod adapter;
pub mod command;

use crate::{
    create_database_statement_variant, drop_database_statement_variant,
    planner::{adapter::PlannerCommand, command::FrontendCommand},
    preprocessor::{Preprocessor, PreprocessorError},
    quit_statement_variant,
};
use backend_api::api::command::{
    backend_api::DatabaseCommand, r#enum::BackendCommand,
};
use derive_more::From;

/// Represents a query planner.
#[derive(Debug, Clone, PartialEq)]
pub struct Planner {
    /// Represents the preprocessor.
    preprocessor: Preprocessor,
}

impl Planner {
    /// Creates a new query planner.
    /// # Arguments
    /// * `preprocessor` - The preprocessor.
    pub fn new<T>(input: T) -> Self
    where
        T: AsRef<str>,
    {
        Self {
            preprocessor: Preprocessor::new(input),
        }
    }

    pub fn next_command(
        &mut self,
    ) -> Option<Result<PlannerCommand, PlannerError>> {
        let node = self.preprocessor.next();
        if let Some(node) = node {
            let Ok(node) = node else {
                return Some(Err(node.expect_err("is error").into()));
            };
            match &node.statement {
                create_database_statement_variant!(_) => {
                    Some(Ok(BackendCommand::Database(DatabaseCommand::Create(
                        node.try_into().expect("is create database"),
                    ))
                    .into()))
                }
                drop_database_statement_variant!(_) => {
                    Some(Ok(BackendCommand::Database(DatabaseCommand::Drop(
                        node.try_into().expect("is drop database"),
                    ))
                    .into()))
                }
                quit_statement_variant!(_) => {
                    Some(Ok(FrontendCommand::Quit.into()))
                }
                _ => unimplemented!(),
            }
        } else {
            None
        }
    }
}

impl Iterator for Planner {
    type Item = Result<PlannerCommand, PlannerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_command()
    }
}

#[derive(Debug, Clone, From, PartialEq)]
pub enum PlannerError {
    PreprocessorError(PreprocessorError),
}

#[cfg(test)]
mod tests {
    use backend_api::api::command::{
        backend_api::{CreateDatabase, DatabaseCommand, DropDatabase},
        r#enum::BackendCommand,
    };

    use crate::planner::{
        adapter::PlannerCommand, command::FrontendCommand, Planner,
    };

    #[test]
    fn test_create_database() {
        let query = "CREATE DATABASE test;";

        let mut planner = Planner::new(query);
        let command = planner.next_command();

        assert!(command.is_some());
        let command = command.unwrap();
        assert!(command.is_ok());
        let command = command.unwrap();

        assert_eq!(
            command,
            PlannerCommand::Backend(BackendCommand::Database(
                DatabaseCommand::Create(CreateDatabase {
                    name: "test".into()
                })
            ))
        );
    }

    #[test]
    fn test_drop_database() {
        let query = "DROP DATABASE test;";

        let mut planner = Planner::new(query);
        let command = planner.next_command();

        assert!(command.is_some());
        let command = command.unwrap();
        assert!(command.is_ok());
        let command = command.unwrap();

        assert_eq!(
            command,
            PlannerCommand::Backend(BackendCommand::Database(
                DatabaseCommand::Drop(DropDatabase {
                    name: "test".into()
                })
            ))
        );
    }

    #[test]
    fn test_quit() {
        let query = "\\quit";

        let mut planner = Planner::new(query);
        let command = planner.next_command();

        assert!(command.is_some());
        let command = command.unwrap();
        assert!(command.is_ok());
        let command = command.unwrap();

        assert_eq!(command, PlannerCommand::Frontend(FrontendCommand::Quit));
    }
}
