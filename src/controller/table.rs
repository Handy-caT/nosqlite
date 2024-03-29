use crate::{
    core::structs::tree::object::{tree::Tree as _, BTree},
    data::{data_storage::DataStorage, id::NumericId, DataUnit},
    schema,
    schema::{
        column::{primary_key, primary_key::PrimaryKey},
        r#type::r#enum::StorageData,
    },
};

/// Represents a mapper from a primary key to a unique identifier.
#[derive(Debug, Clone)]
pub struct KeyId {
    /// The unique identifier.
    pub id: NumericId,

    /// The primary key value.
    pub key: primary_key::Data,
}

impl PartialEq for KeyId {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for KeyId {}

impl PartialOrd for KeyId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for KeyId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

/// Controller for a single table.
/// Is used to change the table's schema and data.
#[derive(Debug)]
pub struct Table<const NODE_SIZE: u8> {
    /// Information about the table.
    info: schema::Table,

    /// B-Tree to store primary key indexes.
    index: BTree<KeyId, NODE_SIZE>,

    /// Vector of page indexes that store the table's data.
    table_pages: Vec<usize>,

    data_storage: DataStorage,
}

impl<const NODE_SIZE: u8> Table<NODE_SIZE> {
    /// Creates a new table controller.
    /// # Arguments
    /// * `name` - The name of the table.
    /// * `data_storage` - The data storage to use.
    /// # Returns
    /// A new table controller.
    pub fn new(name: String, data_storage: DataStorage) -> Self {
        Table {
            info: schema::Table::new(name),
            index: BTree::default(),
            table_pages: Vec::new(),
            data_storage,
        }
    }

    /// Returns the name of the table.
    /// # Returns
    /// * `&String` - The name of the table.
    pub fn get_name(&self) -> &String {
        self.info.get_name()
    }

    /// Adds a column to the table.
    /// # Arguments
    /// * `name` - The name of the column.
    /// * `column` - The column to add.
    pub fn add_column(&mut self, name: String, column: schema::Column) {
        self.data_storage.append_data_type(column.get_type());
        self.info.add_column(name, column);

        //todo!("Add data update when new column added")
    }

    /// Sets the primary key of the table.
    /// # Arguments
    /// * `primary_key` - The primary key to set.
    pub fn set_primary_key(
        &mut self,
        primary_key: PrimaryKey,
    ) -> Result<(), TableControllerError> {
        if let Some(column) = self.info.get_column(primary_key.get_column()) {
            if !PrimaryKey::check_type(column.clone()) {
                return Err(TableControllerError::WrongTypeForPrimaryKey);
            }
            self.info.set_primary_key(primary_key);
            Ok(())
        } else {
            Err(TableControllerError::ColumnDoesNotExist)
        }
    }

    /// Returns the primary key of the table.
    /// # Returns
    /// * `&PrimaryKey` - The primary key of the table.
    pub fn get_primary_key(&self) -> &Option<PrimaryKey> {
        self.info.get_primary_key()
    }

    /// Returns the column with the given name.
    /// # Arguments
    /// * `name` - The name of the column.
    /// # Returns
    /// * `Option<&Column>` - The column with the given name.
    pub fn get_column(&mut self, name: &String) -> Option<schema::Column> {
        self.info.get_column(name)
    }

    /// Adds a row identified by [`NumericId`] to the table.
    /// # Arguments
    /// * `data` - The data to add.
    /// # Returns
    /// * `NumericId` - The index of the new row.
    pub fn add_data(&mut self, mut data: DataUnit) -> Result<(), TableControllerError> {
        let Some(primary_key) = self.get_primary_key() else {
            return Err(TableControllerError::PrimaryKeyDoesNotExist);
        };
        
        let key = primary_key.get_column();
        let Some(key) = data.get(key) else {
            return Err(TableControllerError::ColumnNotProvided);
        };

        let values = data.get_values();
        let Ok(id) = self.data_storage.add_data(values) else {
            return Err(TableControllerError::DataStorageError);
        };
        
        let key_id = KeyId {
            id,
            key: key.try_into().unwrap(),
        };
        
        self.index.push(key_id);

        Ok(())
    }

