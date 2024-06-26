//! Delimiter token module of the lexer.

use derive_more::Display;
use std::str::FromStr;

/// Delimiter is a token that represents a delimiter in the source code.
#[derive(Debug, Display, PartialEq, Clone, Copy)]
pub enum Delimiter {
    /// Token for `,` delimiter.
    #[display(fmt = ",")]
    Comma,

    /// Token for `;` delimiter.
    #[display(fmt = ";")]
    Semicolon,

    /// Token for `(` delimiter.
    #[display(fmt = "(")]
    LeftParenthesis,

    /// Token for `)` delimiter.
    #[display(fmt = ")")]
    RightParenthesis,
}

impl FromStr for Delimiter {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "," => Ok(Delimiter::Comma),
            ";" => Ok(Delimiter::Semicolon),
            "(" => Ok(Delimiter::LeftParenthesis),
            ")" => Ok(Delimiter::RightParenthesis),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod delimiter_tests {
    use crate::lexer::token::delimiter::Delimiter;

    #[test]
    fn test_delimiter_from_str() {
        assert_eq!(",".parse(), Ok(Delimiter::Comma));
        assert_eq!(";".parse(), Ok(Delimiter::Semicolon));
        assert_eq!("(".parse(), Ok(Delimiter::LeftParenthesis));
        assert_eq!(")".parse(), Ok(Delimiter::RightParenthesis));
        assert_eq!("".parse::<Delimiter>(), Err(()));
        assert_eq!("invalid".parse::<Delimiter>(), Err(()));
    }
}
