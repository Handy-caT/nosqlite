use crate::{
    lexer::{
        token,
        token::{DBObject, Keyword, Token},
    },
    preprocessor::LeafNode,
};

/// Describes `DROP DATABASE ...` statement for AST.
#[derive(Debug, Clone, PartialEq)]
pub struct DropDatabase {
    /// Name of the database.
    pub identifier: token::Identifier,
}

impl DropDatabase {
    /// Creates a new [`DropDatabase`] statement.
    /// # Arguments
    /// * `identifier` - Name of the database.
    /// # Returns
    /// * New instance of [`DropDatabase`].
    pub fn new(identifier: token::Identifier) -> Self {
        Self { identifier }
    }
}

impl LeafNode for DropDatabase {}

impl TryFrom<&[Token]> for DropDatabase {
    type Error = ();

    fn try_from(tokens: &[Token]) -> Result<Self, Self::Error> {
        let mut tokens = tokens.iter();
        let drop = tokens.next().ok_or(())?;
        let database = tokens.next().ok_or(())?;
        let identifier = tokens.next().ok_or(())?;

        let Token::DML(token::DMLOperator::Drop) = drop else {
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
        create_database_statement, drop_database_statement,
        lexer::{token, token::Token},
        parser::statement::dml::CreateDatabase,
        preprocessor::Node,
    };

    use super::DropDatabase;

    #[test]
    fn test_create_database_try_from_token_vec_basic() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Drop),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Database)),
            Token::Identifier(token::Identifier("test".to_string())),
        ];

        let actual = DropDatabase::try_from(tokens.as_slice());
        let expected =
            Ok(DropDatabase::new(token::Identifier("test".to_string())));

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_create_database_try_from_token_vec_invalid_tokens() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Drop),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Table)),
            Token::Identifier(token::Identifier("test".to_string())),
        ];

        let actual = DropDatabase::try_from(tokens.as_slice());
        let expected = Err(());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_create_database_try_from_token_vec_not_enough_tokens() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Drop),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Database)),
        ];

        let actual = DropDatabase::try_from(tokens.as_slice());
        let expected = Err(());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_create_database_cant_be_followed_by_nothing() {
        let drop_database = DropDatabase {
            identifier: token::Identifier("test".to_string()),
        };

        let identifier = token::Identifier("test".to_string());

        assert!(!drop_database
            .can_be_followed(&create_database_statement!(identifier.clone())));
        assert!(!drop_database
            .can_be_followed(&drop_database_statement!(identifier)));
    }
}

/// Shortcut for creating a [`DropDatabase`] variant of [`Statement`].
#[macro_export]
macro_rules! drop_database_statement {
    ($arg:expr) => {
        $crate::parser::Statement::Dml(
            $crate::parser::statement::DML::Database(
                $crate::parser::statement::dml::DatabaseNode::DropDatabase(
                    $crate::parser::statement::dml::DropDatabase::new($arg),
                ),
            ),
        )
    };
}
