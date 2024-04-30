use std::fmt::Display;

use crate::{
    lexer::{
        token,
        token::{DBObject, Keyword, Token},
    },
    parser::Statement,
    preprocessor::Node,
    rename_to_statement_variant,
};

/// Describes `ALTER SCHEMA ...` statement for AST.
#[derive(Debug, Clone, PartialEq)]
pub struct AlterSchema {
    /// Name of the schema.
    pub identifier: token::Identifier,
}

impl Display for AlterSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ALTER SCHEMA {}", self.identifier)
    }
}

impl AlterSchema {
    /// Creates a new `AlterSchema` statement.
    /// # Arguments
    /// * `identifier` - Name of the schema.
    /// # Returns
    /// * New instance of `AlterSchema` [`Statement`].
    pub fn new_statement(identifier: token::Identifier) -> Statement {
        use crate::alter_schema_statement_variant;

        alter_schema_statement_variant!(Self { identifier })
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
            Token::Identifier(identifier) => Ok(Self {
                identifier: identifier.clone(),
            }),
            _ => Err(()),
        }
    }
}

/// Shortcut for [`AlterSchema`] variant of [`Statement`].
#[macro_export]
macro_rules! alter_schema_statement_variant {
    ($($arg:tt)*) => {
        $crate::parser::Statement::Dml(
            $crate::parser::statement::DML::Schema(
                $crate::parser::statement::dml::SchemaNode::Alter(
                    $($arg)*,
                ),
            ),
        )
    };
}

#[cfg(test)]
mod alter_schema_tests {
    use crate::{
        lexer::{token, token::Token},
        parser::statement::{common::RenameTo, dml::CreateDatabase},
        preprocessor::Node,
    };

    use super::AlterSchema;

    #[test]
    fn test_alter_schema_try_from_token_vec_basic() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Alter),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Schema)),
            Token::Identifier(token::Identifier("test".to_string())),
        ];

        let actual = AlterSchema::try_from(tokens.as_slice());
        let expected = Ok(AlterSchema {
            identifier: token::Identifier("test".to_string()),
        });

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

        assert!(
            !alter_schema.can_be_followed(&CreateDatabase::new_statement(
                identifier.clone()
            ))
        );
        assert!(
            alter_schema.can_be_followed(&RenameTo::new_statement(identifier))
        );
    }
}
