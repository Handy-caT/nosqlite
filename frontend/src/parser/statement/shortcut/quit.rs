use crate::lexer::token;
use std::fmt::Display;

use crate::{
    lexer::token::{Shortcut, Token},
    parser::{statement::common::RenameTo, Statement},
    preprocessor::{LeafNode, Node},
    rename_to_statement_variant,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Quit;

impl Display for Quit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\\quit")
    }
}

impl Quit {
    /// Creates a new `Quit` statement.
    /// # Returns
    /// * New instance of `Quit` [`Statement`].
    pub fn new_statement() -> Statement {
        use crate::quit_statement_variant;

        quit_statement_variant!(Self)
    }
}

impl Node for Quit {
    fn can_be_followed(&self, _: &Statement) -> bool {
        false
    }
}

impl TryFrom<&[Token]> for Quit {
    type Error = ();

    fn try_from(tokens: &[Token]) -> Result<Self, Self::Error> {
        let mut tokens = tokens.iter();
        let quit = tokens.next().ok_or(())?;

        match quit {
            Token::Shortcut(Shortcut::Quit) => Ok(Self),
            _ => Err(()),
        }
    }
}

/// Shortcut for a [`Quit`] variant of [`Statement`].
#[macro_export]
macro_rules! quit_statement_variant {
    ($($arg:tt)*) => {
        $crate::parser::Statement::Shortcut(
            $crate::parser::statement::Shortcut::Quit(
                $($arg)*,
            ),
        )
    };
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::{token, token::Token},
        parser::statement::{common::RenameTo, dml::CreateDatabase},
        preprocessor::Node,
    };

    use super::Quit;

    #[test]
    fn test_rename_to_try_from_token_vec_basic() {
        let tokens = vec![Token::Shortcut(token::Shortcut::Quit)];

        let actual = Quit::try_from(tokens.as_slice());
        let expected = Ok(Quit);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_rename_to_try_from_token_vec_invalid_tokens() {
        let tokens =
            vec![Token::Identifier(token::Identifier("test".to_string()))];

        let actual = RenameTo::try_from(tokens.as_slice());
        let expected = Err(());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_quit_to_cant_be_followed_by_nothing() {
        let quit = Quit;

        let identifier = token::Identifier("test".to_string());

        assert!(!quit.can_be_followed(&CreateDatabase::new_statement(
            identifier.clone()
        )));
        assert!(!quit.can_be_followed(&RenameTo::new_statement(identifier)));
    }
}
