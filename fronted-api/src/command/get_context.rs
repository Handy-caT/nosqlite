use backend_api::context::Context;
use prettytable::{format, row};
use std::convert::Infallible;

use crate::{
    api::Api,
    command::{Command, Execute},
};

#[derive(Debug, Clone, PartialEq)]
pub struct GetContext;

impl Command for GetContext {}

impl<const NODE_SIZE: u8> Execute<GetContext, Context> for Api<NODE_SIZE> {
    type Ok = ();
    type Err = ExecutionError;

    fn execute(
        _: GetContext,
        ctx: &mut Context,
    ) -> Result<Self::Ok, Self::Err> {
        let mut table = prettytable::Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

        table.set_titles(row!["database", "schema"]);
        
        let db = ctx.current_db().map(|db| db.0.as_str()).unwrap_or("None");
        let schema = ctx.current_schema().map(|schema| schema.0.as_str()).unwrap_or("None");
        
        table.add_row(row![db, schema]);

        table.printstd();
        Ok(())
    }
}

pub type ExecutionError = Infallible;
