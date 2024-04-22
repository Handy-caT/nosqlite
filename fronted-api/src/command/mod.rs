use frontend::planner::command::FrontendCommand;

use crate::api::Api;

mod gateway;
mod get_context;
mod quit;

/// Trait for database frontend commands.
pub trait Execute<Cmd, Ctx = ()> {
    type Ok;
    type Err;

    /// Executes the command with the given context.
    fn execute(cmd: Cmd, ctx: &mut Ctx) -> Result<Self::Ok, Self::Err>;
}

/// Trait for commands.
pub trait Command {}

/// Trait for extracting the context.
pub trait Extract<Ctx> {
    fn extract(&mut self) -> &mut Ctx;
}

pub trait Gateway<Cmd, Ctx = ()>
where
    Self: Execute<Cmd, Ctx>,
{
    type Ok;
    type Err;

    /// Sends a command to the gateway for the execution.
    #[rustfmt::skip]
    fn send(
        &mut self,
        cmd: Cmd,
    ) -> Result<
        <Self as Gateway<Cmd, Ctx>>::Ok,
        <Self as Gateway<Cmd, Ctx>>::Err,
    >;
}

/// Executes a frontend command.
pub fn execute_frontend_command<const NODE_SIZE: u8>(
    api: &mut Api<NODE_SIZE>,
    cmd: FrontendCommand,
) -> Result<(), ()> {
    match cmd {
        FrontendCommand::Quit => {
            let cmd = quit::Quit;
            api.send(cmd).map_err(|_| ())
        }
        FrontendCommand::Help => {
            todo!()
        }
        FrontendCommand::Clear => {
            todo!()
        }
        FrontendCommand::GetContext => {
            let cmd = get_context::GetContext;
            api.send(cmd).map_err(|_| ())
        }
    }
}
