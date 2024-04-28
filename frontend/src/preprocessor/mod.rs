use crate::parser::{ast, ParseError, Parser, Statement};

/// Represents an AST leaf node.
pub trait LeafNode {}

/// Represents an AST node.
pub trait Node {
    /// Determines if the node can be followed by another statement.
    /// # Arguments
    /// * `other` - The statement to check.
    /// # Returns
    /// `true` if the node can be followed by the statement, otherwise `false`.
    fn can_be_followed(&self, other: &Statement) -> bool;
}

impl<T> Node for T
where
    T: LeafNode,
{
    fn can_be_followed(&self, next: &Statement) -> bool {
        matches!(next, Statement::Semicolon)
    }
}

/// Represents a preprocessor.
#[derive(Debug, PartialEq, Clone)]
pub struct Preprocessor {
    /// Represents the parser.
    parser: Parser,
}

impl Preprocessor {
    /// Creates a new preprocessor.
    /// # Arguments
    /// * `parser` - The parser.
    pub fn new<T>(input: T) -> Self
    where
        T: AsRef<str>,
    {
        Self {
            parser: Parser::new(input),
        }
    }

    /// Preprocesses the input.
    pub fn preprocess(
        &mut self,
    ) -> Option<Result<ast::Node, PreprocessorError>> {
        let statement = self.parser.parse_statement();
        if let Some(statement) = statement {
            let statement = statement.map_err(PreprocessorError::ParseError);
            let mut statement = match statement {
                Ok(statement) => statement,
                Err(err) => return Some(Err(err)),
            };
            if matches!(statement, Statement::Semicolon) {
                return None;
            }

            let mut node = ast::Node {
                statement: statement.clone(),
                next: None,
            };
            let mut node_ref = &mut node;

            while !matches!(statement, Statement::Semicolon) {
                let next_statement = self.parser.parse_statement();
                if let Some(next_statement) = next_statement {
                    let next_statement =
                        next_statement.map_err(PreprocessorError::ParseError);
                    let next_statement = match next_statement {
                        Ok(statement) => statement,
                        Err(err) => return Some(Err(err)),
                    };

                    if !node_ref.statement.can_be_followed(&next_statement) {
                        return Some(Err(
                            PreprocessorError::WrongStatementOrder(
                                next_statement,
                            ),
                        ));
                    }

                    if matches!(next_statement, Statement::Semicolon) {
                        return Some(Ok(node));
                    }

                    let next_node = ast::Node {
                        statement: next_statement.clone(),
                        next: None,
                    };
                    node_ref.next = Some(Box::new(next_node));
                    node_ref = node_ref
                        .next
                        .as_mut()
                        .expect("exist because set previously");

                    statement = next_statement;
                } else {
                    return Some(Ok(node));
                }
            }

            Some(Ok(node))
        } else {
            None
        }
    }
}

impl Iterator for Preprocessor {
    type Item = Result<ast::Node, PreprocessorError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.preprocess()
    }
}

/// Represents a preprocessor error.
#[derive(Debug, PartialEq, Clone)]
pub enum PreprocessorError {
    /// Represents a parse error.
    ParseError(ParseError),

