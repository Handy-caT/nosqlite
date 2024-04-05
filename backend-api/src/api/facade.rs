use common::structs::hash_table::scalable::ScalableHashTable;
use std::sync::{Arc, Mutex};

use backend::{
    controller, data::id, page::page_controller::PageController,
    schema::database,
};

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
