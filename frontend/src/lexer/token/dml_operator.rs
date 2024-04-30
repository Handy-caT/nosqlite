//! DML operator token module of the lexer.

use std::str::FromStr;

use derive_more::Display;

/// DML operator token.
#[derive(Debug, Display, PartialEq, Clone, Copy)]
pub enum DMLOperator {
    /// Token for `CREATE` statement.
    #[display(fmt = "CREATE")]
    Create,

    /// Token for `ALTER` statement.
    #[display(fmt = "ALTER")]
    Alter,

    /// Token for `RENAME` statement.
    #[display(fmt = "RENAME")]
    Rename,

    /// Token for `DROP` statement.
    #[display(fmt = "DROP")]
    Drop,

    /// Token for `USE` statement.
    #[display(fmt = "USE")]
    Use,
}

impl FromStr for DMLOperator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "create" => Ok(DMLOperator::Create),
            "alter" => Ok(DMLOperator::Alter),
            "rename" => Ok(DMLOperator::Rename),
            "drop" => Ok(DMLOperator::Drop),
            "use" => Ok(DMLOperator::Use),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod dml_operator_tests {
    use crate::lexer::token::dml_operator::DMLOperator;

    #[test]
    fn test_dml_operator_from_str() {
        assert_eq!("CREATE".parse(), Ok(DMLOperator::Create));
        assert_eq!("ALTER".parse(), Ok(DMLOperator::Alter));
        assert_eq!("RENAME".parse(), Ok(DMLOperator::Rename));
        assert_eq!("DROP".parse(), Ok(DMLOperator::Drop));
        assert_eq!("USE".parse(), Ok(DMLOperator::Use));

        assert_eq!("".parse::<DMLOperator>(), Err(()));
        assert_eq!("invalid".parse::<DMLOperator>(), Err(()));
    }

    #[test]
    fn test_dml_operator_from_str_case_insensitive() {
        assert_eq!("cReAtE".parse(), Ok(DMLOperator::Create));
        assert_eq!("aLtEr".parse(), Ok(DMLOperator::Alter));
        assert_eq!("rEnAmE".parse(), Ok(DMLOperator::Rename));
        assert_eq!("dRoP".parse(), Ok(DMLOperator::Drop));
        assert_eq!("uSe".parse(), Ok(DMLOperator::Use));
    }
}
