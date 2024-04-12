use crate::lexer::{
    token,
    token::{DBObject, Keyword, Token},
};
use crate::parser::Statement;
use crate::parser::statement::dml::DropDatabase;
use crate::preprocessor::Node;
use crate::{rename_to_statement, rename_to_statement_variant};

/// Describes `ALTER SCHEMA ...` statement for AST.
#[derive(Debug, Clone, PartialEq)]
pub struct AlterSchema {
    /// Name of the schema.
    pub identifier: token::Identifier,
}

impl AlterSchema {
    /// Creates a new `AlterSchema` statement.
    /// # Arguments
    /// * `identifier` - Name of the schema.
    /// # Returns
    /// * New instance of `AlterSchema`.
    pub fn new(identifier: token::Identifier) -> Self {
        Self { identifier }
    }
}

impl Node for AlterSchema {
    fn can_be_followed(&self, other: &Statement) -> bool {
        match other {
            rename_to_statement_variant!(_) => true,
            _ => false,
        }
    }
}

impl TryFrom<&[Token]> for AlterSchema {
    type Error = ();

    fn try_from(tokens: &[Token]) -> Result<Self, Self::Error> {
        let mut tokens = tokens.iter();
        let create = tokens.next().ok_or(())?;
        let database = tokens.next().ok_or(())?;
        let identifier = tokens.next().ok_or(())?;

        let Token::DML(token::DMLOperator::Alter) = create else {
            return Err(());
        };
        let Token::Keyword(Keyword::DbObject(DBObject::Schema)) = database
        else {
            return Err(());
        };

        match identifier {
            Token::Identifier(identifier) => Ok(Self::new(identifier.clone())),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod alter_schema_tests {
    use crate::{create_database_statement, rename_to_statement};
    use crate::lexer::{token, token::Token};
    use crate::preprocessor::Node;

    use super::AlterSchema;

    #[test]
    fn test_alter_schema_try_from_token_vec_basic() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Alter),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Schema)),
            Token::Identifier(token::Identifier("test".to_string())),
        ];

        let actual = AlterSchema::try_from(tokens.as_slice());
        let expected =
            Ok(AlterSchema::new(token::Identifier("test".to_string())));

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_alter_schema_try_from_token_vec_invalid_tokens() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Alter),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Table)),
            Token::Identifier(token::Identifier("test".to_string())),
        ];

        let actual = AlterSchema::try_from(tokens.as_slice());
        let expected = Err(());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_alter_schema_try_from_token_vec_not_enough_tokens() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Alter),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Schema)),
        ];

        let actual = AlterSchema::try_from(tokens.as_slice());
        let expected = Err(());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_alter_schema_cant_be_followed_by_rename_to() {
        let alter_schema = AlterSchema {
            identifier: token::Identifier("test".to_string()),
        };

        let identifier = token::Identifier("test".to_string());

        assert!(!alter_schema
            .can_be_followed(&create_database_statement!(identifier.clone())));
        assert!(alter_schema
            .can_be_followed(&rename_to_statement!(identifier)));
    }
}

/// Shortcut for creating a [`AlterSchema`] variant of [`Statement`].
#[macro_export]
macro_rules! alter_schema_statement {
    ($($arg:tt)*) => {
        $crate::parser::Statement::Dml(
            $crate::parser::statement::DML::Schema(
                $crate::parser::statement::dml::SchemaNode::AlterSchema(
                    $crate::parser::statement::dml::AlterSchema::new($($arg)*),
                ),
            ),
        )
    };
}