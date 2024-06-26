//! Defines [`controller::Table`], a controller for a single table.
//!
//! [`controller::Table`]: Table

mod key_id;
mod select;
mod selector;

use std::sync::{Arc, Mutex};

use common::structs::tree::object::tree::Tree as _;

use crate::{
    controller::table::key_id::KeyId,
    data::{data_storage::DataStorage, DataUnit},
    schema,
    schema::{column, column::primary_key::PrimaryKey, table::Name},
};

/// Controller for a single table.
/// Is used to change the table's schema and data.
#[derive(Debug, Clone)]
pub struct Table<const NODE_SIZE: u8> {
    /// Information about the table.
    info: schema::Table,
    /// B-Tree to store primary key indexes.
    //index: BTree<KeyId, NODE_SIZE>,
    index: Vec<KeyId>,
    /// Vector of page indexes that store the table's data.
    table_pages: Vec<usize>,
    /// The data storage to use.
    data_storage: Arc<Mutex<DataStorage>>,
}

impl<const NODE_SIZE: u8> PartialEq for Table<NODE_SIZE> {
    fn eq(&self, other: &Self) -> bool {
        self.info == other.info
    }
}

impl<const NODE_SIZE: u8> Table<NODE_SIZE> {
    /// Creates a new table controller.
    /// # Arguments
    /// * `name` - The name of the table.
    /// # Returns
    /// A new table controller.
    pub fn new(name: Name) -> Self {
        Table {
            info: schema::Table::new(name),
            //index: BTree::default(),
            index: Vec::new(),
            table_pages: Vec::new(),
            data_storage: Arc::new(Mutex::new(DataStorage::default())),
        }
    }

    /// Returns the name of the table.
    /// # Returns
    /// * `&Name` - The name of the table.
    pub fn get_name(&self) -> &Name {
        self.info.get_name()
    }

