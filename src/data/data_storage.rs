use crate::{
    data::{data_allocator::DataAllocator, id, row_type::RowType},
    page::page_controller::PageController,
    schema::r#type::r#enum::{StorageData, StorageDataType},
};

use serde_storage::ser::encoder::{OutputDescriptor, StorageEncoder};
use std::sync::{Arc, Mutex};

/// [`DataStorage`] is a struct that is used to store data of provided format.
/// This object is supposed to be used with [`TableController`] to store
/// table's data.
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

    /// [`RowType`] is used to describe storable data types.
    data_type: RowType,
}

impl DataStorage {
    /// Checks if the provided data type matches the data type of the
    /// [`DataStorage`].
    fn check_data_type(&self, data: &Vec<StorageData>) -> bool {
        if data.len() != self.data_type.len() {
            return false;
        }

        for (i, d) in data.iter().enumerate() {
            if d.data_type() != self.data_type.0[i] {
                return false;
            }
        }

        true
    }

    /// Creates a new [`DataStorage`].
    /// # Returns
    /// * `Self` - [`DataStorage`].
    pub fn new(
        controller: Arc<Mutex<PageController>>,
        registry: Arc<Mutex<id::Registry>>,
    ) -> Self {
        Self {
            page_controller: controller,
            data_allocator: DataAllocator::new(),
            id_registry: registry,
            data_type: RowType::default(),
        }
    }

    /// Adds data to the [`DataStorage`].
    /// # Arguments
    /// * `data` - Data to add.
    /// # Returns
    /// * `NumericId` - [`NumericId`] of the data.
    pub fn add_data<T>(
        &mut self,
        data: Vec<T>,
    ) -> Result<id::NumericId, DataStorageError>
    where
        T: Into<StorageData> + Clone,
    {
        let mut encoder = StorageEncoder::new();

        if !self
            .check_data_type(&data.iter().map(|d| d.clone().into()).collect())
        {
            return Err(DataStorageError::TypeMismatch);
        }

        let _ = data
            .iter()
            .map(|d| encoder.emit::<StorageData>(d.clone().into()))
            .collect::<Vec<_>>();
        let bytes = encoder.output.get_bytes();
        let len = bytes.len();

        let link = self.data_allocator.allocate(len as u16);
        let id = {
            let mut registry = self.id_registry.lock().unwrap();
            registry.add_link(link)
        };

        {
            let mut controller = self.page_controller.lock().unwrap();
            let page = controller.get_page(link.page_index);
            let _ = page.update_data(bytes.as_ref(), link);
        };

        Ok(id)
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
        data: Vec<T>,
    ) -> Result<(), DataStorageError>
    where
        T: Into<StorageData> + Clone,
    {
        self.remove_data(id)?;

        self.add_data(data)?;

        Ok(())
    }

    pub fn get_data(
        &self,
        id: id::NumericId,
    ) -> Result<Vec<StorageData>, DataStorageError> {
        let link = {
            let mut registry = self.id_registry.lock().unwrap();
            registry.get_link(id)
        };

        if let Some(link) = link {
            let mut controller = self.page_controller.lock().unwrap();
            let page = controller.get_page(link.page_index);
            let descriptor: OutputDescriptor = self.data_type.clone().into();

            let data = page.get_by_link(link);

            todo!()
        } else {
            Err(DataStorageError::LinkNotFound)
        }
    }

    /// Gets data type of the [`DataStorage`].
    /// # Returns
    /// * `&Vec<StorageDataType>` - Data type of the [`DataStorage`].
    pub fn get_data_type(&self) -> &Vec<StorageDataType> {
        &self.data_type.0
    }

    /// Sets data type of the [`DataStorage`].
    /// # Arguments
    /// * `data_type` - Data type to set.
    pub fn set_data_type(&mut self, data_type: Vec<StorageDataType>) {
        self.data_type = RowType(data_type);
    }

    /// Appends data type to the [`DataStorage`].
    /// # Arguments
    /// * `data_type` - Data type to append.
    pub fn append_data_type(&mut self, data_type: StorageDataType) {
        self.data_type.0.push(data_type);
    }
}

