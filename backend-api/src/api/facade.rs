use std::sync::{Arc, Mutex};

use backend::{
    controller, data::id, page::page_controller::PageController,
    schema::database,
};
use common::structs::hash_table::scalable::ScalableHashTable;

use crate::context::Context;

#[derive(Debug)]
pub struct BackendFacade<const NODE_SIZE: u8> {
    /// [`PageController`] is used to control pages.
    /// It is used to add new pages and get pages.
    page_controller: Arc<Mutex<PageController>>,

    /// [`id::Registry`] is used to store [`Id`]s and [`PageLink`]s
    /// that are used to access data.
    id_registry: Arc<Mutex<id::Registry>>,

    /// [`Database`] controller.
    pub database_controllers:
        ScalableHashTable<database::Name, controller::Database<NODE_SIZE>>,

    /// [`Context`] is used to store the current database and schema.
    pub context: Context,
}

impl<const NODE_SIZE: u8> Default for BackendFacade<NODE_SIZE> {
    fn default() -> Self {
        BackendFacade {
            page_controller: Arc::new(Mutex::new(PageController::default())),
            id_registry: Arc::new(Mutex::new(id::Registry::default())),
            database_controllers: ScalableHashTable::default(),
            context: Context::default(),
        }
    }
}

impl<const NODE_SIZE: u8> BackendFacade<NODE_SIZE> {
    /// Creates a new [`BackendFacade`] with the given parameters.
    /// # Arguments
    /// * `page_controller` - The [`PageController`] to use.
    /// * `id_registry` - The [`id::Registry`] to use.
    /// # Returns
    /// A new [`BackendFacade`] with the given parameters.
    pub fn new(
        page_controller: Arc<Mutex<PageController>>,
        id_registry: Arc<Mutex<id::Registry>>,
    ) -> Self {
        BackendFacade {
            page_controller,
            id_registry,
            database_controllers: ScalableHashTable::default(),
            context: Context::default(),
        }
    }

    /// Returns the [`id::Registry`] used by the [`BackendFacade`].
    pub fn get_id_registry(&self) -> Arc<Mutex<id::Registry>> {
        self.id_registry.clone()
    }

    /// Returns the [`PageController`] used by the [`BackendFacade`].
    pub fn get_page_controller(&self) -> Arc<Mutex<PageController>> {
        self.page_controller.clone()
    }
}
