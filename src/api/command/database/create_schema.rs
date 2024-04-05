use derive_more::AsRef;

use crate::{
    api::{command::Execute, facade::BackendFacade},
    controller, schema,
};
use crate::api::command::Command;

#[derive(Debug, AsRef, Clone)]
pub struct CreateSchema {
    pub name: schema::Name,
}

impl Command for CreateSchema {}

impl<const NODE_SIZE: u8> Execute<CreateSchema, controller::Database<NODE_SIZE>>
    for BackendFacade<NODE_SIZE>
{
    type Ok = ();
    type Err = ExecutionError;

    fn execute(
        cmd: CreateSchema,
        db_controller: &mut controller::Database<NODE_SIZE>,
    ) -> Result<Self::Ok, Self::Err> {
        let schema = controller::Schema::new(cmd.name);
        if db_controller.add_schema(schema) {
            Ok(())
        } else {
            Err(ExecutionError::SchemaAlreadyExists)
        }
    }
}

pub enum ExecutionError {
    SchemaAlreadyExists,
}

#[cfg(test)]
mod tests {
    use crate::api::command::database::create_schema::CreateSchema;
    use crate::api::command::Gateway;
    use crate::api::command::gateway::test::backend_facade_factory;

    #[test]
    fn creates_schema_when_not_exists() {
        let mut facade = backend_facade_factory();
        let cmd = CreateSchema {
            name: "schema".into(),
        };
        let result = facade.send(cmd);
        assert!(result.is_ok());
    }
}