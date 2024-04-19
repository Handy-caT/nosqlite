use crate::lexer::token;
use backend::schema::database as db;

mod database;

impl From<token::Identifier> for db::Name {
    fn from(identifier: token::Identifier) -> Self {
        db::Name(identifier.0)
    }
}
