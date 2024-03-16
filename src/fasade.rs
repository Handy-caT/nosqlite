use std::sync::{Arc, Mutex};

use crate::data::id;
use crate::page::page_controller::PageController;

pub struct BackendFacade {
    /// [`PageController`] is used to control pages.
    /// It is used to add new pages and get pages.
    page_controller: Arc<Mutex<PageController>>,

    /// [`id::Registry`] is used to store [`Id`]s and [`PageLink`]s
    /// that are used to access data.
    id_registry: Arc<Mutex<id::Registry>>,
    
    
}