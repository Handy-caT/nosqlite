use crate::{schema, schema::table};

/// Trait for commands that operate on a schema.
pub trait SchemaCommand<Ctx = ()>: AsRef<schema::Name> + Command<Ctx> {}

#[rustfmt::skip]
impl<Ctx, T> SchemaCommand<Ctx> for T where T: AsRef<schema::Name> + Command<Ctx>
{}

/// Trait for commands that operate on a table.
pub trait TableCommand<Ctx = ()>: AsRef<table::Name> + Command<Ctx> {}

#[rustfmt::skip]
impl<Ctx, T> TableCommand<Ctx> for T where T: AsRef<table::Name> + Command<Ctx> {}

/// Trait for database backend commands.
pub trait Command<Ctx = ()> {
    type Ok;
    type Err;

    /// Executes the command with the given context.
    fn execute(&self, ctx: &Ctx) -> Result<Self::Ok, Self::Err>;
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

pub trait ExtractFrom<Val, By> {
    fn extract_from(by: By) -> Val;
}