#[derive(Debug)]
pub enum DataStorageError {
    IdNotFound,
    LinkNotFound,
    PageNotFound,
    DataLengthMismatch,
    TypeMismatch,
}

#[cfg(test)]
mod tests {
    use crate::{
        data::{data_storage::DataStorage, id},
        page::page_controller::PageController,
        schema::r#type::{data_types::Integer, r#enum::StorageDataType},
    };
    use std::sync::{Arc, Mutex};

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
    fn test_data_storage_check_data_type() {
        let mut controller = PageController::new();
        controller.add_page();
        let controller = Arc::new(Mutex::new(controller));
        let registry = Arc::new(Mutex::new(id::Registry::new()));

        let mut data_storage = DataStorage::new(controller, registry);
        data_storage.set_data_type(vec![StorageDataType::Integer]);

        let data = Integer(10);
        let res = data_storage.check_data_type(&vec![data.into()]);

        assert!(res);
    }

    #[test]
    fn test_data_storage_set_data_type() {
        let mut controller = PageController::new();
        controller.add_page();
        let controller = Arc::new(Mutex::new(controller));
        let registry = Arc::new(Mutex::new(id::Registry::new()));

        let mut data_storage = DataStorage::new(controller, registry);
        data_storage.set_data_type(vec![StorageDataType::Integer]);

        assert_eq!(
            data_storage.get_data_type(),
            &vec![StorageDataType::Integer]
        );
    }

    #[test]
    fn test_data_storage_append_data_type() {
        let mut controller = PageController::new();
        controller.add_page();
        let controller = Arc::new(Mutex::new(controller));
        let registry = Arc::new(Mutex::new(id::Registry::new()));

        let mut data_storage = DataStorage::new(controller, registry);
        data_storage.set_data_type(vec![StorageDataType::Integer]);
        data_storage.append_data_type(StorageDataType::UInteger);

        assert_eq!(
            data_storage.get_data_type(),
            &vec![StorageDataType::Integer, StorageDataType::UInteger]
        );
    }

    #[test]
    fn test_data_storage_add_data() {
        let mut controller = PageController::new();
        controller.add_page();
        let controller = Arc::new(Mutex::new(controller));
        let registry = Arc::new(Mutex::new(id::Registry::new()));

        let mut data_storage = DataStorage::new(controller, registry);
        data_storage.set_data_type(vec![StorageDataType::Integer]);

        let data = Integer(10);
        let id = data_storage.add_data(vec![data]);

        assert!(id.is_ok());
        let id = id.unwrap();
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
        data_storage.set_data_type(vec![StorageDataType::Integer]);

        let data = Integer(10);
        let id = data_storage.add_data(vec![data]);
        assert!(id.is_ok());
        let id = id.unwrap();

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
        data_storage.set_data_type(vec![StorageDataType::Integer]);

        let data = Integer(10);
        let id = data_storage.add_data(vec![data]);
        assert!(id.is_ok());
        let id = id.unwrap();

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
        data_storage.set_data_type(vec![StorageDataType::Integer]);

        let data = Integer(10);
        let id = data_storage.add_data(vec![data]);
        assert!(id.is_ok());
        let id = id.unwrap();

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

        let data = Integer(87614);
        let id = data_storage.add_data(vec![data]);

        assert!(id.is_ok());
        let id = id.unwrap();
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
        data_storage.set_data_type(vec![StorageDataType::Integer]);

        let data = Integer(10);
        let id = data_storage.add_data(vec![data]);
        assert!(id.is_ok());
        let id = id.unwrap();

        let updated_data = Integer(87614);
        let res = data_storage.update_data(id, vec![updated_data]);

        {
            let controller = data_storage.page_controller.lock().unwrap();
            assert_eq!(controller.get_page_count(), 1);
        }
        assert_eq!(data_storage.data_allocator.allocated_size(), 4);
        {
            let registry = data_storage.id_registry.lock().unwrap();
            assert_eq!(registry.get_id_count(), 1);
        }
        assert!(res.is_ok());
    }
}