    /// Represents an error when the statement order is wrong.
    WrongStatementOrder(Statement),
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::token::DataType,
        parser::{
            ast,
            statement::{
                common::{Column, RenameTo},
                dml::{
                    AlterSchema, CreateDatabase, CreateSchema, CreateTable,
                    DropDatabase, DropSchema, UseDatabase, UseSchema,
                },
                shortcut::{GetContext, Quit},
            },
            Statement,
        },
    };

    use super::{Preprocessor, PreprocessorError};

    #[test]
    fn test_create_database() {
        let input = "CREATE DATABASE test;";

        let mut preprocessor = Preprocessor::new(input);
        let node = preprocessor.preprocess();

        assert_eq!(
            node,
            Some(Ok(ast::Node {
                statement: CreateDatabase::new_statement(
                    "test".to_string().into()
                ),
                next: None
            }))
        );
    }

    #[test]
    fn test_create_database_wrong_order() {
        let input = "CREATE DATABASE test RENAME TO test1;";

        let mut preprocessor = Preprocessor::new(input);
        let node = preprocessor.preprocess();

        assert_eq!(
            node,
            Some(Err(PreprocessorError::WrongStatementOrder(
                RenameTo::new_statement("test1".to_string().into())
            )))
        );
    }

    #[test]
    fn test_drop_database() {
        let input = "DROP DATABASE test;";

        let mut preprocessor = Preprocessor::new(input);
        let node = preprocessor.preprocess();

        assert_eq!(
            node,
            Some(Ok(ast::Node {
                statement: DropDatabase::new_statement(
                    "test".to_string().into()
                ),
                next: None
            }))
        );
    }

    #[test]
    fn test_create_schema() {
        let input = "CREATE SCHEMA test;";

        let mut preprocessor = Preprocessor::new(input);
        let node = preprocessor.preprocess();

        assert_eq!(
            node,
            Some(Ok(ast::Node {
                statement: CreateSchema::new_statement(
                    "test".to_string().into()
                ),
                next: None
            }))
        );
    }

    #[test]
    fn test_drop_schema() {
        let input = "DROP SCHEMA test;";

        let mut preprocessor = Preprocessor::new(input);
        let node = preprocessor.preprocess();

        assert_eq!(
            node,
            Some(Ok(ast::Node {
                statement: DropSchema::new_statement("test".to_string().into()),
                next: None
            }))
        );
    }

    #[test]
    fn test_alter_schema() {
        let input = "ALTER SCHEMA test RENAME TO test1;";

        let mut preprocessor = Preprocessor::new(input);
        let node = preprocessor.preprocess();

        assert_eq!(
            node,
            Some(Ok(ast::Node {
                statement: AlterSchema::new_statement(
                    "test".to_string().into()
                ),
                next: Some(Box::new(ast::Node {
                    statement: RenameTo::new_statement(
                        "test1".to_string().into()
                    ),
                    next: None
                }))
            }))
        );
    }

    #[test]
    fn test_wrong_alter_schema() {
        let input = "ALTER SCHEMA test;";

        let mut preprocessor = Preprocessor::new(input);
        let node = preprocessor.preprocess();

        assert_eq!(
            node,
            Some(Err(PreprocessorError::WrongStatementOrder(
                Statement::Semicolon
            )))
        );
    }

    #[test]
    fn test_use_schema() {
        let input = "USE SCHEMA test;";

        let mut preprocessor = Preprocessor::new(input);
        let node = preprocessor.preprocess();

        assert_eq!(
            node,
            Some(Ok(ast::Node {
                statement: UseSchema::new_statement("test".to_string().into()),
                next: None
            }))
        );
    }

    #[test]
    fn test_use_database() {
        let input = "USE DATABASE test;";

        let mut preprocessor = Preprocessor::new(input);
        let node = preprocessor.preprocess();

        assert_eq!(
            node,
            Some(Ok(ast::Node {
                statement: UseDatabase::new_statement(
                    "test".to_string().into()
                ),
                next: None
            }))
        );
    }

    #[test]
    fn test_create_table() {
        let input = "CREATE TABLE test(id INTEGER PRIMARY KEY);";

        let mut preprocessor = Preprocessor::new(input);
        let node = preprocessor.preprocess();

        assert_eq!(
            node,
            Some(Ok(ast::Node {
                statement: CreateTable::new_statement(
                    "test".to_string().into()
                ),
                next: Some(Box::new(ast::Node {
                    statement: Column::new_statement(Column {
                        identifier: "id".to_string().into(),
                        data_type: DataType::Integer,
                        is_primary_key: true,
                    }),
                    next: None,
                }))
            }))
        );
    }

    #[test]
    fn test_create_table_many_columns() {
        let input = "CREATE TABLE test(id INTEGER PRIMARY KEY,\
                                            name VARCHAR10,);";

        let mut preprocessor = Preprocessor::new(input);
        let node = preprocessor.preprocess();

        assert_eq!(
            node,
            Some(Ok(ast::Node {
                statement: CreateTable::new_statement(
                    "test".to_string().into()
                ),
                next: Some(Box::new(ast::Node {
                    statement: Column::new_statement(Column {
                        identifier: "id".to_string().into(),
                        data_type: DataType::Integer,
                        is_primary_key: true,
                    }),
                    next: Some(Box::new(ast::Node {
                        statement: Column::new_statement(Column {
                            identifier: "name".to_string().into(),
                            data_type: DataType::VarChar(10),
                            is_primary_key: false,
                        }),
                        next: None,
                    }))
                }))
            }))
        );
    }

    #[test]
    fn test_quit() {
        let input = "\\quit";

        let mut preprocessor = Preprocessor::new(input);
        let node = preprocessor.preprocess();

        assert_eq!(
            node,
            Some(Ok(ast::Node {
                statement: Quit::new_statement(),
                next: None
            }))
        );
    }

    #[test]
    fn test_get_context() {
        let input = "\\get_context";

        let mut preprocessor = Preprocessor::new(input);
        let node = preprocessor.preprocess();

        assert_eq!(
            node,
            Some(Ok(ast::Node {
                statement: GetContext::new_statement(),
                next: None
            }))
        );
    }
}
