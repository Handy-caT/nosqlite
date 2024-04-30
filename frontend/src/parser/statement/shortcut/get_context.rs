use std::fmt::Display;

use crate::{
    lexer::token::{Shortcut, Token},
    parser::Statement,
    preprocessor::Node,
};

#[derive(Debug, PartialEq, Clone)]
pub struct GetContext;

impl Display for GetContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\\get_context|\\gc")
    }
}

impl GetContext {
    /// Creates a new `Quit` statement.
    /// # Returns
    /// * New instance of `Quit` [`Statement`].
    pub fn new_statement() -> Statement {
        use crate::get_context_statement_variant;

        get_context_statement_variant!(Self)
    }
}

impl Node for GetContext {
    fn can_be_followed(&self, _: &Statement) -> bool {
        false
    }
}

impl TryFrom<&[Token]> for GetContext {
    type Error = ();

    fn try_from(tokens: &[Token]) -> Result<Self, Self::Error> {
        let mut tokens = tokens.iter();
        let quit = tokens.next().ok_or(())?;

        match quit {
            Token::Shortcut(Shortcut::GetContext) => Ok(Self),
            _ => Err(()),
        }
    }
}

/// Shortcut for a [`Quit`] variant of [`Statement`].
#[macro_export]
macro_rules! get_context_statement_variant {
    ($($arg:tt)*) => {
        $crate::parser::Statement::Shortcut(
            $crate::parser::statement::Shortcut::GetContext(
                $($arg)*,
            ),
        )
    };
}
