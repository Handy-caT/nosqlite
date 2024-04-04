mod database;
mod schema;

use crate::{schema as schema_info, schema::table};

/// Trait for commands that operate on a schema.
pub trait SchemaCommand<Ctx = ()>:
    AsRef<schema_info::Name> + Command<Ctx>
{
}

#[rustfmt::skip]
impl<Ctx, T> SchemaCommand<Ctx> for T where T: AsRef<schema_info::Name> + Command<Ctx>
{}

/// Trait for commands that operate on a table.
pub trait TableCommand<Ctx = ()>: AsRef<table::Name> + Command<Ctx> {}

#[rustfmt::skip]
impl<Ctx, T> TableCommand<Ctx> for T where T: AsRef<table::Name> + Command<Ctx> {}

/// Trait for database backend commands.
pub trait Command<Cmd, Ctx = ()> {
    type Ok;
    type Err;

    /// Executes the command with the given context.
    fn execute(&self, cmd: Cmd, ctx: &mut Ctx) -> Result<Self::Ok, Self::Err>;
}

pub trait Gateway<Cmd, Ctx = ()>
where
    Cmd: Command<Ctx>,
{
    type Ok;
    type Err;

    /// Sends a command to the gateway for the execution.
    fn send(&self, cmd: Cmd) -> Result<Self::Ok, Self::Err>;
}
