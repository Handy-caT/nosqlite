use std::str::FromStr;

/// Represents a shortcut for frontend commands.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Shortcut {
    /// Token for `Help` command.
    Help,

    /// Token for `Quit` command.
    Quit,

    /// Token for `Clear` command.
    Clear,

    /// Token for `GetContext` command.
    GetContext,
}

impl FromStr for Shortcut {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('\\') {
            return Err(());
        }

        match s.to_lowercase().as_str() {
            "\\h" | "\\help" => Ok(Shortcut::Help),
            "\\q" | "\\quit" => Ok(Shortcut::Quit),
            "\\c" | "\\clear" => Ok(Shortcut::Clear),
            "\\gc" | "\\get_context" => Ok(Shortcut::GetContext),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod shortcut_tests {
    use crate::lexer::token::shortcut::Shortcut;

    #[test]
    fn test_shortcut_from_str() {
        assert_eq!("\\help".parse(), Ok(Shortcut::Help));
        assert_eq!("\\quit".parse(), Ok(Shortcut::Quit));
        assert_eq!("\\clear".parse(), Ok(Shortcut::Clear));
        assert_eq!("\\get_context".parse(), Ok(Shortcut::GetContext));

        assert_eq!("\\h".parse(), Ok(Shortcut::Help));
        assert_eq!("\\q".parse(), Ok(Shortcut::Quit));
        assert_eq!("\\c".parse(), Ok(Shortcut::Clear));
        assert_eq!("\\gc".parse(), Ok(Shortcut::GetContext));

        assert_eq!("".parse::<Shortcut>(), Err(()));
        assert_eq!("invalid".parse::<Shortcut>(), Err(()));
    }

    #[test]
    fn test_shortcut_from_str_case_insensitive() {
        assert_eq!("\\hElP".parse(), Ok(Shortcut::Help));
        assert_eq!("\\qUiT".parse(), Ok(Shortcut::Quit));
        assert_eq!("\\cLeAr".parse(), Ok(Shortcut::Clear));
    }

    #[test]
    fn test_shortcut_from_str_not_starting_with_backslash() {
        assert_eq!("help".parse::<Shortcut>(), Err(()));
        assert_eq!("quit".parse::<Shortcut>(), Err(()));
        assert_eq!("clear".parse::<Shortcut>(), Err(()));
    }
}
