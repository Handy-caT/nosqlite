use crate::{
    lexer::{
        token,
        token::{DBObject, Keyword, Token},
    },
    parser::Statement,
    preprocessor::LeafNode,
};

/// Describes `USE SCHEMA` statement for AST.
#[derive(Debug, Clone, PartialEq)]
pub struct UseSchema {
    /// Name of the database.
    pub identifier: token::Identifier,
}

impl UseSchema {
    /// Creates a new `UseSchema` statement.
    /// # Arguments
    /// * `identifier` - Name of the database.
    /// # Returns
    /// * New instance of `UseSchema` [`Statement`].
    pub fn new_statement(identifier: token::Identifier) -> Statement {
        use crate::use_schema_statement_variant;

        use_schema_statement_variant!(Self { identifier })
    }
}

impl LeafNode for UseSchema {}

impl TryFrom<&[Token]> for UseSchema {
    type Error = ();

    fn try_from(tokens: &[Token]) -> Result<Self, Self::Error> {
        let mut tokens = tokens.iter();
        let use_ = tokens.next().ok_or(())?;
        let schema = tokens.next().ok_or(())?;
        let identifier = tokens.next().ok_or(())?;

        let Token::DML(token::DMLOperator::Use) = use_ else {
            return Err(());
        };
        let Token::Keyword(Keyword::DbObject(DBObject::Schema)) = schema else {
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

/// Shortcut for a [`UseSchema`] variant of [`Statement`].
#[macro_export]
macro_rules! use_schema_statement_variant {
    ($($arg:tt)*) => {
        $crate::parser::Statement::Dml(
            $crate::parser::statement::DML::Schema(
                $crate::parser::statement::dml::SchemaNode::Use(
                    $($arg)*,
                ),
            ),
        )
    };
}

#[cfg(test)]
mod use_schema_tests {
    use crate::{
        lexer::{token, token::Token},
        preprocessor::Node,
    };

    use super::UseSchema;

    #[test]
    fn test_use_schema_try_from_token_vec_basic() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Use),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Schema)),
            Token::Identifier(token::Identifier("test".to_string())),
        ];

        let actual = UseSchema::try_from(tokens.as_slice());
        let expected = Ok(UseSchema {
            identifier: token::Identifier("test".to_string()),
        });

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_use_schema_try_from_token_vec_invalid_tokens() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Create),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Table)),
            Token::Identifier(token::Identifier("test".to_string())),
        ];

        let actual = UseSchema::try_from(tokens.as_slice());
        let expected = Err(());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_use_schema_try_from_token_vec_not_enough_tokens() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Create),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Database)),
        ];

        let actual = UseSchema::try_from(tokens.as_slice());
        let expected = Err(());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_use_schema_cant_be_followed_by_nothing() {
        let create_database = UseSchema {
            identifier: token::Identifier("test".to_string()),
        };

        let identifier = token::Identifier("test".to_string());

        assert!(!create_database
            .can_be_followed(&UseSchema::new_statement(identifier)));
    }
}
