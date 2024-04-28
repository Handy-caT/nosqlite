use crate::{
    lexer::{
        token,
        token::{Key, Keyword, Token},
    },
    parser::Statement,
    preprocessor::Node,
};

/// Represents a column in a table.
#[derive(Debug, PartialEq, Clone)]
pub struct Column {
    /// Name of the column.
    pub identifier: token::Identifier,

    /// Data type of the column.
    pub data_type: token::DataType,

    /// Whether the column is a primary key.
    pub is_primary_key: bool,
}

impl Column {
    /// Returns a new instance of `Column`.
    /// # Arguments
    /// * `column` - The column.
    /// # Returns
    /// * New instance of `Column` [`Statement`].
    pub fn new_statement(column: Column) -> Statement {
        use crate::column_statement_variant;

        column_statement_variant!(column)
    }
}

impl Node for Column {
    fn can_be_followed(&self, other: &Statement) -> bool {
        use crate::column_statement_variant;

        match other {
            column_statement_variant!(_) => true,
            _ => false,
        }
    }
}

impl TryFrom<&[Token]> for Column {
    type Error = ();

    fn try_from(tokens: &[Token]) -> Result<Self, Self::Error> {
        let mut tokens = tokens.iter();
        let identifier = tokens.next().ok_or(())?;
        let type_ = tokens.next().ok_or(())?;
        let key_type = tokens.next();
        let key = tokens.next();

        let identifier = match identifier {
            Token::Identifier(identifier) => identifier.clone(),
            _ => return Err(()),
        };

        let data_type = match type_ {
            Token::DataType(data_type) => data_type.clone(),
            _ => return Err(()),
        };

        let is_primary_key = matches!(
            (key_type, key),
            (
                Some(Token::Keyword(Keyword::Key(Key::Primary))),
                Some(Token::Keyword(Keyword::Key(Key::Key))),
            )
        );

        Ok(Self {
            identifier,
            data_type,
            is_primary_key,
        })
    }
}

/// Shortcut for [`Column`] variant of [`Statement`].
#[macro_export]
macro_rules! column_statement_variant {
    ($($arg:tt)*) => {
        $crate::parser::Statement::Common(
            $crate::parser::statement::Common::Column(
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

    use super::Column;

    #[test]
    fn test_column_to_try_from_token_vec_basic() {
        let tokens = vec![
            Token::Identifier(token::Identifier("test".to_string())),
            Token::DataType(token::DataType::Integer),
        ];

        let actual = Column::try_from(tokens.as_slice());
        let expected = Ok(Column {
            identifier: token::Identifier("test".to_string()),
            data_type: token::DataType::Integer,
            is_primary_key: false,
        });

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_column_try_from_token_vec_invalid_tokens() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Create),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Table)),
            Token::Identifier(token::Identifier("test".to_string())),
        ];

        let actual = Column::try_from(tokens.as_slice());
        let expected = Err(());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_column_try_from_token_vec_not_enough_tokens() {
        let tokens = vec![
            Token::DML(token::DMLOperator::Create),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Schema)),
        ];

        let actual = Column::try_from(tokens.as_slice());
        let expected = Err(());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_column_can_be_followed_by_column() {
        let column = Column {
            identifier: token::Identifier("test".to_string()),
            data_type: token::DataType::Integer,
            is_primary_key: false,
        };

        let another_column = Column {
            identifier: token::Identifier("test".to_string()),
            data_type: token::DataType::UInteger,
            is_primary_key: true,
        };

        let identifier = token::Identifier("test".to_string());

        assert!(!column.can_be_followed(&CreateDatabase::new_statement(
            identifier.clone()
        )));
        assert!(column.can_be_followed(&Column::new_statement(another_column)));
    }
}
