use derive_more::Display;

pub mod command;
pub mod facade;

/// Represents a command result that returns a string.
#[derive(Debug, Display, Default, Clone, PartialEq)]
pub struct CommandResultString {
    /// The result of the command.
    pub result: String,
}
