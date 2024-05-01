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
    quit_statement_variant, schema_statement_variant, table_statement_variant,
    use_schema_statement_variant,
};

use crate::planner::{
    adapter::ParseError,
    planners::{SchemaPlanner, TablePlanner},
};
use derive_more::{Display, From};

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
                use_schema_statement_variant!(_) => {
                    Some(DatabasePlanner::new(node).parse_command())
                }
                schema_statement_variant!(_) => {
                    Some(SchemaPlanner::new(node).parse_command())
                }
                table_statement_variant!(_) => {
                    Some(TablePlanner::new(node).parse_command())
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
#[derive(Debug, Display, Clone, From, PartialEq)]
pub enum PlannerError {
    /// Represents a preprocessor error.
    #[display(fmt = "{:?}", _0)]
    PreprocessorError(PreprocessorError),

    /// Represents a parse error.
    ParseError(ParseError),

    /// Represents an unexpected statement.
    UnexpectedStatement(Statement),
}

#[cfg(test)]
mod tests {
    use backend::schema::{
        column::primary_key::PrimaryKey, r#type::r#enum::StorageDataType,
        Column,
    };
    use backend_api::api::command::{
        backend_api::{
            CreateDatabase, DatabaseCommand, DropDatabase, ShowDatabases,
            UseDatabase, UseSchema,
        },
        database::{
            CreateSchema, DropSchema, RenameSchema, SchemaCommand, ShowSchemas,
        },
        r#enum::BackendCommand,
        schema::{CreateTable, DropTable, TableCommand},
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
            PlannerCommand::Backend(BackendCommand::Database(
                DatabaseCommand::UseSchema(UseSchema {
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
            PlannerCommand::Backend(BackendCommand::Database(
                DatabaseCommand::UseSchema(UseSchema {
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
    fn test_create_table_with_db_from() {
        let query = "CREATE TABLE xd.test.tbl (id LONG PRIMARY KEY);";

        let mut planner = Planner::new(query);
        let command = planner.next_command();

        assert!(command.is_some());
        let command = command.unwrap();
        assert!(command.is_ok());
        let command = command.unwrap();

        assert_eq!(
            command,
            PlannerCommand::Backend(BackendCommand::Table(
                TableCommand::Create(CreateTable {
                    database_name: Some("xd".into()),
                    schema_name: Some("test".into()),
                    name: "tbl".into(),
                    columns: vec![(
                        "id".into(),
                        Column::new(StorageDataType::Long)
                    )],
                    primary_key: PrimaryKey::new("pk".into(), "id".into())
                })
            ))
        );
    }

    #[test]
    fn test_create_table_with_many_columns() {
        let query = "CREATE TABLE xd.test.tbl (id LONG PRIMARY KEY,\
                                                    name VARCHAR10,);";

        let mut planner = Planner::new(query);
        let command = planner.next_command();

        assert!(command.is_some());
        let command = command.unwrap();
        assert!(command.is_ok());
        let command = command.unwrap();

        assert_eq!(
            command,
            PlannerCommand::Backend(BackendCommand::Table(
                TableCommand::Create(CreateTable {
                    database_name: Some("xd".into()),
                    schema_name: Some("test".into()),
                    name: "tbl".into(),
                    columns: vec![
                        ("id".into(), Column::new(StorageDataType::Long)),
                        (
                            "name".into(),
                            Column::new(StorageDataType::VarChar(10))
                        )
                    ],
                    primary_key: PrimaryKey::new("pk".into(), "id".into())
                })
            ))
        );
    }

    #[test]
    fn test_drop_table_with_db_from() {
        let query = "DROP TABLE xd.test.tbl;";

        let mut planner = Planner::new(query);
        let command = planner.next_command();

        assert!(command.is_some());
        let command = command.unwrap();
        assert!(command.is_ok());
        let command = command.unwrap();

        assert_eq!(
            command,
            PlannerCommand::Backend(BackendCommand::Table(TableCommand::Drop(
                DropTable {
                    database_name: Some("xd".into()),
                    schema_name: Some("test".into()),
                    name: "tbl".into(),
                }
            )))
        );
    }

    #[test]
    fn test_show_databases_with_db_from() {
        let query = "SHOW DATABASES;";

        let mut planner = Planner::new(query);
        let command = planner.next_command();

        assert!(command.is_some());
        let command = command.unwrap();
        assert!(command.is_ok());
        let command = command.unwrap();

        assert_eq!(
            command,
            PlannerCommand::Backend(BackendCommand::Database(
                DatabaseCommand::ShowDatabases(ShowDatabases {})
            ))
        );
    }

    #[test]
    fn test_show_schemas_with_db_from() {
        let query = "SHOW SCHEMAS FROM db;";

        let mut planner = Planner::new(query);
        let command = planner.next_command();

        assert!(command.is_some());
        let command = command.unwrap();
        assert!(command.is_ok());
        let command = command.unwrap();

        assert_eq!(
            command,
            PlannerCommand::Backend(BackendCommand::Schema(
                SchemaCommand::Show(ShowSchemas {
                    database_name: "db".into(),
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
