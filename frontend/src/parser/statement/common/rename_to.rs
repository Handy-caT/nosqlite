use std::fmt::Display;

use crate::{
    lexer::{
        token,
        token::{Keyword, Preposition, Token},
    },
    parser::Statement,
    preprocessor::LeafNode,
};

/// Describes `RENAME TO ...` statement for AST.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RenameTo {
    /// Name to rename to.
    pub identifier: token::Identifier,
}

impl Display for RenameTo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RENAME TO {}", self.identifier.0)
    }
}

impl RenameTo {
    /// Creates a new `RenameTo` statement.
    /// # Arguments
    /// * `identifier` - Name of the schema.
    /// # Returns
    /// * New instance of `RenameTo` [`Statement`].
    pub fn new_statement(identifier: token::Identifier) -> Statement {
        use crate::rename_to_statement_variant;

        rename_to_statement_variant!(Self { identifier })
    }
}

impl LeafNode for RenameTo {}

impl TryFrom<&[Token]> for RenameTo {
    type Error = ();

    fn try_from(tokens: &[Token]) -> Result<Self, Self::Error> {
        let mut tokens = tokens.iter();
        let rename = tokens.next().ok_or(())?;
        let to = tokens.next().ok_or(())?;
        let identifier = tokens.next().ok_or(())?;

        let Token::DML(token::DMLOperator::Rename) = rename else {
            return Err(());
        };
        let Token::Keyword(Keyword::Preposition(Preposition::To)) = to else {
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

/// Shortcut for a [`RenameTo`] variant of [`Statement`].
#[macro_export]
macro_rules! rename_to_statement_variant {
    ($($arg:tt)*) => {
        $crate::parser::Statement::Common(
            $crate::parser::statement::Common::RenameTo(
                $($arg)*,
            ),
        )
    };
}

#[cfg(test)]
mod rename_to_tests {
    use crate::{
        lexer::{token, token::Token},
        parser::statement::dml::CreateDatabase,
        preprocessor::Node,
    };

    use super::RenameTo;

    #[test]
    fn test_rename_to_try_from_token_vec_basic() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Rename),
            Token::Keyword(token::Keyword::Preposition(token::Preposition::To)),
            Token::Identifier(token::Identifier("test".to_string())),
        ];

        let actual = RenameTo::try_from(tokens.as_slice());
        let expected = Ok(RenameTo {
            identifier: token::Identifier("test".to_string()),
        });

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_rename_to_try_from_token_vec_invalid_tokens() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Create),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Table)),
            Token::Identifier(token::Identifier("test".to_string())),
        ];

        let actual = RenameTo::try_from(tokens.as_slice());
        let expected = Err(());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_rename_to_try_from_token_vec_not_enough_tokens() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Create),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Schema)),
        ];

        let actual = RenameTo::try_from(tokens.as_slice());
        let expected = Err(());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_rename_to_cant_be_followed_by_nothing() {
        let rename_to = RenameTo {
            identifier: token::Identifier("test".to_string()),
        };

        let identifier = token::Identifier("test".to_string());

        assert!(!rename_to.can_be_followed(&CreateDatabase::new_statement(
            identifier.clone()
        )));
        assert!(
            !rename_to.can_be_followed(&RenameTo::new_statement(identifier))
        );
    }
}
