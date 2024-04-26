use crate::{column_statement_variant, lexer::{
    token,
    token::{DBObject, Keyword, Token},
}, parser::{statement::dml::CreateSchema, Statement}};
use crate::preprocessor::Node;

/// Represents the `CREATE TABLE ...` statement.
#[derive(Debug, PartialEq, Clone)]
pub struct CreateTable {
    /// Name of the table.
    pub identifier: token::Identifier,
}

impl CreateTable {
    /// Creates a new `CreateTable` statement.
    /// # Arguments
    /// * `identifier` - Name of the table.
    /// # Returns
    /// * New instance of `CreateTable` [`Statement`].
    pub fn new_statement(identifier: token::Identifier) -> Statement {
        use crate::create_table_statement_variant;

        create_table_statement_variant!(Self { identifier })
    }
}

impl Node for CreateTable {
    fn can_be_followed(&self, other: &Statement) -> bool {
        match other {
            column_statement_variant!(_) => true,
            _ => false,
        }
    }
}

impl TryFrom<&[Token]> for CreateTable {
    type Error = ();

    fn try_from(tokens: &[Token]) -> Result<Self, Self::Error> {
        let mut tokens = tokens.iter();
        let create = tokens.next().ok_or(())?;
        let table = tokens.next().ok_or(())?;
        let identifier = tokens.next().ok_or(())?;

        let Token::DML(token::DMLOperator::Create) = create else {
            return Err(());
        };
        let Token::Keyword(Keyword::DbObject(DBObject::Table)) = table else {
            return Err(());
        };

        match identifier {
            Token::Identifier(identifier) => Ok(Self {
                identifier: identifier.clone(),
            }),
            _ => Err(()),
        }
    }
}

/// Shortcut for [`CreateTable`] variant of [`Statement`].
#[macro_export]
macro_rules! create_table_statement_variant {
    ($($arg:tt)*) => {
        $crate::parser::Statement::Dml(
            $crate::parser::statement::DML::Table(
                $crate::parser::statement::dml::TableNode::Create(
                    $($arg)*,
                ),
            ),
        )
    };
}
