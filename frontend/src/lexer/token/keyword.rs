//! Keyword token module of the lexer.

use std::str::FromStr;

use derive_more::Display;

/// Keyword token.
#[derive(Debug, Display, PartialEq, Clone, Copy)]
pub enum Keyword {
    /// Token for [`DBObject`].
    DbObject(DBObject),

    /// Token for [`DBObjectMany`].
    DbObjectMany(DBObjectMany),

    /// Token for [`Preposition`].
    Preposition(Preposition),

    /// Token for [`Key`].
    Key(Key),
}

impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(db_object) = s.parse::<DBObject>() {
            return Ok(Keyword::DbObject(db_object));
        }

        if let Ok(db_object_many) = s.parse::<DBObjectMany>() {
            return Ok(Keyword::DbObjectMany(db_object_many));
        }

        if let Ok(preposition) = s.parse::<Preposition>() {
            return Ok(Keyword::Preposition(preposition));
        }

        if let Ok(key) = s.parse::<Key>() {
            return Ok(Keyword::Key(key));
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
#[derive(Debug, Display, PartialEq, Clone, Copy)]
pub enum DBObject {
    /// Token for `DATABASE` object.
    #[display(fmt = "DATABASE")]
    Database,

    /// Token for `SCHEMA` object.
    #[display(fmt = "SCHEMA")]
    Schema,

    /// Token for `TABLE` object.
    #[display(fmt = "TABLE")]
    Table,

    /// Token for `COLUMN` object.
    #[display(fmt = "COLUMN")]
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

/// Represents a keyword in the SQL language for the database objects.
#[derive(Debug, Display, PartialEq, Clone, Copy)]
pub enum DBObjectMany {
    /// Token for `DATABASE` object.
    #[display(fmt = "DATABASES")]
    Databases,

    /// Token for `SCHEMA` object.
    #[display(fmt = "SCHEMAS")]
    Schemas,

    /// Token for `TABLE` object.
    #[display(fmt = "TABLES")]
    Tables,

    /// Token for `COLUMN` object.
    #[display(fmt = "COLUMNS")]
    Columns,
}

impl FromStr for DBObjectMany {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "databases" => Ok(DBObjectMany::Databases),
            "schemas" => Ok(DBObjectMany::Schemas),
            "tables" => Ok(DBObjectMany::Tables),
            "columns" => Ok(DBObjectMany::Columns),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod dbobject_many_tests {
    use crate::lexer::token::keyword::DBObjectMany;

    #[test]
    fn test_db_object_from_str() {
        assert_eq!("databases".parse(), Ok(DBObjectMany::Databases));
        assert_eq!("schemas".parse(), Ok(DBObjectMany::Schemas));
        assert_eq!("tables".parse(), Ok(DBObjectMany::Tables));
        assert_eq!("columns".parse(), Ok(DBObjectMany::Columns));
        assert_eq!("".parse::<DBObjectMany>(), Err(()));
        assert_eq!("invalid".parse::<DBObjectMany>(), Err(()));
    }

    #[test]
    fn test_db_object_from_str_case_insensitive() {
        assert_eq!("DATabaSES".parse(), Ok(DBObjectMany::Databases));
        assert_eq!("SChEMaS".parse(), Ok(DBObjectMany::Schemas));
        assert_eq!("taBLES".parse(), Ok(DBObjectMany::Tables));
        assert_eq!("COluMNS".parse(), Ok(DBObjectMany::Columns));
    }
}

/// Represents a keyword in the SQL language for the prepositions.
#[derive(Debug, Display, PartialEq, Clone, Copy)]
pub enum Preposition {
    /// Token for `IN` preposition.
    #[display(fmt = "IN")]
    In,

    /// Token for `TO` preposition.
    #[display(fmt = "TO")]
    To,

    /// Token for `FROM` preposition.
    #[display(fmt = "FROM")]
    From,
}

impl FromStr for Preposition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "in" => Ok(Preposition::In),
            "to" => Ok(Preposition::To),
            "from" => Ok(Preposition::From),
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
        assert_eq!("from".parse(), Ok(Preposition::From));

        assert_eq!("".parse::<Preposition>(), Err(()));
        assert_eq!("invalid".parse::<Preposition>(), Err(()));
    }

    #[test]
    fn test_preposition_from_str_case_insensitive() {
        assert_eq!("iN".parse(), Ok(Preposition::In));
        assert_eq!("tO".parse(), Ok(Preposition::To));
        assert_eq!("fRoM".parse(), Ok(Preposition::From));
    }
}

/// Represents a keyword in the SQL language for the keys.
#[derive(Debug, Display, PartialEq, Clone, Copy)]
pub enum Key {
    /// Token for `PRIMARY` key.
    #[display(fmt = "PRIMARY")]
    Primary,

    /// Token for `FOREIGN` key.
    #[display(fmt = "FOREIGN")]
    Foreign,

    /// Token for `Key`.
    #[display(fmt = "KEY")]
    Key,
}

impl FromStr for Key {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "primary" => Ok(Key::Primary),
            "foreign" => Ok(Key::Foreign),
            "key" => Ok(Key::Key),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod key_tests {
    use crate::lexer::token::keyword::Key;

    #[test]
    fn test_key_from_str() {
        assert_eq!("primary".parse(), Ok(Key::Primary));
        assert_eq!("foreign".parse(), Ok(Key::Foreign));
        assert_eq!("key".parse(), Ok(Key::Key));

        assert_eq!("".parse::<Key>(), Err(()));
        assert_eq!("invalid".parse::<Key>(), Err(()));
    }

    #[test]
    fn test_key_from_str_case_insensitive() {
        assert_eq!("pRiMaRy".parse(), Ok(Key::Primary));
        assert_eq!("fOrEiGn".parse(), Ok(Key::Foreign));
        assert_eq!("kEy".parse(), Ok(Key::Key));
    }
}
