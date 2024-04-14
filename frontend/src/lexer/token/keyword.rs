//! Keyword token module of the lexer.

use std::str::FromStr;

/// Keyword token.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Keyword {
    /// Token for [`DBObject`].
    DbObject(DBObject),

    /// Token for [`Preposition`].
    Preposition(Preposition),
}

impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(db_object) = s.parse::<DBObject>() {
            return Ok(Keyword::DbObject(db_object));
        }

        if let Ok(preposition) = s.parse::<Preposition>() {
            return Ok(Keyword::Preposition(preposition));
        }

        Err(())
    }
}

#[cfg(test)]
mod keyword_tests {
    use crate::lexer::token::keyword::{DBObject, Keyword};

    #[test]
    fn test_keyword_from_str() {
        assert_eq!(
            "database".parse(),
            Ok(Keyword::DbObject(DBObject::Database))
        );
        assert_eq!("schema".parse(), Ok(Keyword::DbObject(DBObject::Schema)));
        assert_eq!("table".parse(), Ok(Keyword::DbObject(DBObject::Table)));
        assert_eq!("column".parse(), Ok(Keyword::DbObject(DBObject::Column)));
        assert_eq!("".parse::<Keyword>(), Err(()));
        assert_eq!("invalid".parse::<Keyword>(), Err(()));
    }

    #[test]
    fn test_keyword_from_str_case_insensitive() {
        assert_eq!(
            "DATabaSE".parse(),
            Ok(Keyword::DbObject(DBObject::Database))
        );
        assert_eq!("SChEMa".parse(), Ok(Keyword::DbObject(DBObject::Schema)));
        assert_eq!("taBLE".parse(), Ok(Keyword::DbObject(DBObject::Table)));
        assert_eq!("COluMN".parse(), Ok(Keyword::DbObject(DBObject::Column)));
    }
}

/// Represents a keyword in the SQL language for the database objects.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DBObject {
    /// Token for `DATABASE` object.
    Database,

    /// Token for `SCHEMA` object.
    Schema,

    /// Token for `TABLE` object.
    Table,

    /// Token for `COLUMN` object.
    Column,
}

impl FromStr for DBObject {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "database" => Ok(DBObject::Database),
            "schema" => Ok(DBObject::Schema),
            "table" => Ok(DBObject::Table),
            "column" => Ok(DBObject::Column),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod dbobject_tests {
    use crate::lexer::token::keyword::DBObject;

    #[test]
    fn test_db_object_from_str() {
        assert_eq!("database".parse(), Ok(DBObject::Database));
        assert_eq!("schema".parse(), Ok(DBObject::Schema));
        assert_eq!("table".parse(), Ok(DBObject::Table));
        assert_eq!("column".parse(), Ok(DBObject::Column));
        assert_eq!("".parse::<DBObject>(), Err(()));
        assert_eq!("invalid".parse::<DBObject>(), Err(()));
    }

    #[test]
    fn test_db_object_from_str_case_insensitive() {
        assert_eq!("DATabaSE".parse(), Ok(DBObject::Database));
        assert_eq!("SChEMa".parse(), Ok(DBObject::Schema));
        assert_eq!("taBLE".parse(), Ok(DBObject::Table));
        assert_eq!("COluMN".parse(), Ok(DBObject::Column));
    }
}

/// Represents a keyword in the SQL language for the prepositions.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Preposition {
    /// Token for `IN` preposition.
    In,

    /// Token for `TO` preposition.
    To,
}

impl FromStr for Preposition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "in" => Ok(Preposition::In),
            "to" => Ok(Preposition::To),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod preposition_tests {
    use crate::lexer::token::keyword::Preposition;

    #[test]
    fn test_preposition_from_str() {
        assert_eq!("in".parse(), Ok(Preposition::In));
        assert_eq!("to".parse(), Ok(Preposition::To));
        assert_eq!("".parse::<Preposition>(), Err(()));
        assert_eq!("invalid".parse::<Preposition>(), Err(()));
    }

    #[test]
    fn test_preposition_from_str_case_insensitive() {
        assert_eq!("iN".parse(), Ok(Preposition::In));
        assert_eq!("tO".parse(), Ok(Preposition::To));
    }
}
