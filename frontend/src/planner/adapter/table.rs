use backend::schema::{self, column, column::primary_key::PrimaryKey, table};
use backend_api::api::command::schema::{CreateTable, DropTable};

use crate::{
    column_statement_variant, create_table_statement_variant,
    drop_table_statement_variant,
    lexer::token::{Key, Keyword, Token},
    parser::{ast, statement, statement::common::Column},
    planner::adapter::{parse_identifier, ParseError, WrongIdentifierError},
};

impl TryFrom<ast::Node> for DropTable {
    type Error = ParseError;

    fn try_from(node: ast::Node) -> Result<Self, Self::Error> {
        if let drop_table_statement_variant!(statement) = node.statement {
            let mut names =
                parse_identifier(statement.identifier.clone()).into_iter();
            let name = names
                .next()
                .ok_or(ParseError::WrongIdentifier(WrongIdentifierError {
                    got: statement.identifier,
                    expected_type: "`table_name`",
                }))?
                .into();
            let schema_name = names.next().map(|name| name.into());
            let db_name = names.next().map(|name| name.into());

            Ok(DropTable {
                database_name: db_name,
                schema_name,
                name,
            })
        } else {
            Err(ParseError::UnexpectedStatement(node.statement))
        }
    }
}

impl TryFrom<ast::Node> for CreateTable {
    type Error = ParseError;

    fn try_from(node: ast::Node) -> Result<Self, Self::Error> {
        #[rustfmt::skip]
        let (name, schema_name, db_name)
            = if let create_table_statement_variant!(
            statement
        ) = node.statement
        {
            let mut names =
                parse_identifier(statement.identifier.clone()).into_iter();
            let name: table::Name = names
                .next()
                .ok_or(ParseError::WrongIdentifier(WrongIdentifierError {
                    got: statement.identifier,
                    expected_type: "`table_name`",
                }))?
                .into();
            let schema_name = names.next().map(|name| name.into());
            let db_name = names.next().map(|name| name.into());

            (name, schema_name, db_name)
        } else {
            return Err(ParseError::UnexpectedStatement(node.statement));
        };

        let (columns, primary_key) = {
            let mut next = node.next;
            let mut columns = vec![];
            let mut primary_key = None;

            if next.is_none() {
                return Err(ParseError::ExpectedStatement(
                    Column::new_statement(Column::default()),
                ));
            }

            while next.is_some() {
                let node = next.unwrap();
                if let column_statement_variant!(statement) =
                    node.statement.clone()
                {
                    let column_name: column::Name =
                        statement.identifier.0.into();
                    let data_type = statement.data_type.into();
                    let column = schema::Column::new(data_type);

                    if statement.is_primary_key {
                        if primary_key.is_some() {
                            return Err(ParseError::UnexpectedStatement(
                                node.statement,
                            ));
                        }
                        primary_key = Some(column_name.clone());
                    }

                    columns.push((column_name, column));
                } else {
                    return Err(ParseError::UnexpectedStatement(
                        node.statement,
                    ));
                }

                next = node.next;
            }

            let primary_key = if let Some(primary_key) = primary_key {
                PrimaryKey::new("pk".into(), primary_key)
            } else {
                return Err(ParseError::ExpectedTokens(vec![
                    Token::Keyword(Keyword::Key(Key::Primary)),
                    Token::Keyword(Keyword::Key(Key::Key)),
                ]));
            };

            (columns, primary_key)
        };

        Ok(CreateTable {
            database_name: db_name,
            schema_name,
            name,
            columns,
            primary_key,
        })
    }
}
