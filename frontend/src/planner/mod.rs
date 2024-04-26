pub mod adapter;
pub mod command;
mod planners;

use crate::{create_database_statement_variant, create_schema_statement_variant, database_statement_variant, drop_database_statement_variant, get_context_statement_variant, planner::{adapter::PlannerCommand, command::FrontendCommand}, preprocessor::{Preprocessor, PreprocessorError}, quit_statement_variant, use_database_statement_variant, use_schema_statement_variant};
use backend_api::api::command::{
    backend_api::DatabaseCommand, database::SchemaCommand,
    r#enum::BackendCommand,
};
use derive_more::From;
use crate::parser::Statement;
use crate::planner::planners::DatabasePlanner;

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
                database_statement_variant!(_) => {
                    Some(DatabasePlanner::new(node).parse_command())
                }
                quit_statement_variant!(_) => {
                    Some(Ok(FrontendCommand::Quit.into()))
                }
                get_context_statement_variant!(_) => {
                    Some(Ok(FrontendCommand::GetContext.into()))
                }
                use_schema_statement_variant!(_) => Some(Ok(
                    BackendCommand::Schema(SchemaCommand::Use(
                        node.try_into().expect("is use schema"),
                    ))
                    .into(),
                )),
                create_schema_statement_variant!(_) => {
                    Some(Ok(BackendCommand::Schema(SchemaCommand::Create(
                        node.try_into().expect("is create schema"),
                    ))
                    .into()))
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

/// Represents a query planner error.
#[derive(Debug, Clone, From, PartialEq)]
pub enum PlannerError {
    /// Represents a preprocessor error.
    PreprocessorError(PreprocessorError),
    
    /// Represents an unexpected statement.
    UnexpectedStatement(Statement),
}

#[cfg(test)]
mod tests {
    use backend_api::api::command::{
        backend_api::{
            CreateDatabase, DatabaseCommand, DropDatabase, UseDatabase,
        },
        database::{CreateSchema, SchemaCommand, UseSchema},
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
    fn test_use_database() {
        let query = "USE DATABASE test;";

        let mut planner = Planner::new(query);
        let command = planner.next_command();

        assert!(command.is_some());
        let command = command.unwrap();
        assert!(command.is_ok());
        let command = command.unwrap();

        assert_eq!(
            command,
            PlannerCommand::Backend(BackendCommand::Database(
                DatabaseCommand::Use(UseDatabase {
                    name: "test".into()
                })
            ))
        );
    }

    #[test]
    fn test_use_schema() {
        let query = "USE SCHEMA test;";

        let mut planner = Planner::new(query);
        let command = planner.next_command();

        assert!(command.is_some());
        let command = command.unwrap();
        assert!(command.is_ok());
        let command = command.unwrap();

        assert_eq!(
            command,
            PlannerCommand::Backend(BackendCommand::Schema(
                SchemaCommand::Use(UseSchema {
                    database_name: None,
                    name: "test".into()
                })
            ))
        );
    }

    #[test]
    fn test_use_schema_with_db() {
        let query = "USE SCHEMA xd.test;";

        let mut planner = Planner::new(query);
        let command = planner.next_command();

        assert!(command.is_some());
        let command = command.unwrap();
        assert!(command.is_ok());
        let command = command.unwrap();

        assert_eq!(
            command,
            PlannerCommand::Backend(BackendCommand::Schema(
                SchemaCommand::Use(UseSchema {
                    database_name: Some("xd".into()),
                    name: "test".into()
                })
            ))
        );
    }

    #[test]
    fn test_create_schema() {
        let query = "CREATE SCHEMA test;";

        let mut planner = Planner::new(query);
        let command = planner.next_command();

        assert!(command.is_some());
        let command = command.unwrap();
        assert!(command.is_ok());
        let command = command.unwrap();

        assert_eq!(
            command,
            PlannerCommand::Backend(BackendCommand::Schema(
                SchemaCommand::Create(CreateSchema {
                    database_name: None,
                    name: "test".into()
                })
            ))
        );
    }

    #[test]
    fn test_create_schema_with_db() {
        let query = "CREATE SCHEMA xd.test;";

        let mut planner = Planner::new(query);
        let command = planner.next_command();

        assert!(command.is_some());
        let command = command.unwrap();
        assert!(command.is_ok());
        let command = command.unwrap();

        assert_eq!(
            command,
            PlannerCommand::Backend(BackendCommand::Schema(
                SchemaCommand::Create(CreateSchema {
                    database_name: Some("xd".into()),
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

    #[test]
    fn test_get_context() {
        let query = "\\get_context";

        let mut planner = Planner::new(query);
        let command = planner.next_command();

        assert!(command.is_some());
        let command = command.unwrap();
        assert!(command.is_ok());
        let command = command.unwrap();

        assert_eq!(
            command,
            PlannerCommand::Frontend(FrontendCommand::GetContext)
        );
    }
}
