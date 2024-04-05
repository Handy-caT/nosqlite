use crate::{
    api::{
        command::{Command, Execute, ExecuteDatabase, Gateway, GatewayError},
        facade::BackendFacade,
    },
    controller,
    schema::database,
};
use common::structs::hash_table::MutHashTable;

impl<Cmd, const NODE_SIZE: u8> Gateway<Cmd, controller::Database<NODE_SIZE>>
    for BackendFacade<NODE_SIZE>
where
    Self: ExecuteDatabase<Cmd, NODE_SIZE>,
    Cmd: Command + AsRef<database::Name>,
{
    type Ok = <Self as Execute<Cmd, controller::Database<NODE_SIZE>>>::Ok;
    type Err = GatewayError<
        <Self as Execute<Cmd, controller::Database<NODE_SIZE>>>::Err,
        DatabaseGatewayError,
    >;

    fn send(
        &mut self,
        cmd: Cmd,
    ) -> Result<
        <Self as Gateway<Cmd, controller::Database<NODE_SIZE>>>::Ok,
        <Self as Gateway<Cmd, controller::Database<NODE_SIZE>>>::Err,
    > {
        let database_name = cmd.as_ref();
        let database =
            self.database_controllers.get_mut_value(database_name);
        if let Some(database) = database {
            <Self as Execute<Cmd, controller::Database<NODE_SIZE>>>::execute(
                cmd, database,
            )
            .map_err(GatewayError::Cmd)
        } else {
            Err(GatewayError::Gateway(
                DatabaseGatewayError::DatabaseNotFound,
            ))
        }
    }
}

pub enum DatabaseGatewayError {
    DatabaseNotFound,
}

#[cfg(test)]
pub mod test {
    use std::sync::{Arc, Mutex};
    
    use crate::api::facade::BackendFacade;
    use crate::data::id;
    use crate::page::page_controller::PageController;

    pub fn backend_facade_factory() -> BackendFacade<4> {
        let id_registry = Arc::new(Mutex::new(id::Registry::new()));
        let page_controller = Arc::new(Mutex::new(PageController::new()));
        
        BackendFacade::new(page_controller, id_registry)
    }
}