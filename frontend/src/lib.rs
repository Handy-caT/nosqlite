use crate::planner::{adapter::PlannerCommand, Planner, PlannerError};

pub mod lexer;
pub mod parser;
pub mod planner;
pub mod preprocessor;

/// Represents the frontend API.
#[derive(Debug)]
pub struct FrontendApi {
    /// Represents the preprocessor.
    planner: Planner,
}

impl Default for FrontendApi {
    fn default() -> Self {
        Self {
            planner: Planner::new(""),
        }
    }
}

impl FrontendApi {
    /// Creates a new frontend API.
    /// # Arguments
    /// * `input` - The input.
    pub fn new<T>(input: T) -> Self
    where
        T: AsRef<str>,
    {
        Self {
            planner: Planner::new(input),
        }
    }

    /// Returns the next command.
    pub fn next_command(
        &mut self,
    ) -> Option<Result<PlannerCommand, PlannerError>> {
        self.planner.next()
    }

    /// Returns all commands in the input.
    pub fn commands(&mut self) -> Vec<Result<PlannerCommand, PlannerError>> {
        let mut commands = Vec::new();
        while let Some(command) = self.next_command() {
            commands.push(command);
        }
        commands
    }

    /// Sets the input.
    pub fn set_input<T>(&mut self, input: T)
    where
        T: AsRef<str>,
    {
        self.planner = Planner::new(input);
    }
}

impl Iterator for FrontendApi {
    type Item = Result<PlannerCommand, PlannerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_command()
    }
}
