use backend_api::api::command::backend_api::{CreateDatabase, DropDatabase};

use crate::{
    create_database_statement_variant, drop_database_statement_variant,
    parser::ast,
};

impl TryFrom<ast::Node> for CreateDatabase {
    type Error = ();

    fn try_from(node: ast::Node) -> Result<Self, Self::Error> {
        if let create_database_statement_variant!(statement) = node.statement {
            Ok(CreateDatabase {
                name: statement.identifier.into(),
            })
        } else {
            Err(())
        }
    }
}

impl TryFrom<ast::Node> for DropDatabase {
    type Error = ();

    fn try_from(node: ast::Node) -> Result<Self, Self::Error> {
        if let drop_database_statement_variant!(statement) = node.statement {
            Ok(DropDatabase {
                name: statement.identifier.into(),
            })
        } else {
            Err(())
        }
    }
}
