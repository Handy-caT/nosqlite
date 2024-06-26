use crate::lexer::token::Token;

pub mod token;

/// [`Lexer`] is a struct that represents a lexer for the source code.
#[derive(Debug, PartialEq, Clone)]
pub struct Lexer {
    /// The input source code string.
    input: String,

    /// The current position of the lexer.
    current_position: usize,

    /// The read position of the lexer.
    read_position: usize,
}

impl Lexer {
    /// Creates a new lexer with the given input source code.
    /// # Arguments
    /// * `input` - The input source code string.
    /// # Returns
    /// A new lexer with the given input source code.
    pub fn new<T>(input: T) -> Self
    where
        T: AsRef<str>,
    {
        Lexer {
            input: input.as_ref().trim().to_string(),
            current_position: 0,
            read_position: 0,
        }
    }

    /// Skips the whitespace characters in the input source code.
    fn skip_whitespace(&mut self) {
        if self.current_position >= self.input.len() {
            return;
        }

        let mut ch = self
            .input
            .chars()
            .nth(self.current_position)
            .expect("exists because of the check");
        while ch.is_whitespace() {
            self.current_position += 1;
            if self.current_position >= self.input.len() {
                return;
            }
            ch = self
                .input
                .chars()
                .nth(self.current_position)
                .expect("exists because of the check");
        }
    }

    /// Skips the alphanumeric characters in the input source code.
    fn read_alphanumeric(&mut self) {
        if self.read_position >= self.input.len() {
            return;
        }

        let mut ch = self
            .input
            .chars()
            .nth(self.read_position)
            .expect("exists because of the check");
        while ch.is_alphanumeric() || ch == '_' || ch == '.' {
            self.read_position += 1;
            if self.read_position >= self.input.len() {
                return;
            }
            ch = self
                .input
                .chars()
                .nth(self.read_position)
                .expect("exists because of the check");
        }
    }

    /// Reads the next token from the lexer.
    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        if self.current_position >= self.input.len() {
            return None;
        }

        self.read_position = self.current_position + 1;
        {
            let substr = self
                .input
                .get(self.current_position..self.read_position)
                .expect("exists because of the check");

            let delimiter = substr.parse::<token::Delimiter>();
            if let Ok(delimiter) = delimiter {
                self.current_position = self.read_position;
                return Some(Token::Delimiter(delimiter));
            }
        }

        self.read_alphanumeric();
        let substr = self
            .input
            .get(self.current_position..self.read_position)
            .expect("exists because of the check");

        let shortcut = substr.parse::<token::Shortcut>();
        if let Ok(shortcut) = shortcut {
            self.current_position = self.read_position;
            return Some(Token::Shortcut(shortcut));
        }

        let dml = substr.parse::<token::DMLOperator>();
        if let Ok(dml) = dml {
            self.current_position = self.read_position;
            return Some(Token::DML(dml));
        }

        let ddl = substr.parse::<token::DDLOperator>();
        if let Ok(ddl) = ddl {
            self.current_position = self.read_position;
            return Some(Token::DDL(ddl));
        }

        let keyword = substr.parse::<token::Keyword>();
        if let Ok(keyword) = keyword {
            self.current_position = self.read_position;
            return Some(Token::Keyword(keyword));
        }

        let data_type = substr.parse::<token::DataType>();
        if let Ok(data_type) = data_type {
            self.current_position = self.read_position;
            return Some(Token::DataType(data_type));
        }

        self.current_position = self.read_position;

