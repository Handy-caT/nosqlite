pub mod backend_api;
pub mod database;
pub mod r#enum;
mod extract;
mod gateway;
pub mod schema;

use crate::Context;
use std::convert::Infallible;

/// Trait for commands.
pub trait Command<Ctx> {
    type Ok;
    type Err;

    fn execute(
        self,
        ctx: &mut Ctx,
    ) -> Result<<Self as Command<Ctx>>::Ok, <Self as Command<Ctx>>::Err>;
}

/// Trait for objects that can be optionally referenced.
pub trait OptionalBy<T> {
    fn by(&self) -> Option<T>;
}

impl<T> OptionalBy<()> for T {
    fn by(&self) -> Option<()> {
        Some(())
    }
}

/// Trait for schema commands.
pub trait ContextReceiver {
    /// Receives the context from API.
    fn receive(&mut self, _: &Context) {}
}

pub trait Extract<Ctx> {
    fn extract_mut(&mut self) -> &mut Ctx;
}

pub trait TryExtract<Ctx> {
    type Err;
    type By;

    fn try_extract_mut(&mut self, by: Self::By) -> Result<&mut Ctx, Self::Err>;
}

impl<Ctx, T> TryExtract<Ctx> for T
where
    T: Extract<Ctx>,
{
    type Err = Infallible;
    type By = ();

    fn try_extract_mut(&mut self, (): ()) -> Result<&mut Ctx, Self::Err> {
        Ok(self.extract_mut())
    }
}

pub trait Gateway<Cmd, CtxMut = ()>
where
    Cmd: Command<CtxMut>,
{
    type Ok;
    type Err;

    /// Sends a command to the gateway for the execution.
    fn send(
        &mut self,
        cmd: Cmd,
    ) -> Result<
        <Self as Gateway<Cmd, CtxMut>>::Ok,
        <Self as Gateway<Cmd, CtxMut>>::Err,
    >;
}
