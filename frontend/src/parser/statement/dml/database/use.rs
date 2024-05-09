use crate::{
    lexer::{
        token,
        token::{DBObject, Keyword, Token},
    },
    parser::Statement,
    preprocessor::LeafNode,
};
use std::fmt::Display;

/// Describes `USE DATABASE` statement for AST.
#[derive(Debug, Clone, PartialEq)]
pub struct UseDatabase {
    /// Name of the database.
    pub identifier: token::Identifier,
}

impl Display for UseDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "USE DATABASE {}", self.identifier)
    }
}

impl UseDatabase {
    /// Creates a new `UseDatabase` statement.
    /// # Arguments
    /// * `identifier` - Name of the database.
    /// # Returns
    /// * New instance of `UseDatabase` [`Statement`].
    pub fn new_statement(identifier: token::Identifier) -> Statement {
        use crate::use_database_statement_variant;

        use_database_statement_variant!(Self { identifier })
    }
}

impl LeafNode for UseDatabase {}

impl TryFrom<&[Token]> for UseDatabase {
    type Error = ();

    fn try_from(tokens: &[Token]) -> Result<Self, Self::Error> {
        let mut tokens = tokens.iter();
        let use_ = tokens.next().ok_or(())?;
        let database = tokens.next().ok_or(())?;
        let identifier = tokens.next().ok_or(())?;

        let Token::DML(token::DMLOperator::Use) = use_ else {
            return Err(());
        };
        let Token::Keyword(Keyword::DbObject(DBObject::Database)) = database
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

/// Shortcut for a [`UseDatabase`] variant of [`Statement`].
#[macro_export]
macro_rules! use_database_statement_variant {
    ($($arg:tt)*) => {
        $crate::parser::Statement::Dml(
            $crate::parser::statement::DML::Database(
                $crate::parser::statement::dml::DatabaseNode::Use(
                    $($arg)*,
                ),
            ),
        )
    };
}

#[cfg(test)]
mod use_database_tests {
    use crate::{
        lexer::{token, token::Token},
        preprocessor::Node,
    };

    use super::UseDatabase;

    #[test]
    fn test_use_database_try_from_token_vec_basic() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Use),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Database)),
            Token::Identifier(token::Identifier("test".to_string())),
        ];

        let actual = UseDatabase::try_from(tokens.as_slice());
        let expected = Ok(UseDatabase {
            identifier: token::Identifier("test".to_string()),
        });

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_use_database_try_from_token_vec_invalid_tokens() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Create),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Table)),
            Token::Identifier(token::Identifier("test".to_string())),
        ];

        let actual = UseDatabase::try_from(tokens.as_slice());
        let expected = Err(());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_use_database_try_from_token_vec_not_enough_tokens() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Create),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Database)),
        ];

        let actual = UseDatabase::try_from(tokens.as_slice());
        let expected = Err(());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_use_database_cant_be_followed_by_nothing() {
        let create_database = UseDatabase {
            identifier: token::Identifier("test".to_string()),
        };

        let identifier = token::Identifier("test".to_string());

        assert!(!create_database
            .can_be_followed(&UseDatabase::new_statement(identifier)));
    }
}
