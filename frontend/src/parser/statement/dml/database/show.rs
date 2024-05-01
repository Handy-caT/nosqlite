use std::fmt::Display;

use crate::{
    lexer::{
        token,
        token::{DBObjectMany, Keyword, Token},
    },
    parser::Statement,
    preprocessor::LeafNode,
};

/// Describes `SHOW DATABASES` statement for AST.
#[derive(Debug, Clone, PartialEq)]
pub struct ShowDatabases {}

impl Display for ShowDatabases {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SHOW DATABASES")
    }
}

impl ShowDatabases {
    /// Creates a new [`ShowDatabases`] statement.
    /// # Arguments
    /// * `identifier` - Name of the database.
    /// # Returns
    /// * New instance of `ShowDatabase` [`Statement`].
    pub fn new_statement() -> Statement {
        use crate::show_databases_statement_variant;

        show_databases_statement_variant!(Self {})
    }
}

impl LeafNode for ShowDatabases {}

impl TryFrom<&[Token]> for ShowDatabases {
    type Error = ();

    fn try_from(tokens: &[Token]) -> Result<Self, Self::Error> {
        let mut tokens = tokens.iter();
        let show = tokens.next().ok_or(())?;
        let databases = tokens.next().ok_or(())?;

        let Token::DML(token::DMLOperator::Show) = show else {
            return Err(());
        };
        let Token::Keyword(Keyword::DbObjectMany(DBObjectMany::Databases)) =
            databases
        else {
            return Err(());
        };

        Ok(Self {})
    }
}

/// Shortcut for a [`ShowDatabases`] variant of [`Statement`].
#[macro_export]
macro_rules! show_databases_statement_variant {
    ($($arg:tt)*) => {
        $crate::parser::Statement::Dml(
            $crate::parser::statement::DML::Database(
                $crate::parser::statement::dml::DatabaseNode::Show(
                    $($arg)*,
                ),
            ),
        )
    };
}

#[cfg(test)]
mod show_database_tests {
    use crate::{
        lexer::{token, token::Token},
        parser::statement::dml::CreateDatabase,
        preprocessor::Node,
    };

    use super::ShowDatabases;

    #[test]
    fn test_show_databases_try_from_token_vec_basic() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Show),
            Token::Keyword(token::Keyword::DbObjectMany(
                token::DBObjectMany::Databases,
            )),
        ];

        let actual = ShowDatabases::try_from(tokens.as_slice());
        let expected = Ok(ShowDatabases {});

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_show_databases_try_from_token_vec_invalid_tokens() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Drop),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Table)),
            Token::Identifier(token::Identifier("test".to_string())),
        ];

        let actual = ShowDatabases::try_from(tokens.as_slice());
        let expected = Err(());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_show_databases_try_from_token_vec_not_enough_tokens() {
        let tokens = vec![Token::DML(token::DMLOperator::Drop)];

        let actual = ShowDatabases::try_from(tokens.as_slice());
        let expected = Err(());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_show_databases_cant_be_followed_by_nothing() {
        let show_database = ShowDatabases {};

        let identifier = token::Identifier("test".to_string());

        assert!(!show_database.can_be_followed(
            &CreateDatabase::new_statement(identifier.clone())
        ));
    }
}
