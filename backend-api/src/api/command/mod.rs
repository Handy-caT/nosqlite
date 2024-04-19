pub mod backend_api;
pub mod database;
pub mod r#enum;
mod gateway;
pub mod schema;

use std::fmt::Debug;

use backend::{
    controller, schema as schema_info,
    schema::{database as database_info, table},
};

use crate::api::facade::BackendFacade;

/// Trait for commands that operate on a full backend instance.
pub trait ExecuteBackend<Cmd, const NODE_SIZE: u8>:
    Execute<Cmd, BackendFacade<NODE_SIZE>>
{
}

impl<Cmd, T, const NODE_SIZE: u8> ExecuteBackend<Cmd, NODE_SIZE> for T where
    T: Execute<Cmd, BackendFacade<NODE_SIZE>>
{
}

/// Trait for commands that operate on a database.
pub trait ExecuteDatabase<Cmd, const NODE_SIZE: u8>:
    Execute<Cmd, controller::Database<NODE_SIZE>>
{
}

impl<Cmd, T, const NODE_SIZE: u8> ExecuteDatabase<Cmd, NODE_SIZE> for T
where
    T: Execute<Cmd, controller::Database<NODE_SIZE>>,
    Cmd: AsRef<database_info::Name>,
{
}

/// Trait for commands that operate on a schema.
pub trait ExecuteSchema<Cmd, const NODE_SIZE: u8>:
    Execute<Cmd, controller::Schema<NODE_SIZE>>
{
}

impl<Cmd, T, const NODE_SIZE: u8> ExecuteSchema<Cmd, NODE_SIZE> for T
where
    T: Execute<Cmd, controller::Schema<NODE_SIZE>>,
    Cmd: AsRef<schema_info::Name>,
{
}

/// Trait for commands that operate on a table.
pub trait ExecuteTable<Cmd, const NODE_SIZE: u8>:
    Execute<Cmd, controller::Table<NODE_SIZE>>
{
}

impl<Cmd, T, const NODE_SIZE: u8> ExecuteTable<Cmd, NODE_SIZE> for T
where
    T: Execute<Cmd, controller::Table<NODE_SIZE>>,
    Cmd: AsRef<table::Name>,
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
#[derive(Debug)]
pub enum GatewayError<CmdErr, GatewayErr> {
    Cmd(CmdErr),
    Gateway(GatewayErr),
}
