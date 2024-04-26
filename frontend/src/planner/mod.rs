pub mod adapter;
pub mod command;
mod planners;

use crate::{
    database_statement_variant, get_context_statement_variant,
    parser::Statement,
    planner::{
        adapter::PlannerCommand, command::FrontendCommand,
        planners::DatabasePlanner,
    },
    preprocessor::{Preprocessor, PreprocessorError},
    quit_statement_variant, schema_statement_variant,
};

use crate::planner::planners::SchemaPlanner;
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
                database_statement_variant!(_) => {
                    Some(DatabasePlanner::new(node).parse_command())
                }
                schema_statement_variant!(_) => {
                    Some(SchemaPlanner::new(node).parse_command())
                }
                quit_statement_variant!(_) => {
                    Some(Ok(FrontendCommand::Quit.into()))
                }
                get_context_statement_variant!(_) => {
                    Some(Ok(FrontendCommand::GetContext.into()))
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
    use backend_api::api::command::database::{DropSchema, RenameSchema};

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
    fn test_drop_schema() {
        let query = "DROP SCHEMA test;";

        let mut planner = Planner::new(query);
        let command = planner.next_command();

        assert!(command.is_some());
        let command = command.unwrap();
        assert!(command.is_ok());
        let command = command.unwrap();

        assert_eq!(
            command,
            PlannerCommand::Backend(BackendCommand::Schema(
                SchemaCommand::Drop(DropSchema {
                    database_name: None,
                    name: "test".into()
                })
            ))
        );
    }

    #[test]
    fn test_drop_schema_with_db() {
        let query = "DROP SCHEMA xd.test;";

        let mut planner = Planner::new(query);
        let command = planner.next_command();

        assert!(command.is_some());
        let command = command.unwrap();
        assert!(command.is_ok());
        let command = command.unwrap();

        assert_eq!(
            command,
            PlannerCommand::Backend(BackendCommand::Schema(
                SchemaCommand::Drop(DropSchema {
                    database_name: Some("xd".into()),
                    name: "test".into()
                })
            ))
        );
    }

    #[test]
    fn test_rename_schema() {
        let query = "ALTER SCHEMA test RENAME TO test1;";

        let mut planner = Planner::new(query);
        let command = planner.next_command();

        assert!(command.is_some());
        let command = command.unwrap();
        assert!(command.is_ok());
        let command = command.unwrap();

        assert_eq!(
            command,
            PlannerCommand::Backend(BackendCommand::Schema(
                SchemaCommand::Rename(RenameSchema {
                    database_name: None,
                    new_name: "test1".into(),
                    old_name: "test".into()
                })
            ))
        );
    }

    #[test]
    fn test_rename_schema_with_db() {
        let query = "ALTER SCHEMA xd.test RENAME TO xd.test1;";

        let mut planner = Planner::new(query);
        let command = planner.next_command();

        assert!(command.is_some());
        let command = command.unwrap();
        assert!(command.is_ok());
        let command = command.unwrap();

        assert_eq!(
            command,
            PlannerCommand::Backend(BackendCommand::Schema(
                SchemaCommand::Rename(RenameSchema {
                    database_name: Some("xd".into()),
                    new_name: "test1".into(),
                    old_name: "test".into()
                })
            ))
        );
    }

    #[test]
    fn test_rename_schema_with_db_from() {
        let query = "ALTER SCHEMA xd.test RENAME TO test1;";

        let mut planner = Planner::new(query);
        let command = planner.next_command();

        assert!(command.is_some());
        let command = command.unwrap();
        assert!(command.is_ok());
        let command = command.unwrap();

        assert_eq!(
            command,
            PlannerCommand::Backend(BackendCommand::Schema(
                SchemaCommand::Rename(RenameSchema {
                    database_name: Some("xd".into()),
                    new_name: "test1".into(),
                    old_name: "test".into()
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
