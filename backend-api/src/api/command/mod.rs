pub mod backend_api;
pub mod database;
pub mod r#enum;
mod extract;
mod gateway;
mod impls;
pub mod schema;
pub mod table;

use crate::Context;
use std::convert::Infallible;

/// Trait for commands.
pub trait Command<Ctx> {
    /// The success type.
    type Ok;
    /// The error type.
    type Err;

    /// Executes the command with the given context.
    fn execute(
        self,
        ctx: &mut Ctx,
    ) -> Result<<Self as Command<Ctx>>::Ok, <Self as Command<Ctx>>::Err>;
}

/// Trait for database commands.
pub trait DatabaseCommand {
    /// Returns the name of the database where the command should be executed.
    fn get_db_name(&self) -> Option<backend::schema::database::Name>;

    /// Returns a mutable reference to the name of the database where the
    /// command should be executed.
    fn get_db_name_mut(
        &mut self,
    ) -> &mut Option<backend::schema::database::Name>;
}

/// Trait for schema commands.
pub trait SchemaCommand: DatabaseCommand {
    /// Returns the name of the schema where the command should be executed.
    fn get_schema_name(&self) -> Option<backend::schema::Name>;

    /// Returns a mutable reference to the name of the schema where the
    /// command should be executed.
    fn get_schema_name_mut(&mut self) -> &mut Option<backend::schema::Name>;
}

/// Trait for table commands.
pub trait TableCommand: SchemaCommand {
    /// Returns the name of the table where the command should be executed.
    fn get_table_name(&self) -> backend::schema::table::Name;

    /// Returns a mutable reference to the name of the table where the
    /// command should be executed.
    fn get_table_name_mut(&mut self) -> &mut backend::schema::table::Name;
}

/// Trait for objects that can be optionally referenced.
pub trait OptionalBy<T> {
    type Err;

    fn by(&self) -> Result<T, Self::Err>;
}

impl<T> OptionalBy<()> for T {
    type Err = Infallible;

    fn by(&self) -> Result<(), Self::Err> {
        Ok(())
    }
}

/// Trait for schema commands.
pub trait ContextReceiver<Ctx> {
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
