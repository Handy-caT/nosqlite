use crate::{
    controller::{
        table::{selector::DataSelector, TableControllerError},
        Table,
    },
    data::DataUnit,
    schema::column,
};

impl<const NODE_SIZE: u8> Table<NODE_SIZE> {
    fn select_without_filters(
        &mut self,
        rows: Option<Vec<column::Name>>,
    ) -> Result<DataUnit, TableControllerError> {
        if rows.is_some() {
            todo!()
        } else {
            let mut data_unit = DataUnit::new(self.info.get_column_names());
            for id in self.index.iter().map(|v| v.id) {
                let row = { self.data_storage.lock().unwrap().get_data(id) }
                    .map_err(|_| TableControllerError::DataStorageError)?;

                data_unit.insert(row);
            }

            Ok(data_unit)
        }
    }

    /// Gets rows by [`DataSelector`] from the table.
    /// # Arguments
    /// * `selector` - The selector to use.
    /// # Returns
    /// * `Result<DataUnit, TableControllerError>` - The result of the
    ///   operation.
    pub fn get_data(
        &mut self,
        selector: DataSelector,
    ) -> Result<DataUnit, TableControllerError> {
        if selector.filters.is_none() {
            return self.select_without_filters(selector.row_names);
        }

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        controller::{table, table::selector::DataSelector},
        data::DataUnit,
        schema,
        schema::{
            column::primary_key,
            r#type::{
                r#enum::{StorageData, StorageDataType},
                DataRow,
            },
        },
    };

    #[test]
    fn test_table_get_data() {
        let name: table::Name = "table".into();
        let mut table = crate::controller::Table::<16>::new(name.clone());
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

        table.add_data(data).expect("is ok");

        let selector = DataSelector {
            row_names: None,
            filters: None,
        };

        let result = table.get_data(selector);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.len(), 3);
        let values = result.get_values();

        assert_eq!(
            values.first().unwrap(),
            &DataRow::from(vec![StorageData::Integer(0.into())])
        );
        assert_eq!(
            values.get(1).unwrap(),
            &DataRow::from(vec![StorageData::Integer(1.into())])
        );
        assert_eq!(
            values.last().unwrap(),
            &DataRow::from(vec![StorageData::Integer(2.into())])
        );
    }
}
