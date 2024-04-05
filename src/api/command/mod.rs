mod database;
mod gateway;
mod schema;

use crate::{
    controller, schema as schema_info,
    schema::{database as database_info, table},
};

/// Trait for commands that operate on a database.
pub trait ExecuteDatabase<Cmd, const NODE_SIZE: u8>:
    AsRef<database_info::Name> + Execute<Cmd, controller::Database<NODE_SIZE>>
{
}

impl<Cmd, T, const NODE_SIZE: u8> ExecuteDatabase<Cmd, NODE_SIZE> for T where
    T: AsRef<database_info::Name>
        + Execute<Cmd, controller::Database<NODE_SIZE>>
{
}

/// Trait for commands that operate on a schema.
pub trait ExecuteSchema<Cmd, const NODE_SIZE: u8>:
    AsRef<schema_info::Name> + Execute<Cmd, controller::Schema<NODE_SIZE>>
{
}

impl<Cmd, T, const NODE_SIZE: u8> ExecuteSchema<Cmd, NODE_SIZE> for T where
    T: AsRef<schema_info::Name> + Execute<Cmd, controller::Schema<NODE_SIZE>>
{
}

/// Trait for commands that operate on a table.
pub trait ExecuteTable<Cmd, const NODE_SIZE: u8>:
    AsRef<table::Name> + Execute<Cmd, controller::Table<NODE_SIZE>>
{
}

impl<Cmd, T, const NODE_SIZE: u8> ExecuteTable<Cmd, NODE_SIZE> for T where
    T: AsRef<table::Name> + Execute<Cmd, controller::Table<NODE_SIZE>>
{
}

/// Trait for database backend commands.
pub trait Execute<Cmd, Ctx = ()> {
    type Ok;
    type Err;

    /// Executes the command with the given context.
    fn execute(cmd: Cmd, ctx: &mut Ctx) -> Result<Self::Ok, Self::Err>;
}

/// Trait for commands.
pub trait Command {}

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

/// Represents an error that occurred during the execution of a command.
pub enum GatewayError<CmdErr, GatewayErr> {
    Cmd(CmdErr),
    Gateway(GatewayErr),
}
