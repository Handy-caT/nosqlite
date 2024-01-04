use crate::{
    data::{data_allocator::DataAllocator, id},
    page::page_controller::PageController,
};

/// [`DataStorage`] is a struct that is used to store various data.
pub struct DataStorage {
    /// [`PageController`] is used to control pages.
    /// It is used to add new pages and get pages.
    page_controller: PageController,

    /// [`DataAllocator`] is used to find free space in pages.
    data_allocator: DataAllocator,

    /// [`id::Registry`] is used to store [`Id`]s and [`PageLink`]s
    /// that are used to access data.
    id_registry: id::Registry,
}

impl DataStorage {
    /// Creates a new [`DataStorage`].
    /// # Returns
    /// * `Self` - [`DataStorage`].
    pub fn new() -> Self {
        let mut page_controller = PageController::new();
        page_controller.add_page();

        Self {
            page_controller,
            data_allocator: DataAllocator::new(),
            id_registry: id::Registry::new(),
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
        let id = self.id_registry.add_link(link);

        let page = self.page_controller.get_page(link.page_index);
        let _ = page.update_data(slice, link);

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
        let link = self.id_registry.get_link(id);
        if let Some(link) = link {
            self.data_allocator.remove(link);
            let res = self.id_registry.remove_id(id);
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
    use crate::data::data_storage::DataStorage;

    #[test]
    fn test_data_storage_new() {
        let data_storage = DataStorage::new();

        assert_eq!(data_storage.page_controller.get_page_count(), 1);
        assert_eq!(data_storage.data_allocator.allocated_size(), 0);
        assert_eq!(data_storage.id_registry.get_id_count(), 0);
    }

    #[test]
    fn test_data_storage_add_data() {
        let mut data_storage = DataStorage::new();

        let data = 10u32;
        let id = data_storage.add_data(&data);

        assert_eq!(data_storage.page_controller.get_page_count(), 1);
        assert_eq!(data_storage.data_allocator.allocated_size(), 4);
        assert_eq!(data_storage.id_registry.get_id_count(), 1);
        assert_eq!(id.0, 1);
    }

    #[test]
    fn test_data_storage_remove_data() {
        let mut data_storage = DataStorage::new();

        let data = 10u32;
        let id = data_storage.add_data(&data);
        let res = data_storage.remove_data(id);

        assert_eq!(data_storage.page_controller.get_page_count(), 1);
        assert_eq!(data_storage.data_allocator.allocated_size(), 4);
        assert_eq!(data_storage.id_registry.get_id_count(), 0);
        assert!(res.is_ok());
    }

    #[test]
    fn test_data_storage_remove_data_not_found() {
        let mut data_storage = DataStorage::new();

        let data = 10u32;
        let id = data_storage.add_data(&data);
        let res = data_storage.remove_data(id);
        let res = data_storage.remove_data(id);

        assert_eq!(data_storage.page_controller.get_page_count(), 1);
        assert_eq!(data_storage.data_allocator.allocated_size(), 4);
        assert_eq!(data_storage.id_registry.get_id_count(), 0);
        assert!(res.is_err());
    }

    #[test]
    fn test_data_storage_add_data_after_remove() {
        let mut data_storage = DataStorage::new();

        let data = 10u32;
        let id = data_storage.add_data(&data);
        let res = data_storage.remove_data(id);

        assert_eq!(data_storage.page_controller.get_page_count(), 1);
        assert_eq!(data_storage.data_allocator.allocated_size(), 4);
        assert_eq!(data_storage.id_registry.get_id_count(), 0);
        assert!(res.is_ok());

        let data = 87614u32;
        let id = data_storage.add_data(&data);

        assert_eq!(data_storage.page_controller.get_page_count(), 1);
        assert_eq!(data_storage.data_allocator.allocated_size(), 4);
        assert_eq!(data_storage.id_registry.get_id_count(), 1);
        assert_eq!(id.0, 1);
    }

    #[test]
    fn test_data_storage_update_data() {
        let mut data_storage = DataStorage::new();

        let data = 10u32;
        let id = data_storage.add_data(&data);

        let updated_data = 87614u64;
        let res = data_storage.update_data(id, &updated_data);

        assert_eq!(data_storage.page_controller.get_page_count(), 1);
        assert_eq!(data_storage.data_allocator.allocated_size(), 12);
        assert_eq!(data_storage.id_registry.get_id_count(), 1);
        assert!(res.is_ok());
    }
}
