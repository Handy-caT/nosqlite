use crate::{
    lexer::{
        token,
        token::{Keyword, Preposition, Token},
    },
    preprocessor::LeafNode,
};

/// Describes `RENAME TO ...` statement for AST.
#[derive(Debug, Clone, PartialEq)]
pub struct RenameTo {
    /// Name to rename to.
    pub identifier: token::Identifier,
}

impl RenameTo {
    /// Creates a new `RenameTo` statement.
    /// # Arguments
    /// * `identifier` - Name of the schema.
    /// # Returns
    /// * New instance of `RenameTo`.
    pub fn new(identifier: token::Identifier) -> Self {
        Self { identifier }
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
            Token::Identifier(identifier) => Ok(Self::new(identifier.clone())),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod create_database_tests {
    use crate::lexer::{token, token::Token};

    use super::RenameTo;

    #[test]
    fn test_create_database_try_from_token_vec_basic() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Rename),
            Token::Keyword(token::Keyword::Preposition(token::Preposition::To)),
            Token::Identifier(token::Identifier("test".to_string())),
        ];

        let actual = RenameTo::try_from(tokens.as_slice());
        let expected = Ok(RenameTo::new(token::Identifier("test".to_string())));

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_create_database_try_from_token_vec_invalid_tokens() {
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
    fn test_create_database_try_from_token_vec_not_enough_tokens() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Create),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Schema)),
        ];

        let actual = RenameTo::try_from(tokens.as_slice());
        let expected = Err(());

        assert_eq!(actual, expected);
    }
}
