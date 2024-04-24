use backend_api::api::command::backend_api::UseSchema;
use crate::create_database_statement_variant;
use crate::parser::ast;
use crate::planner::adapter::database::{ParseError, WrongIdentifierError};
use crate::planner::adapter::parse_identifier;

impl TryFrom<ast::Node> for UseSchema {
    type Error = ParseError;

    fn try_from(node: ast::Node) -> Result<Self, Self::Error> {
        if let create_database_statement_variant!(statement) = node.statement {
            let names = parse_identifier(statement.identifier.clone());
            let name = names.first().ok_or(WrongIdentifierError {
                got: statement.identifier,
                expected_type: "`schema_name`"
            })?.to_string();
            let (schema_name, db_name) = if let Some(db_name) = names.get(1) {
                (name, Some(db_name.to_string()))
            } else {
                (name.clone(), None)
            };

            Ok(UseSchema { : name.into() })
        } else {
            Err(ParseError::UnexpectedStatement)
        }
    }
}