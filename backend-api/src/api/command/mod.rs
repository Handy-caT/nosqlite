pub mod backend_api;
pub mod database;
pub mod r#enum;
mod gateway;
pub mod schema;
mod extract;

use std::fmt::Debug;

use backend::{
    controller, schema as schema_info,
    schema::{database as database_info, table},
};

use crate::api::facade::BackendFacade;

/// Trait for commands.
pub trait Command<Ctx> {
    type Ok;
    type Err;

    fn execute(&self, ctx: &mut Ctx) -> Result<<Self as Command<Ctx>>::Ok, <Self as Command<Ctx>>::Err>;
}

pub trait Extract<Ctx> {
    fn extract(&self) -> &mut Ctx;
}

pub trait TryExtract<Ctx> {
    type Err;

    fn try_extract(&self) -> Result<&mut Ctx, Self::Err>;
}

pub trait Gateway<Cmd, Ctx = ()>
where
    Cmd: Command<Ctx>,
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

/// Represents an error that occurred during the execution of a command.
#[derive(Debug)]
pub enum GatewayError<CmdErr, GatewayErr> {
    Cmd(CmdErr),
    Gateway(GatewayErr),
}