    /// Adds a page to the table.
    /// # Arguments
    /// * `index` - The index of the page to add.
    pub fn add_page(&mut self, index: usize) {
        self.table_pages.push(index);
    }
}

#[derive(Debug, PartialEq)]
pub enum TableControllerError {
    ColumnNotProvided,
    ColumnDoesNotExist,
    WrongTypeForPrimaryKey,
    PrimaryKeyDoesNotExist,
    DataStorageError,
}

#[cfg(test)]
mod tests {
    use crate::{
        controller::table::{KeyId, Table, TableControllerError},
        core::structs::tree::object::tree::Tree as _,
        data::{data_storage::DataStorage, id, id::NumericId},
        page::page_controller::PageController,
        schema,
        schema::{column::primary_key, r#type::r#enum::StorageDataType},
    };
    use std::sync::{Arc, Mutex};
    use crate::data::DataUnit;
    use crate::schema::r#type::r#enum::StorageData;

    /// Creates a new instance of `DataStorage`.
    fn data_storage_factory() -> DataStorage {
        let mut controller = PageController::new();
        controller.add_page();
        let controller = Arc::new(Mutex::new(controller));
        let registry = Arc::new(Mutex::new(id::Registry::new()));

        DataStorage::new(controller, registry)
    }

    #[test]
    fn test_new() {
        let name = "table".to_string();
        let data_storage = data_storage_factory();
        let table = Table::<16>::new(name.clone(), data_storage);

        assert_eq!(table.get_name(), &name);
    }

    #[test]
    fn test_add_data() {
        let name = "table".to_string();
        let data_storage = data_storage_factory();
        let mut table = Table::<16>::new(name.clone(), data_storage);
        table.add_column(
            "id".to_string(),
            schema::Column::new(StorageDataType::Integer),
        );

        let primary_key = primary_key::PrimaryKey::new(
            "pk".to_string(),
            "id".to_string(),
        );
        table.set_primary_key(primary_key.clone()).expect("Failed to set primary key");
    
        let mut data = DataUnit::new(1);
        data.insert("id".to_string(), StorageData::Integer(0.into()));
        
        let res = table.add_data(data);
    
        assert!(res.is_ok());
    }

    #[test]
    fn test_set_primary_key_without_column() {
        let name = "table".to_string();
        let data_storage = data_storage_factory();
        let mut table = Table::<16>::new(name.clone(), data_storage);

        let primary_key = primary_key::PrimaryKey::new(
            "pk".to_string(),
            "id".to_string(),
        );
        let res = table.set_primary_key(primary_key.clone());

        assert!(res.is_err());
        assert_eq!(res.err(), Some(TableControllerError::ColumnDoesNotExist));
    }

    #[test]
    fn test_set_primary_key() {
        let name = "table".to_string();
        let data_storage = data_storage_factory();
        let mut table = Table::<16>::new(name.clone(), data_storage);
        table.add_column(
            "id".to_string(),
            schema::Column::new(StorageDataType::Integer),
        );

        let primary_key = primary_key::PrimaryKey::new(
            "pk".to_string(),
            "id".to_string(),
        );

        let res = table.set_primary_key(primary_key.clone());
        assert!(res.is_ok());
        assert_eq!(table.get_primary_key(), &Some(primary_key));
    }

    #[test]
    fn test_set_primary_key_wrong_type() {
        let name = "table".to_string();
        let data_storage = data_storage_factory();
        let mut table = Table::<16>::new(name.clone(), data_storage);
        table.add_column(
            "id".to_string(),
            schema::Column::new(StorageDataType::VarChar(40)),
        );

        let primary_key = primary_key::PrimaryKey::new(
            "pk".to_string(),
            "id".to_string(),
        );

        let res = table.set_primary_key(primary_key.clone());
        assert!(res.is_err());
        assert_eq!(
            res.err(),
            Some(TableControllerError::WrongTypeForPrimaryKey)
        );
    }

    #[test]
    fn test_get_name() {
        let name = "table".to_string();
        let data_storage = data_storage_factory();
        let table = Table::<16>::new(name.clone(), data_storage);

        assert_eq!(table.get_name(), &name);
    }

    #[test]
    fn test_get_column() {
        let name = "table".to_string();
        let data_storage = data_storage_factory();
        let mut table = Table::<16>::new(name.clone(), data_storage);

        let column = schema::Column::new(StorageDataType::Integer);
        table.add_column("column".to_string(), column.clone());

        assert_eq!(table.get_column(&"column".to_string()), Some(column));
    }

    #[test]
    fn test_add_column() {
        let name = "table".to_string();
        let data_storage = data_storage_factory();
        let mut table = Table::<16>::new(name.clone(), data_storage);

        let column = schema::Column::new(StorageDataType::Integer);
        table.add_column("column".to_string(), column.clone());

        assert_eq!(table.get_column(&"column".to_string()), Some(column));
        assert_eq!(table.data_storage.get_data_type(), &vec![StorageDataType::Integer]);
    }

    #[test]
    fn test_add_column_multiple() {
        let name = "table".to_string();
        let data_storage = data_storage_factory();
        let mut table = Table::<16>::new(name.clone(), data_storage);

        let column = schema::Column::new(StorageDataType::Integer);
        table.add_column("column".to_string(), column.clone());

        let column = schema::Column::new(StorageDataType::Integer);
        table.add_column("column2".to_string(), column.clone());

        assert_eq!(
            table.get_column(&"column".to_string()),
            Some(schema::Column::new(StorageDataType::Integer))
        );
        assert_eq!(
            table.get_column(&"column2".to_string()),
            Some(schema::Column::new(StorageDataType::Integer))
        );
    }

    #[test]
    fn test_add_page() {
        let name = "table".to_string();
        let data_storage = data_storage_factory();
        let mut table = Table::<16>::new(name.clone(), data_storage);

        table.add_page(0);

        assert_eq!(table.table_pages.len(), 1);
    }
}
