use crate::{
    create_database_statement_variant, create_schema_statement_variant,
    lexer::{
        token::{DBObject, DMLOperator, Keyword, Token},
        Lexer,
    },
    parser::Statement,
};

/// Represents a DML parser.
#[derive(Debug, PartialEq)]
pub struct DmlParser<'a> {
    /// Represents the lexer.
    lexer: &'a mut Lexer,

    /// Represents the state of the parser.
    state: &'a mut Vec<Token>,
}

impl<'a> DmlParser<'a> {
    /// Creates a new DML parser.
    pub fn new(lexer: &'a mut Lexer, state: &'a mut Vec<Token>) -> Self {
        Self { lexer, state }
    }

    /// Parses a DML operation.
    pub fn parse(&mut self) -> Result<Statement, ParseError> {
        let token = self
            .state
            .last()
            .expect("exist because passed from `Parser`");
        if let Token::DML(token) = token {
            match token {
                DMLOperator::Create => self.parse_create_statement(),
                DMLOperator::Alter => {
                    todo!()
                }
                DMLOperator::Rename => {
                    todo!()
                }
                DMLOperator::Drop => {
                    todo!()
                }
            }
        } else {
            Err(ParseError::WrongTokenProvided(token.clone()))
        }
    }

    /// Parse `CREATE ...` statement.
    fn parse_create_statement(&mut self) -> Result<Statement, ParseError> {
        let which_object = self.lexer.next();
        let identifier = self.lexer.next();

        if let (Some(which_object), Some(identifier)) =
            (which_object, identifier)
        {
            let identifier = if let Token::Identifier(identifier) = identifier {
                Ok(identifier)
            } else {
                Err(ParseError::WrongTokenProvided(identifier))
            };

            if let Token::Keyword(Keyword::DbObject(obj)) = which_object {
                match obj {
                    DBObject::Database => {
                        self.state.push(which_object);
                        self.state.push(identifier?.into());

                        Ok(create_database_statement_variant!(self
                            .state
                            .as_slice()
                            .try_into()
                            .expect("valid tokens")))
                    }
                    DBObject::Schema => {
                        self.state.push(which_object);
                        self.state.push(identifier?.into());

                        Ok(create_schema_statement_variant!(self
                            .state
                            .as_slice()
                            .try_into()
                            .expect("valid tokens")))
                    }
                    DBObject::Table => {
                        self.state.push(which_object);
                        self.state.push(identifier?.into());

                        todo!("Add create table support")
                    }
                    DBObject::Column => {
                        Err(ParseError::WrongTokenProvided(which_object))
                    }
                }
            } else {
                Err(ParseError::WrongTokenProvided(which_object))
            }
        } else {
            Err(ParseError::NotEnoughTokens)
        }
    }
}

/// Error of [`DmlParser`] execution.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// Wrong token provided to the [`DmlParser`].
    WrongTokenProvided(Token),

    /// Not enough tokens got from the [`Lexer`].
    NotEnoughTokens,
}

#[cfg(test)]
mod test {
    use crate::{
        lexer::{
            token::{Identifier, Keyword, Preposition, Token},
            Lexer,
        },
        parser::statement::dml::{CreateDatabase, CreateSchema},
    };

    use super::{DmlParser, ParseError};

    #[test]
    #[should_panic]
    fn test_no_tokens_in_state() {
        let mut lexer = Lexer::new("CREATE DATABASE test".to_string());
        let mut state = vec![];
        let mut parser = DmlParser::new(&mut lexer, &mut state);

        let _ = parser.parse();
    }

    #[test]
    fn test_wrong_tokens() {
        let mut lexer = Lexer::new("CREATE TO DATABASE test".to_string());
        let mut state = vec![lexer.next().unwrap()];
        let mut parser = DmlParser::new(&mut lexer, &mut state);

        let statement = parser.parse();

        assert_eq!(
            statement,
            Err(ParseError::WrongTokenProvided(Token::Keyword(
                Keyword::Preposition(Preposition::To)
            )))
        );
    }

    #[test]
    fn test_create_database_statement() {
        let mut lexer = Lexer::new("CREATE DATABASE test".to_string());
        let mut state = vec![lexer.next().unwrap()];
        let mut parser = DmlParser::new(&mut lexer, &mut state);

        let statement = parser.parse();

        assert_eq!(
            statement,
            Ok(CreateDatabase::new_statement(Identifier(
                "test".to_string()
            )))
        );
    }

    #[test]
    fn test_create_schema_statement() {
        let mut lexer = Lexer::new("CREATE SCHEMA test".to_string());
        let mut state = vec![lexer.next().unwrap()];
        let mut parser = DmlParser::new(&mut lexer, &mut state);

        let statement = parser.parse();

        assert_eq!(
            statement,
            Ok(CreateSchema::new_statement(Identifier("test".to_string())))
        );
    }
}
