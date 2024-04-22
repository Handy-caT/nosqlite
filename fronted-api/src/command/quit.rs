use std::convert::Infallible;

use crate::api::{Api, Context};
use crate::command::{Command, Execute};

#[derive(Debug, Clone, PartialEq)]
pub struct Quit;

impl Command for Quit {}

impl<const NODE_SIZE: u8> Execute<Quit, Context> for Api<NODE_SIZE>
{
    type Ok = ();
    type Err = ExecutionError;

    fn execute(
        _: Quit,
        ctx: &mut Context,
    ) -> Result<Self::Ok, Self::Err> {
        ctx.quit = true;
        Ok(())
    }
}

pub type ExecutionError = Infallible;
