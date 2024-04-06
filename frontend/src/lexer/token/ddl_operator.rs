//! DDL operator token module of the lexer.

use std::str::FromStr;

/// DDL operator token.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DDLOperator {
    /// Token for `SELECT` statement.
    Select,

    /// Token for `INSERT` statement.
    Insert,

    /// Token for `UPDATE` statement.
    Update,

    /// Token for `DELETE` statement.
    Delete,
}

impl FromStr for DDLOperator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "select" => Ok(DDLOperator::Select),
            "insert" => Ok(DDLOperator::Insert),
            "update" => Ok(DDLOperator::Update),
            "delete" => Ok(DDLOperator::Delete),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod ddl_operator_tests {
    use crate::lexer::token::ddl_operator::DDLOperator;

    #[test]
    fn test_ddl_operator_from_str() {
        assert_eq!("SELECT".parse(), Ok(DDLOperator::Select));
        assert_eq!("INSERT".parse(), Ok(DDLOperator::Insert));
        assert_eq!("UPDATE".parse(), Ok(DDLOperator::Update));
        assert_eq!("DELETE".parse(), Ok(DDLOperator::Delete));
        assert_eq!("".parse::<DDLOperator>(), Err(()));
        assert_eq!("invalid".parse::<DDLOperator>(), Err(()));
    }

    #[test]
    fn test_ddl_operator_from_str_case_insensitive() {
        assert_eq!("sElEcT".parse(), Ok(DDLOperator::Select));
        assert_eq!("iNsErT".parse(), Ok(DDLOperator::Insert));
        assert_eq!("uPdAtE".parse(), Ok(DDLOperator::Update));
        assert_eq!("dElEtE".parse(), Ok(DDLOperator::Delete));
    }
}