        let identifier = token::Identifier(substr.to_string());
        Some(Token::Identifier(identifier))
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod lexer_tests {
    use crate::lexer::{
        token,
        token::{Preposition, Token},
        Lexer,
    };

    #[test]
    fn test_lexer_next_token_basic() {
        let mut lexer = Lexer::new("CREATE TABLE users;");
        assert_eq!(
            lexer.next_token(),
            Some(Token::DML(token::DMLOperator::Create))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::Keyword(token::Keyword::DbObject(
                token::DBObject::Table
            )))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::Identifier(token::Identifier("users".to_string())))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::Delimiter(token::Delimiter::Semicolon))
        );
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_lexer_next_token_with_whitespace() {
        let lexer = Lexer::new("CREATE   TABLE users;");
        let expected = vec![
            Token::DML(token::DMLOperator::Create),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Table)),
            Token::Identifier(token::Identifier("users".to_string())),
            Token::Delimiter(token::Delimiter::Semicolon),
        ];

        let actual: Vec<Token> = lexer.collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_lexer_next_token_identifier_with_underscore() {
        let lexer = Lexer::new("CREATE TABLE users_user;");
        let expected = vec![
            Token::DML(token::DMLOperator::Create),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Table)),
            Token::Identifier(token::Identifier("users_user".to_string())),
            Token::Delimiter(token::Delimiter::Semicolon),
        ];

        let actual: Vec<Token> = lexer.collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_lexer_next_token_identifier_with_dot() {
        let lexer = Lexer::new("CREATE TABLE users.user;");
        let expected = vec![
            Token::DML(token::DMLOperator::Create),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Table)),
            Token::Identifier(token::Identifier("users.user".to_string())),
            Token::Delimiter(token::Delimiter::Semicolon),
        ];

        let actual: Vec<Token> = lexer.collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_lexer_next_token_identifier_with_keyword() {
        let lexer = Lexer::new("CREATE TABLE create_user;");
        let expected = vec![
            Token::DML(token::DMLOperator::Create),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Table)),
            Token::Identifier(token::Identifier("create_user".to_string())),
            Token::Delimiter(token::Delimiter::Semicolon),
        ];

        let actual: Vec<Token> = lexer.collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_lexer_next_token_delimiters_without_spaces() {
        let lexer = Lexer::new("CREATE,TABLE,create_user;");
        let expected = vec![
            Token::DML(token::DMLOperator::Create),
            Token::Delimiter(token::Delimiter::Comma),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Table)),
            Token::Delimiter(token::Delimiter::Comma),
            Token::Identifier(token::Identifier("create_user".to_string())),
            Token::Delimiter(token::Delimiter::Semicolon),
        ];

        let actual: Vec<Token> = lexer.collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_lexer_keywords() {
        let lexer = Lexer::new("ALTER TABLE user RENAME TO user1;");
        let expected = vec![
            Token::DML(token::DMLOperator::Alter),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Table)),
            Token::Identifier(token::Identifier("user".to_string())),
            Token::DML(token::DMLOperator::Rename),
            Token::Keyword(token::Keyword::Preposition(Preposition::To)),
            Token::Identifier(token::Identifier("user1".to_string())),
            Token::Delimiter(token::Delimiter::Semicolon),
        ];

        let actual: Vec<Token> = lexer.collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_lexer_shortcuts() {
        let lexer = Lexer::new("\\q \\quit");
        let expected = vec![
            Token::Shortcut(token::Shortcut::Quit),
            Token::Shortcut(token::Shortcut::Quit),
        ];

        let actual: Vec<Token> = lexer.collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_lexer_show() {
        let lexer = Lexer::new("SHOW SCHEMAS FROM db");
        let expected = vec![
            Token::DML(token::DMLOperator::Show),
            Token::Keyword(token::Keyword::DbObjectMany(
                token::DBObjectMany::Schemas,
            )),
            Token::Keyword(token::Keyword::Preposition(Preposition::From)),
            Token::Identifier(token::Identifier("db".to_string())),
        ];

        let actual: Vec<Token> = lexer.collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_lexer_next_token_key_datatype() {
        let lexer = Lexer::new(
            "CREATE TABLE users(id INT4 PRIMARY \
                                       KEY);",
        );
        let expected = vec![
            Token::DML(token::DMLOperator::Create),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Table)),
            Token::Identifier(token::Identifier("users".to_string())),
            Token::Delimiter(token::Delimiter::LeftParenthesis),
            Token::Identifier(token::Identifier("id".to_string())),
            Token::DataType(token::DataType::Integer),
            Token::Keyword(token::Keyword::Key(token::Key::Primary)),
            Token::Keyword(token::Keyword::Key(token::Key::Key)),
            Token::Delimiter(token::Delimiter::RightParenthesis),
            Token::Delimiter(token::Delimiter::Semicolon),
        ];

        let actual: Vec<Token> = lexer.collect();

        assert_eq!(actual, expected);
    }
}
