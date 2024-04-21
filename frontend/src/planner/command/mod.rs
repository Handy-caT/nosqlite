/// The frontend commands that can be executed by the user.
#[derive(Debug, Clone, PartialEq)]
pub enum FrontendCommand {
    /// Represents a `Quit` command.
    Quit,

    /// Represents a `Help` command.
    Help,

    /// Represents a `Clear` command.
    Clear,
}
