use crate::{
    lexer::{
        token,
        token::{DBObject, Keyword, Token},
    },
    parser::Statement,
    preprocessor::LeafNode,
};

/// Describes `CREATE SCHEMA ...` statement for AST.
#[derive(Debug, Clone, PartialEq)]
pub struct DropSchema {
    /// Name of the schema.
    pub identifier: token::Identifier,
}

impl DropSchema {
    /// Creates a new `DropSchema` statement.
    /// # Arguments
    /// * `identifier` - Name of the schema.
    /// # Returns
    /// * New instance of `DropSchema` [`Statement`].
    pub fn new_statement(identifier: token::Identifier) -> Statement {
        use crate::drop_schema_statement_variant;

        drop_schema_statement_variant!(Self { identifier })
    }
}

impl LeafNode for DropSchema {}

impl TryFrom<&[Token]> for DropSchema {
    type Error = ();

    fn try_from(tokens: &[Token]) -> Result<Self, Self::Error> {
        let mut tokens = tokens.iter();
        let create = tokens.next().ok_or(())?;
        let database = tokens.next().ok_or(())?;
        let identifier = tokens.next().ok_or(())?;

        let Token::DML(token::DMLOperator::Drop) = create else {
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

/// Shortcut for [`DropSchema`] variant of [`Statement`].
#[macro_export]
macro_rules! drop_schema_statement_variant {
    ($($arg:tt)*) => {
        $crate::parser::Statement::Dml(
            $crate::parser::statement::DML::Schema(
                $crate::parser::statement::dml::SchemaNode::DropSchema(
                    $($arg)*,
                ),
            ),
        )
    };
}

#[cfg(test)]
mod drop_schema_tests {
    use crate::{
        lexer::{token, token::Token},
        parser::statement::dml::{CreateDatabase, CreateSchema},
        preprocessor::Node,
    };

    use super::DropSchema;

    #[test]
    fn test_drop_schema_try_from_token_vec_basic() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Drop),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Schema)),
            Token::Identifier(token::Identifier("test".to_string())),
        ];

        let actual = DropSchema::try_from(tokens.as_slice());
        let expected = Ok(DropSchema {
            identifier: token::Identifier("test".to_string()),
        });

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_drop_schema_try_from_token_vec_invalid_tokens() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Drop),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Table)),
            Token::Identifier(token::Identifier("test".to_string())),
        ];

        let actual = DropSchema::try_from(tokens.as_slice());
        let expected = Err(());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_drop_schema_try_from_token_vec_not_enough_tokens() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Drop),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Schema)),
        ];

        let actual = DropSchema::try_from(tokens.as_slice());
        let expected = Err(());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_drop_schema_cant_be_followed_by_nothing() {
        let drop_schema = DropSchema {
            identifier: token::Identifier("test".to_string()),
        };

        let identifier = token::Identifier("test".to_string());

        assert!(!drop_schema
            .can_be_followed(&CreateDatabase::new_statement(identifier)));
    }
}
