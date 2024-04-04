use derive_more::AsRef;

use crate::{
    api::{command::Command, facade::BackendFacade},
    controller, schema,
};

#[derive(Debug, AsRef, Clone)]
pub struct CreateSchema {
    #[as_ref]
    pub name: schema::Name,
}

impl<const NODE_SIZE: u8> Command<CreateSchema, controller::Database<NODE_SIZE>>
    for BackendFacade<NODE_SIZE>
{
    type Ok = ();
    type Err = ExecutionError;

    fn execute(
        &self,
        cmd: CreateSchema,
        db_controller: &mut controller::Database<NODE_SIZE>,
    ) -> Result<Self::Ok, Self::Err> {
        let schema = controller::Schema::new(cmd.name);
        db_controller.add_schema(schema);
        Ok(())
    }
}

pub enum ExecutionError {}
