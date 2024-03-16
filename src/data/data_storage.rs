use std::sync::{Arc, Mutex};
use crate::{
    data::{data_allocator::DataAllocator, id},
    page::page_controller::PageController,
};

/// [`DataStorage`] is a struct that is used to store various data.
#[derive(Debug)]
pub struct DataStorage {
    /// [`PageController`] is used to control pages.
    /// It is used to add new pages and get pages.
    page_controller: Arc<Mutex<PageController>>,

    /// [`id::Registry`] is used to store [`Id`]s and [`PageLink`]s
    /// that are used to access data.
    id_registry: Arc<Mutex<id::Registry>>,

    /// [`DataAllocator`] is used to find free space in pages.
    data_allocator: DataAllocator,
}

impl DataStorage {
    /// Creates a new [`DataStorage`].
    /// # Returns
    /// * `Self` - [`DataStorage`].
    pub fn new(controller: Arc<Mutex<PageController>>, registry: Arc<Mutex<id::Registry>>) -> Self {
        Self {
            page_controller: controller,
            data_allocator: DataAllocator::new(),
            id_registry: registry,
        }
    }

    /// Adds data to the [`DataStorage`].
    /// # Arguments
    /// * `data` - Data to add.
    /// # Returns
    /// * `NumericId` - [`NumericId`] of the data.
    pub fn add_data<T>(&mut self, data: &T) -> id::NumericId
    where
        T: Sized,
    {
        let slice = unsafe {
            std::slice::from_raw_parts(
                data as *const T as *const u8,
                std::mem::size_of_val(data),
            )
        };
        let len = slice.len();

        let link = self.data_allocator.allocate(len as u16);
        let id = {
            let mut registry = self.id_registry.lock().unwrap();
            registry.add_link(link)
        };

        {
            let mut controller = self.page_controller.lock().unwrap();
            let page = controller.get_page(link.page_index);
            let _ = page.update_data(slice, link);
        };

        id
    }

    /// Removes data from the [`DataStorage`].
    /// # Arguments
    /// * `id` - [`NumericId`] of the data.
    /// # Returns
    /// * `Result<(), DataStorageError>` - Result of the operation.
    /// # Errors
    /// * `DataStorageError::IdNotFound` - [`NumericId`] was not found.
    pub fn remove_data(
        &mut self,
        id: id::NumericId,
    ) -> Result<(), DataStorageError> {
        let link = {
            let mut registry = self.id_registry.lock().unwrap();
            registry.get_link(id)
        };
        
        if let Some(link) = link {
            self.data_allocator.remove(link);
            let res = {
                let mut registry = self.id_registry.lock().unwrap();
                registry.remove_id(id)
            };
            if res.is_err() {
                return Err(DataStorageError::IdNotFound);
            }

            Ok(())
        } else {
            Err(DataStorageError::LinkNotFound)
        }
    }

    /// Updates data in the [`DataStorage`].
    /// # Arguments
    /// * `id` - [`NumericId`] of the data.
    /// * `data` - Data to update.
    /// # Returns
    /// * `Result<(), DataStorageError>` - Result of the operation.
    /// # Errors
    /// * `DataStorageError::IdNotFound` - [`NumericId`] was not found.
    pub fn update_data<T>(
        &mut self,
        id: id::NumericId,
        data: &T,
    ) -> Result<(), DataStorageError>
    where
        T: Sized,
    {
        self.remove_data(id)?;

        self.add_data(data);

        Ok(())
    }
}

pub enum DataStorageError {
    IdNotFound,
    LinkNotFound,
    PageNotFound,
    DataLengthMismatch,
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use crate::data::data_storage::DataStorage;
    use crate::data::id;
    use crate::page::page_controller::PageController;

    #[test]
    fn test_data_storage_new() {
        let mut controller = PageController::new();
        controller.add_page();
        let controller = Arc::new(Mutex::new(controller));
        let registry = Arc::new(Mutex::new(id::Registry::new()));
        
        let data_storage = DataStorage::new(controller, registry);

        {
            let controller = data_storage.page_controller.lock().unwrap();
            assert_eq!(controller.get_page_count(), 1);
        }
        assert_eq!(data_storage.data_allocator.allocated_size(), 0);
        {
            let registry = data_storage.id_registry.lock().unwrap();
            assert_eq!(registry.get_id_count(), 0);
        }
    }

