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
pub trait OptionalRef<T> {
    fn as_ref(&self) -> Option<&T>;
}

impl<T> OptionalRef<()> for T {
    fn as_ref(&self) -> Option<&()> {
        Some(&())
    }
}

/// Trait for schema commands.
pub trait ContextReceiver {
    /// Receives the context from API.
    fn receive(&mut self, _: &Context) {}
}

pub trait Extract<Ctx> {
    fn extract(&mut self) -> &mut Ctx;
}

pub trait TryExtractBy<Ctx> {
    type Err;
    type By;

    fn try_extract(&mut self, by: &Self::By) -> Result<&mut Ctx, Self::Err>;
}

impl<Ctx, T> TryExtractBy<Ctx> for T
where
    T: Extract<Ctx>,
{
    type Err = Infallible;
    type By = ();

    fn try_extract(&mut self, (): &()) -> Result<&mut Ctx, Self::Err> {
        Ok(self.extract())
    }
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
