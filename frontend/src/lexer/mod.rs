use crate::lexer::token::Token;

mod token;

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
    pub fn new(input: String) -> Self {
        Lexer {
            input: input.trim().to_string(),
            current_position: 0,
            read_position: 0,
        }
    }
    
    /// Skips the whitespace characters in the input source code.
    fn skip_whitespace(&mut self) {
        if self.current_position >= self.input.len() {
            return;
        }
        
        let mut ch = self.input.chars().nth(self.current_position).expect("exists because of the check");
        while ch.is_whitespace() {
            self.current_position += 1;
            if self.current_position >= self.input.len() {
                return;
            }
            ch = self.input.chars().nth(self.current_position).expect("exists because of the check");
        }
    }
    
    /// Reads the next token from the lexer.
    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        if self.current_position >= self.input.len() {
            return None;
        }
        
        self.read_position = self.current_position + 1;
        let mut substr = self.input.get(self.current_position..self.read_position).expect("exists because of the check");
        
        let delimiter = substr.parse::<token::Delimiter>();
        if let Ok(delimiter) = delimiter {
            self.current_position = self.read_position;
            return Some(Token::Delimiter(delimiter));
        }
        
        let mut ch = self.input.chars().nth(self.read_position).expect("exists because of the check");
        while ch.is_alphanumeric() || ch == '_' {
            self.read_position += 1;
            substr = self.input.get(self.current_position..self.read_position).expect("exists because of the check");
            
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
            ch = self.input.chars().nth(self.read_position).expect("exists because of the check");
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
    use crate::lexer::{Lexer, token};
    use crate::lexer::token::Token;
    
    #[test]
    fn test_lexer_next_token_basic() {
        let mut lexer = Lexer::new("CREATE TABLE users;".to_string());
        assert_eq!(lexer.next_token(), Some(Token::DML(token::DMLOperator::Create)));
        assert_eq!(lexer.next_token(), Some(Token::Keyword(token::Keyword::DbObject(token::DBObject::Table))));
        assert_eq!(lexer.next_token(), Some(Token::Identifier(token::Identifier("users".to_string()))));
        assert_eq!(lexer.next_token(), Some(Token::Delimiter(token::Delimiter::Semicolon)));
        assert_eq!(lexer.next_token(), None);
    }
    
    #[test]
    fn test_lexer_next_token_with_whitespace() {
        let lexer = Lexer::new("CREATE   TABLE users;".to_string());
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
        let lexer = Lexer::new("CREATE TABLE users_user;".to_string());
        let expected = vec![
            Token::DML(token::DMLOperator::Create),
            Token::Keyword(token::Keyword::DbObject(token::DBObject::Table)),
            Token::Identifier(token::Identifier("users_user".to_string())),
            Token::Delimiter(token::Delimiter::Semicolon),
        ];

        let actual: Vec<Token> = lexer.collect();

        assert_eq!(actual, expected);
    }
}