    #[test]
    fn test_data_storage_add_data() {
        let mut controller = PageController::new();
        controller.add_page();
        let controller = Arc::new(Mutex::new(controller));
        let registry = Arc::new(Mutex::new(id::Registry::new()));

        let mut data_storage = DataStorage::new(controller, registry);

        let data = 10u32;
        let id = data_storage.add_data(&data);

        {
            let controller = data_storage.page_controller.lock().unwrap();
            assert_eq!(controller.get_page_count(), 1);
        }
        assert_eq!(data_storage.data_allocator.allocated_size(), 4);
        {
            let registry = data_storage.id_registry.lock().unwrap();
            assert_eq!(registry.get_id_count(), 1);
        }
        assert_eq!(id.0, 1);
    }

    #[test]
    fn test_data_storage_remove_data() {
        let mut controller = PageController::new();
        controller.add_page();
        let controller = Arc::new(Mutex::new(controller));
        let registry = Arc::new(Mutex::new(id::Registry::new()));

        let mut data_storage = DataStorage::new(controller, registry);

        let data = 10u32;
        let id = data_storage.add_data(&data);
        let res = data_storage.remove_data(id);

        {
            let controller = data_storage.page_controller.lock().unwrap();
            assert_eq!(controller.get_page_count(), 1);
        }
        assert_eq!(data_storage.data_allocator.allocated_size(), 4);
        {
            let registry = data_storage.id_registry.lock().unwrap();
            assert_eq!(registry.get_id_count(), 0);
        }
        assert!(res.is_ok());
    }

    #[test]
    fn test_data_storage_remove_data_not_found() {
        let mut controller = PageController::new();
        controller.add_page();
        let controller = Arc::new(Mutex::new(controller));
        let registry = Arc::new(Mutex::new(id::Registry::new()));

        let mut data_storage = DataStorage::new(controller, registry);

        let data = 10u32;
        let id = data_storage.add_data(&data);
        let res = data_storage.remove_data(id);
        let res = data_storage.remove_data(id);

        {
            let controller = data_storage.page_controller.lock().unwrap();
            assert_eq!(controller.get_page_count(), 1);
        }
        assert_eq!(data_storage.data_allocator.allocated_size(), 4);
        {
            let registry = data_storage.id_registry.lock().unwrap();
            assert_eq!(registry.get_id_count(), 0);
        }
        assert!(res.is_err());
    }

    #[test]
    fn test_data_storage_add_data_after_remove() {
        let mut controller = PageController::new();
        controller.add_page();
        let controller = Arc::new(Mutex::new(controller));
        let registry = Arc::new(Mutex::new(id::Registry::new()));

        let mut data_storage = DataStorage::new(controller, registry);

        let data = 10u32;
        let id = data_storage.add_data(&data);
        let res = data_storage.remove_data(id);

        {
            let controller = data_storage.page_controller.lock().unwrap();
            assert_eq!(controller.get_page_count(), 1);
        }
        assert_eq!(data_storage.data_allocator.allocated_size(), 4);
        {
            let registry = data_storage.id_registry.lock().unwrap();
            assert_eq!(registry.get_id_count(), 0);
        }
        assert!(res.is_ok());

        let data = 87614u32;
        let id = data_storage.add_data(&data);

        {
            let controller = data_storage.page_controller.lock().unwrap();
            assert_eq!(controller.get_page_count(), 1);
        }
        assert_eq!(data_storage.data_allocator.allocated_size(), 4);
        {
            let registry = data_storage.id_registry.lock().unwrap();
            assert_eq!(registry.get_id_count(), 1);
        }
        assert_eq!(id.0, 1);
    }

    #[test]
    fn test_data_storage_update_data() {
        let mut controller = PageController::new();
        controller.add_page();
        let controller = Arc::new(Mutex::new(controller));
        let registry = Arc::new(Mutex::new(id::Registry::new()));

        let mut data_storage = DataStorage::new(controller, registry);

        let data = 10u32;
        let id = data_storage.add_data(&data);

        let updated_data = 87614u64;
        let res = data_storage.update_data(id, &updated_data);

        {
            let controller = data_storage.page_controller.lock().unwrap();
            assert_eq!(controller.get_page_count(), 1);
        }
        assert_eq!(data_storage.data_allocator.allocated_size(), 12);
        {
            let registry = data_storage.id_registry.lock().unwrap();
            assert_eq!(registry.get_id_count(), 1);
        }
        assert!(res.is_ok());
    }
}
