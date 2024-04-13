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
pub struct CreateSchema {
    /// Name of the schema.
    pub identifier: token::Identifier,
}

impl CreateSchema {
    /// Creates a new `CreateSchema` statement.
    /// # Arguments
    /// * `identifier` - Name of the schema.
    /// # Returns
    /// * New instance of `CreateSchema` [`Statement`].
    pub fn new_statement(identifier: token::Identifier) -> Statement {
        use crate::create_schema_statement_variant;

        create_schema_statement_variant!(Self { identifier })
    }
}

impl LeafNode for CreateSchema {}

impl TryFrom<&[Token]> for CreateSchema {
    type Error = ();

    fn try_from(tokens: &[Token]) -> Result<Self, Self::Error> {
        let mut tokens = tokens.iter();
        let create = tokens.next().ok_or(())?;
        let database = tokens.next().ok_or(())?;
        let identifier = tokens.next().ok_or(())?;

        let Token::DML(token::DMLOperator::Create) = create else {
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

/// Shortcut for [`CreateSchema`] variant of [`Statement`].
#[macro_export]
macro_rules! create_schema_statement_variant {
    ($($arg:tt)*) => {
        $crate::parser::Statement::Dml(
            $crate::parser::statement::DML::Schema(
                $crate::parser::statement::dml::SchemaNode::CreateSchema(
                    $($arg)*,
                ),
            ),
        )
    };
}

#[cfg(test)]
mod create_schema_tests {
    use crate::{
        lexer::{token, token::Token},
        parser::statement::dml::CreateDatabase,
        preprocessor::Node,
    };

    use super::CreateSchema;

    #[test]
    fn test_create_schema_try_from_token_vec_basic() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Create),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Schema)),
            Token::Identifier(token::Identifier("test".to_string())),
        ];

        let actual = CreateSchema::try_from(tokens.as_slice());
        let expected = Ok(CreateSchema {
            identifier: token::Identifier("test".to_string()),
        });

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_create_schema_try_from_token_vec_invalid_tokens() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Create),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Table)),
            Token::Identifier(token::Identifier("test".to_string())),
        ];

        let actual = CreateSchema::try_from(tokens.as_slice());
        let expected = Err(());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_create_schema_try_from_token_vec_not_enough_tokens() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Create),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Schema)),
        ];

        let actual = CreateSchema::try_from(tokens.as_slice());
        let expected = Err(());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_create_schema_cant_be_followed_by_nothing() {
        let create_schema = CreateSchema {
            identifier: token::Identifier("test".to_string()),
        };

        let identifier = token::Identifier("test".to_string());

        assert!(!create_schema
            .can_be_followed(&CreateDatabase::new_statement(identifier)));
    }
}
