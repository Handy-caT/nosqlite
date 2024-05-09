use crate::{
    alter_schema_statement_variant, create_database_statement_variant,
    create_schema_statement_variant, create_table_statement_variant,
    drop_database_statement_variant, drop_schema_statement_variant,
    drop_table_statement_variant,
    lexer::{
        token::{
            DBObject, DBObjectMany, DMLOperator, Identifier, Keyword,
            Preposition, Token,
        },
        Lexer,
    },
    parser::Statement,
    rename_to_statement_variant, show_databases_statement_variant,
    show_schemas_statement_variant, use_database_statement_variant,
    use_schema_statement_variant,
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
                DMLOperator::Alter => self.parse_alter_statement(),
                DMLOperator::Rename => self.parse_rename_statement(),
                DMLOperator::Drop => self.parse_drop_statement(),
                DMLOperator::Use => self.parse_use_statement(),
                DMLOperator::Show => self.parse_show_statement(),
            }
        } else {
            panic!("Wrong token provided to the DML parser")
        }
    }

    /// Parse `IDENTIFIER` token.
    fn parse_identifier(&mut self) -> Result<Identifier, ParseError> {
        let identifier = self.lexer.next();
        if let Some(identifier) = identifier {
            if let Token::Identifier(identifier) = identifier {
                Ok(identifier)
            } else {
                Err(ParseError::ExpectedIdentifier(identifier))
            }
        } else {
            Err(ParseError::NotEnoughTokens)
        }
    }

    /// Parse `CREATE ...` statement.
    fn parse_create_statement(&mut self) -> Result<Statement, ParseError> {
        let which_object = self.lexer.next();
        let identifier = self.parse_identifier();

        if let Some(which_object) = which_object {
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

                        Ok(create_table_statement_variant!(self
                            .state
                            .as_slice()
                            .try_into()
                            .expect("valid tokens")))
                    }
                    DBObject::Column => Err(ParseError::WrongTokenProvided {
                        got: which_object,
                        expected: "DATABASE|SCHEMA|TABLE".to_string(),
                    }),
                }
            } else {
                Err(ParseError::WrongTokenProvided {
                    got: which_object,
                    expected: "DATABASE|SCHEMA|TABLE".to_string(),
                })
            }
        } else {
            Err(ParseError::NotEnoughTokens)
        }
    }

    /// Parse `DROP ...` statement.
    fn parse_drop_statement(&mut self) -> Result<Statement, ParseError> {
        let which_object = self.lexer.next();
        let identifier = self.parse_identifier();

        if let Some(which_object) = which_object {
            if let Token::Keyword(Keyword::DbObject(obj)) = which_object {
                match obj {
                    DBObject::Database => {
                        self.state.push(which_object);
                        self.state.push(identifier?.into());

                        Ok(drop_database_statement_variant!(self
                            .state
                            .as_slice()
                            .try_into()
                            .expect("valid tokens")))
                    }
                    DBObject::Schema => {
                        self.state.push(which_object);
                        self.state.push(identifier?.into());

                        Ok(drop_schema_statement_variant!(self
                            .state
                            .as_slice()
                            .try_into()
                            .expect("valid tokens")))
                    }
                    DBObject::Table => {
                        self.state.push(which_object);
                        self.state.push(identifier?.into());

                        Ok(drop_table_statement_variant!(self
                            .state
                            .as_slice()
                            .try_into()
                            .expect("valid tokens")))
                    }
                    DBObject::Column => Err(ParseError::WrongTokenProvided {
                        got: which_object,
                        expected: "DATABASE|SCHEMA|TABLE".to_string(),
                    }),
                }
            } else {
                Err(ParseError::WrongTokenProvided {
                    got: which_object,
                    expected: "DATABASE|SCHEMA|TABLE".to_string(),
                })
            }
        } else {
            Err(ParseError::NotEnoughTokens)
        }
    }

    /// Parse `DROP ...` statement.
    fn parse_alter_statement(&mut self) -> Result<Statement, ParseError> {
        let which_object = self.lexer.next();
        let identifier = self.parse_identifier();

        if let Some(which_object) = which_object {
            if let Token::Keyword(Keyword::DbObject(obj)) = which_object {
                match obj {
                    DBObject::Database => Err(ParseError::WrongTokenProvided {
                        got: which_object,
                        expected: "SCHEMA|TABLE".to_string(),
                    }),
                    DBObject::Schema => {
                        self.state.push(which_object);
                        self.state.push(identifier?.into());

                        Ok(alter_schema_statement_variant!(self
                            .state
                            .as_slice()
                            .try_into()
                            .expect("valid tokens")))
                    }
                    DBObject::Table => {
                        self.state.push(which_object);
                        self.state.push(identifier?.into());

                        todo!("Add alter table support")
                    }
                    DBObject::Column => Err(ParseError::WrongTokenProvided {
                        got: which_object,
                        expected: "SCHEMA|TABLE".to_string(),
                    }),
                }
            } else {
                Err(ParseError::WrongTokenProvided {
                    got: which_object,
                    expected: "SCHEMA|TABLE".to_string(),
                })
            }
        } else {
            Err(ParseError::NotEnoughTokens)
        }
    }

    /// Parse `RENAME TO ...` statement.
    fn parse_rename_statement(&mut self) -> Result<Statement, ParseError> {
        let to = self.lexer.next();
        let identifier = self.parse_identifier();

        if let Some(to) = to {
            if let Token::Keyword(Keyword::Preposition(Preposition::To)) = to {
                self.state.push(to);
                self.state.push(identifier?.into());

                Ok(rename_to_statement_variant!(self
                    .state
                    .as_slice()
                    .try_into()
                    .expect("valid tokens")))
            } else {
                Err(ParseError::WrongTokenProvided {
                    got: to,
                    expected: "TO".to_string(),
                })
            }
        } else {
            Err(ParseError::NotEnoughTokens)
        }
    }

    fn parse_use_statement(&mut self) -> Result<Statement, ParseError> {
        let which_object = self.lexer.next();
        let identifier = self.parse_identifier();

        if let Some(which_object) = which_object {
            if let Token::Keyword(Keyword::DbObject(obj)) = which_object {
                match obj {
                    DBObject::Database => {
                        self.state.push(which_object);
                        self.state.push(identifier?.into());

                        Ok(use_database_statement_variant!(self
                            .state
                            .as_slice()
                            .try_into()
                            .expect("valid tokens")))
                    }
                    DBObject::Schema => {
                        self.state.push(which_object);
                        self.state.push(identifier?.into());

                        Ok(use_schema_statement_variant!(self
                            .state
                            .as_slice()
                            .try_into()
                            .expect("valid tokens")))
                    }
                    DBObject::Table => Err(ParseError::WrongTokenProvided {
                        got: which_object,
                        expected: "DATABASE|SCHEMA".to_string(),
                    }),
                    DBObject::Column => Err(ParseError::WrongTokenProvided {
                        got: which_object,
                        expected: "DATABASE|SCHEMA".to_string(),
                    }),
                }
            } else {
                Err(ParseError::WrongTokenProvided {
                    got: which_object,
                    expected: "DATABASE|SCHEMA".to_string(),
                })
            }
        } else {
            Err(ParseError::NotEnoughTokens)
        }
    }

    /// Parse `SHOW ...` statement.
    fn parse_show_statement(&mut self) -> Result<Statement, ParseError> {
        let which_object = self.lexer.next();
        if let Some(which_object) = which_object {
            if let Token::Keyword(Keyword::DbObjectMany(obj)) = which_object {
                match obj {
                    DBObjectMany::Databases => {
                        self.state.push(which_object);

                        Ok(show_databases_statement_variant!(self
                            .state
                            .as_slice()
                            .try_into()
                            .expect("valid tokens")))
                    }
                    DBObjectMany::Schemas => {
                        self.state.push(which_object);
                        let from = self.lexer.next();
                        if let Some(Token::Keyword(Keyword::Preposition(
                            Preposition::From,
                        ))) = from
                        {
                        } else {
                            return Err(ParseError::WrongTokenProvided {
                                got: from.unwrap(),
                                expected: "FROM".to_string(),
                            });
                        }
                        self.state.push(from.expect("exist because checked"));
                        let identifier = self.parse_identifier();
                        self.state.push(identifier?.into());

                        Ok(show_schemas_statement_variant!(self
                            .state
                            .as_slice()
                            .try_into()
                            .expect("valid tokens")))
                    }
                    DBObjectMany::Tables => {
                        Err(ParseError::WrongTokenProvided {
                            got: which_object,
                            expected: "DATABASES|SCHEMAS".to_string(),
                        })
                    }
                    DBObjectMany::Columns => {
                        Err(ParseError::WrongTokenProvided {
                            got: which_object,
                            expected: "DATABASES|SCHEMAS".to_string(),
                        })
                    }
                }
            } else {
                Err(ParseError::WrongTokenProvided {
                    got: which_object,
                    expected: "DATABASES|SCHEMAS".to_string(),
                })
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
    WrongTokenProvided { got: Token, expected: String },

    /// Not enough tokens got from the [`Lexer`].
    NotEnoughTokens,

    /// Expected identifier token but got something else.
    ExpectedIdentifier(Token),
}

#[cfg(test)]
mod test {
    use crate::{
        lexer::{
            token::{Identifier, Keyword, Preposition, Token},
            Lexer,
        },
        parser::statement::dml::{
            AlterSchema, CreateDatabase, CreateSchema, CreateTable,
            DropDatabase, DropSchema, DropTable, ShowDatabases, ShowSchemas,
        },
    };

    use super::{DmlParser, ParseError};

    #[test]
    #[should_panic]
    fn test_no_tokens_in_state() {
        let mut lexer = Lexer::new("CREATE DATABASE test");
        let mut state = vec![];
        let mut parser = DmlParser::new(&mut lexer, &mut state);

        let _ = parser.parse();
    }

    #[test]
    fn test_wrong_tokens() {
        let mut lexer = Lexer::new("CREATE TO DATABASE test");
        let mut state = vec![lexer.next().unwrap()];
        let mut parser = DmlParser::new(&mut lexer, &mut state);

        let statement = parser.parse();

        assert_eq!(
            statement,
            Err(ParseError::WrongTokenProvided {
                got: Token::Keyword(Keyword::Preposition(Preposition::To)),
                expected: "DATABASE|SCHEMA|TABLE".to_string()
            })
        );
    }

    #[test]
    fn test_create_database_statement() {
        let mut lexer = Lexer::new("CREATE DATABASE test");
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
    fn test_create_table_statement() {
        let mut lexer = Lexer::new("CREATE TABLE test");
        let mut state = vec![lexer.next().unwrap()];
        let mut parser = DmlParser::new(&mut lexer, &mut state);

        let statement = parser.parse();

        assert_eq!(
            statement,
            Ok(CreateTable::new_statement(Identifier("test".to_string())))
        );
    }

    #[test]
    fn test_create_schema_statement() {
        let mut lexer = Lexer::new("CREATE SCHEMA test");
        let mut state = vec![lexer.next().unwrap()];
        let mut parser = DmlParser::new(&mut lexer, &mut state);

        let statement = parser.parse();

        assert_eq!(
            statement,
            Ok(CreateSchema::new_statement(Identifier("test".to_string())))
        );
    }

    #[test]
    fn test_drop_database_statement() {
        let mut lexer = Lexer::new("DROP DATABASE test");
        let mut state = vec![lexer.next().unwrap()];
        let mut parser = DmlParser::new(&mut lexer, &mut state);

        let statement = parser.parse();

        assert_eq!(
            statement,
            Ok(DropDatabase::new_statement(Identifier("test".to_string())))
        );
    }

    #[test]
    fn test_drop_schema_statement() {
        let mut lexer = Lexer::new("DROP SCHEMA test");
        let mut state = vec![lexer.next().unwrap()];
        let mut parser = DmlParser::new(&mut lexer, &mut state);

        let statement = parser.parse();

        assert_eq!(
            statement,
            Ok(DropSchema::new_statement(Identifier("test".to_string())))
        );
    }

    #[test]
    fn test_drop_table_statement() {
        let mut lexer = Lexer::new("DROP TABLE test");
        let mut state = vec![lexer.next().unwrap()];
        let mut parser = DmlParser::new(&mut lexer, &mut state);

        let statement = parser.parse();

        assert_eq!(
            statement,
            Ok(DropTable::new_statement(Identifier("test".to_string())))
        );
    }

    #[test]
    fn test_alter_schema_statement() {
        let mut lexer = Lexer::new("ALTER SCHEMA test");
        let mut state = vec![lexer.next().unwrap()];
        let mut parser = DmlParser::new(&mut lexer, &mut state);

        let statement = parser.parse();

        assert_eq!(
            statement,
            Ok(AlterSchema::new_statement(Identifier("test".to_string())))
        );
    }

    #[test]
    fn test_show_databases_statement() {
        let mut lexer = Lexer::new("SHOW DATABASES");
        let mut state = vec![lexer.next().unwrap()];
        let mut parser = DmlParser::new(&mut lexer, &mut state);

        let statement = parser.parse();

        assert_eq!(statement, Ok(ShowDatabases::new_statement()));
    }

    #[test]
    fn test_show_schemas_statement() {
        let mut lexer = Lexer::new("SHOW SCHEMAS FROM test");
        let mut state = vec![lexer.next().unwrap()];
        let mut parser = DmlParser::new(&mut lexer, &mut state);

        let statement = parser.parse();

        assert_eq!(
            statement,
            Ok(ShowSchemas::new_statement("test".to_string().into()))
        );
    }
}