    /// Adds a column to the table.
    /// # Arguments
    /// * `name` - The name of the column.
    /// * `column` - The column to add.
    pub fn add_column(&mut self, name: column::Name, column: schema::Column) {
        {
            let mut data_storage = self.data_storage.lock().unwrap();
            data_storage.append_data_type(column.get_type());
        }
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
    pub fn get_column(
        &mut self,
        name: &column::Name,
    ) -> Option<schema::Column> {
        self.info.get_column(name)
    }

    /// Adds a [`DataUnit`] to the table.
    /// # Arguments
    /// * `data` - The data to add.
    /// # Returns
    /// * `Result<(), TableControllerError>` - The result of the operation.
    pub fn add_data(
        &mut self,
        mut data: DataUnit,
    ) -> Result<(), TableControllerError> {
        let Some(primary_key) = self.get_primary_key() else {
            return Err(TableControllerError::PrimaryKeyDoesNotExist);
        };

        let key_name = primary_key.get_column();
        let key_index = if let Some(index) = data.get_index(key_name) {
            index
        } else {
            return Err(TableControllerError::ColumnNotProvided);
        };

        for row in data.get_values() {
            let key = row.0.get(key_index).expect("exists").clone();

            let id = {
                let mut data_storage = self.data_storage.lock().unwrap();
                let Ok(id) = data_storage.add_data(row) else {
                    return Err(TableControllerError::DataStorageError);
                };
                id
            };

            let key_id = KeyId {
                id,
                key: key.try_into().unwrap(),
            };

            self.index.push(key_id);
        }

        Ok(())
    }

    /// Adds a page to the table.
    /// # Arguments
    /// * `index` - The index of the page to add.
    pub fn add_page(&mut self, index: usize) {
        self.table_pages.push(index);
    }

    /// Returns the pages of the table.
    /// # Returns
    /// * `&Vec<usize>` - The pages of the table.
    pub fn get_pages(&self) -> &Vec<usize> {
        &self.table_pages
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
    use common::structs::tree::object::tree::Tree as _;

    use crate::{
        controller::{
            table,
            table::{KeyId, Table, TableControllerError},
        },
        data::{data_storage::DataStorage, id, id::NumericId, DataUnit},
        page::page_controller::PageController,
        schema,
        schema::{
            column::primary_key,
            r#type::r#enum::{StorageData, StorageDataType},
        },
    };
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_new() {
        let name: table::Name = "table".into();
        let table = Table::<16>::new(name.clone());

        assert_eq!(table.get_name(), &name);
    }

    #[test]
    fn test_add_data() {
        let name: table::Name = "table".into();
        let mut table = Table::<16>::new(name.clone());
        table.add_column(
            "id".into(),
            schema::Column::new(StorageDataType::Integer),
        );

        let primary_key =
            primary_key::PrimaryKey::new("pk".into(), "id".into());
        table
            .set_primary_key(primary_key.clone())
            .expect("Failed to set primary key");

        let mut data = DataUnit::new(vec!["id".into()]);
        data.insert(vec![StorageData::Integer(0.into())].into());

        let res = table.add_data(data);

        assert!(res.is_ok());
        let index = &table.index;
        assert_eq!(index.len(), 1);
    }

    #[test]
    fn test_add_data_multiple() {
        let name: table::Name = "table".into();
        let mut table = Table::<16>::new(name.clone());
        table.add_column(
            "id".into(),
            schema::Column::new(StorageDataType::Integer),
        );

        let primary_key =
            primary_key::PrimaryKey::new("pk".into(), "id".into());
        table
            .set_primary_key(primary_key.clone())
            .expect("Failed to set primary key");

        let mut data = DataUnit::new(vec!["id".into()]);
        data.insert(vec![StorageData::Integer(0.into())].into());
        data.insert(vec![StorageData::Integer(1.into())].into());
        data.insert(vec![StorageData::Integer(2.into())].into());

        let res = table.add_data(data);

        assert!(res.is_ok());

        let index = &table.index;
        assert_eq!(index.len(), 3);
    }

    #[test]
    fn test_set_primary_key_without_column() {
        let name: table::Name = "table".into();
        let mut table = Table::<16>::new(name.clone());

        let primary_key =
            primary_key::PrimaryKey::new("pk".into(), "id".into());
        let res = table.set_primary_key(primary_key.clone());

        assert!(res.is_err());
        assert_eq!(res.err(), Some(TableControllerError::ColumnDoesNotExist));
    }

    #[test]
    fn test_set_primary_key() {
        let name: table::Name = "table".into();
        let mut table = Table::<16>::new(name.clone());
        table.add_column(
            "id".into(),
            schema::Column::new(StorageDataType::Integer),
        );

        let primary_key =
            primary_key::PrimaryKey::new("pk".into(), "id".into());

        let res = table.set_primary_key(primary_key.clone());
        assert!(res.is_ok());
        assert_eq!(table.get_primary_key(), &Some(primary_key));
    }

    #[test]
    fn test_set_primary_key_wrong_type() {
        let name: table::Name = "table".into();
        let mut table = Table::<16>::new(name.clone());
        table.add_column(
            "id".into(),
            schema::Column::new(StorageDataType::VarChar(40)),
        );

        let primary_key =
            primary_key::PrimaryKey::new("pk".into(), "id".into());

        let res = table.set_primary_key(primary_key.clone());
        assert!(res.is_err());
        assert_eq!(
            res.err(),
            Some(TableControllerError::WrongTypeForPrimaryKey)
        );
    }

    #[test]
    fn test_get_name() {
        let name: table::Name = "table".into();
        let table = Table::<16>::new(name.clone());

        assert_eq!(table.get_name(), &name);
    }

    #[test]
    fn test_get_column() {
        let name: table::Name = "table".into();
        let mut table = Table::<16>::new(name.clone());

        let column = schema::Column::new(StorageDataType::Integer);
        table.add_column("column".into(), column.clone());

        assert_eq!(table.get_column(&"column".into()), Some(column));
    }

    #[test]
    fn test_add_column() {
        let name: table::Name = "table".into();
        let mut table = Table::<16>::new(name.clone());

        let column = schema::Column::new(StorageDataType::Integer);
        table.add_column("column".into(), column.clone());

        assert_eq!(table.get_column(&"column".into()), Some(column));

        {
            let data_storage = table.data_storage.lock().unwrap();
            assert_eq!(
                data_storage.get_data_type(),
                &vec![StorageDataType::Integer]
            );
        }
    }

    #[test]
    fn test_add_column_multiple() {
        let name: table::Name = "table".into();
        let mut table = Table::<16>::new(name.clone());

        let column = schema::Column::new(StorageDataType::Integer);
        table.add_column("column".into(), column.clone());

        let column = schema::Column::new(StorageDataType::Integer);
        table.add_column("column2".into(), column.clone());

        assert_eq!(
            table.get_column(&"column".into()),
            Some(schema::Column::new(StorageDataType::Integer))
        );
        assert_eq!(
            table.get_column(&"column2".into()),
            Some(schema::Column::new(StorageDataType::Integer))
        );
    }

    #[test]
    fn test_add_page() {
        let name: table::Name = "table".into();
        let mut table = Table::<16>::new(name.clone());

        table.add_page(0);

        assert_eq!(table.table_pages.len(), 1);
    }
}
