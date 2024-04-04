use crate::controller;
use std::sync::{Arc, Mutex};

use crate::{data::id, page::page_controller::PageController};

#[derive(Debug)]
pub struct BackendFacade<const NODE_SIZE: u8> {
    /// [`PageController`] is used to control pages.
    /// It is used to add new pages and get pages.
    page_controller: Arc<Mutex<PageController>>,

    /// [`id::Registry`] is used to store [`Id`]s and [`PageLink`]s
    /// that are used to access data.
    id_registry: Arc<Mutex<id::Registry>>,

    /// [`Database`] controller.
    database_controller: controller::Database<NODE_SIZE>,
}
