use crate::lexer::{
    token,
    token::{DBObject, Keyword, Token},
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
    /// * New instance of `DropSchema`.
    pub fn new(identifier: token::Identifier) -> Self {
        Self { identifier }
    }
}

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
            Token::Identifier(identifier) => Ok(Self::new(identifier.clone())),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod create_database_tests {
    use crate::lexer::{token, token::Token};

    use super::DropSchema;

    #[test]
    fn test_create_database_try_from_token_vec_basic() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Drop),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Schema)),
            Token::Identifier(token::Identifier("test".to_string())),
        ];

        let actual = DropSchema::try_from(tokens.as_slice());
        let expected =
            Ok(DropSchema::new(token::Identifier("test".to_string())));

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_create_database_try_from_token_vec_invalid_tokens() {
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
    fn test_create_database_try_from_token_vec_not_enough_tokens() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Drop),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Schema)),
        ];

        let actual = DropSchema::try_from(tokens.as_slice());
        let expected = Err(());

        assert_eq!(actual, expected);
    }
}
