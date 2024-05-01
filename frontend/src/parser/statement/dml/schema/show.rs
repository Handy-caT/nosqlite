use std::fmt::Display;

use crate::{
    lexer::{
        token,
        token::{DBObjectMany, Identifier, Keyword, Token},
    },
    parser::Statement,
    preprocessor::LeafNode,
};

/// Describes `SHOW SCHEMA` statement for AST.
#[derive(Debug, Clone, PartialEq)]
pub struct ShowSchemas {
    /// Name of the database to show schemas
    from: Identifier,
}

impl Display for ShowSchemas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SHOW SCHEMAS FROM {}", self.from)
    }
}

impl ShowSchemas {
    /// Creates a new [`ShowSchemas`] statement.
    /// # Arguments
    /// * `identifier` - Name of the database.
    /// # Returns
    /// * New instance of `ShowSchema` [`Statement`].
    pub fn new_statement(identifier: Identifier) -> Statement {
        use crate::show_schemas_statement_variant;

        show_schemas_statement_variant!(Self { from: identifier })
    }
}

impl LeafNode for ShowSchemas {}

impl TryFrom<&[Token]> for ShowSchemas {
    type Error = ();

    fn try_from(tokens: &[Token]) -> Result<Self, Self::Error> {
        let mut tokens = tokens.iter();
        let show = tokens.next().ok_or(())?;
        let schemas = tokens.next().ok_or(())?;
        let from = tokens.next().ok_or(())?;
        let identifier = tokens.next().ok_or(())?;

        let Token::DML(token::DMLOperator::Show) = show else {
            return Err(());
        };
        let Token::Keyword(Keyword::DbObjectMany(DBObjectMany::Schemas)) =
            schemas
        else {
            return Err(());
        };
        let Token::Keyword(Keyword::Preposition(token::Preposition::From)) =
            from
        else {
            return Err(());
        };

        match identifier {
            Token::Identifier(identifier) => Ok(Self {
                from: identifier.clone(),
            }),
            _ => Err(()),
        }
    }
}

/// Shortcut for a [`ShowSchemas`] variant of [`Statement`].
#[macro_export]
macro_rules! show_schemas_statement_variant {
    ($($arg:tt)*) => {
        $crate::parser::Statement::Dml(
            $crate::parser::statement::DML::Schema(
                $crate::parser::statement::dml::SchemaNode::Show(
                    $($arg)*,
                ),
            ),
        )
    };
}

#[cfg(test)]
mod show_schemas_tests {
    use crate::{
        lexer::{token, token::Token},
        parser::statement::dml::CreateDatabase,
        preprocessor::Node,
    };

    use super::ShowSchemas;

    #[test]
    fn test_show_schemas_try_from_token_vec_basic() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Show),
            Token::Keyword(token::Keyword::DbObjectMany(
                token::DBObjectMany::Schemas,
            )),
            Token::Keyword(token::Keyword::Preposition(token::Preposition::From)),
            Token::Identifier(token::Identifier("test".to_string())),
        ];

        let actual = ShowSchemas::try_from(tokens.as_slice());
        let expected = Ok(ShowSchemas {
            from: token::Identifier("test".to_string()),
        });

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_show_schemas_try_from_token_vec_invalid_tokens() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Drop),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Table)),
            Token::Identifier(token::Identifier("test".to_string())),
        ];

        let actual = ShowSchemas::try_from(tokens.as_slice());
        let expected = Err(());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_show_schemas_try_from_token_vec_not_enough_tokens() {
        let tokens = vec![Token::DML(token::DMLOperator::Drop)];

        let actual = ShowSchemas::try_from(tokens.as_slice());
        let expected = Err(());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_show_schemas_cant_be_followed_by_nothing() {
        let identifier = token::Identifier("test".to_string());
        let show_schemas = ShowSchemas {
            from: identifier.clone(),
        };

        assert!(!show_schemas.can_be_followed(
            &CreateDatabase::new_statement(identifier.clone())
        ));
    }
}
