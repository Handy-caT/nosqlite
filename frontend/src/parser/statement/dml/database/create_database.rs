use crate::{
    lexer::{
        token,
        token::{DBObject, Keyword, Token},
    },
    preprocessor::LeafNode,
};

/// Describes `CREATE DATABASE` statement for AST.
#[derive(Debug, Clone, PartialEq)]
pub struct CreateDatabase {
    /// Name of the database.
    pub identifier: token::Identifier,
}

impl CreateDatabase {
    /// Creates a new `CreateDatabase` statement.
    /// # Arguments
    /// * `identifier` - Name of the database.
    /// # Returns
    /// * New instance of `CreateDatabase`.
    pub fn new(identifier: token::Identifier) -> Self {
        Self { identifier }
    }
}

impl LeafNode for CreateDatabase {}

impl TryFrom<&[Token]> for CreateDatabase {
    type Error = ();

    fn try_from(tokens: &[Token]) -> Result<Self, Self::Error> {
        let mut tokens = tokens.iter();
        let create = tokens.next().ok_or(())?;
        let database = tokens.next().ok_or(())?;
        let identifier = tokens.next().ok_or(())?;

        let Token::DML(token::DMLOperator::Create) = create else {
            return Err(());
        };
        let Token::Keyword(Keyword::DbObject(DBObject::Database)) = database
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
mod create_database_tests {
    use crate::{
        create_database_statement,
        lexer::{token, token::Token},
        preprocessor::Node,
    };

    use super::CreateDatabase;

    #[test]
    fn test_create_database_try_from_token_vec_basic() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Create),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Database)),
            Token::Identifier(token::Identifier("test".to_string())),
        ];

        let actual = CreateDatabase::try_from(tokens.as_slice());
        let expected =
            Ok(CreateDatabase::new(token::Identifier("test".to_string())));

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_create_database_try_from_token_vec_invalid_tokens() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Create),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Table)),
            Token::Identifier(token::Identifier("test".to_string())),
        ];

        let actual = CreateDatabase::try_from(tokens.as_slice());
        let expected = Err(());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_create_database_try_from_token_vec_not_enough_tokens() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Create),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Database)),
        ];

        let actual = CreateDatabase::try_from(tokens.as_slice());
        let expected = Err(());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_create_database_cant_be_followed_by_nothing() {
        let create_database = CreateDatabase {
            identifier: token::Identifier("test".to_string()),
        };

        let identifier = token::Identifier("test".to_string());

        assert!(!create_database
            .can_be_followed(&create_database_statement!(identifier)));
    }
}

/// Shortcut for creating a [`CreateDatabase`] variant of [`Statement`].
#[macro_export]
macro_rules! create_database_statement {
    ($($arg:tt)*) => {
        $crate::parser::Statement::Dml(
            $crate::parser::statement::DML::Database(
                $crate::parser::statement::dml::DatabaseNode::CreateDatabase(
                    $crate::parser::statement::dml::CreateDatabase::new($($arg)*),
                ),
            ),
        )
    };
}
