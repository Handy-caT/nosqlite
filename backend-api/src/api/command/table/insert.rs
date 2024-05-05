use backend::{
    controller,
    controller::table::TableControllerError,
    data::DataUnit,
    schema,
    schema::{
        column, column::primary_key::PrimaryKey, database, table, Column,
    },
};
use derive_more::Display;

use crate::{
    api::{
        command::{
            schema::ProvideError, Command, ContextReceiver, DatabaseCommand,
            OptionalBy, SchemaCommand, TableCommand,
        },
        CommandResultString,
    },
    Context,
};

/// [`Command`] to insert data to a table in a database.
#[derive(Debug, PartialEq)]
pub struct Insert {
    /// The name of the database where the table will be created.
    pub database_name: Option<database::Name>,

    /// The name of the schema where the table will be created.
    pub schema_name: Option<schema::Name>,

    /// The name of the table to create.
    pub name: table::Name,

    /// The data to insert.
    pub data: DataUnit,
}

impl DatabaseCommand for Insert {
    fn get_db_name(&self) -> Option<database::Name> {
        self.database_name.clone()
    }

    fn get_db_name_mut(&mut self) -> &mut Option<database::Name> {
        &mut self.database_name
    }
}

impl SchemaCommand for Insert {
    fn get_schema_name(&self) -> Option<schema::Name> {
        self.schema_name.clone()
    }

    fn get_schema_name_mut(&mut self) -> &mut Option<schema::Name> {
        &mut self.schema_name
    }
}

impl TableCommand for Insert {
    fn get_table_name(&self) -> table::Name {
        self.name.clone()
    }

    fn get_table_name_mut(&mut self) -> &mut table::Name {
        &mut self.name
    }
}

impl<const NODE_SIZE: u8> Command<controller::Table<NODE_SIZE>> for Insert {
    type Ok = CommandResultString;
    type Err = ExecutionError;

    fn execute(
        self,
        table_controller: &mut controller::Table<NODE_SIZE>,
    ) -> Result<Self::Ok, Self::Err> {
        let rows_affected = self.data.len();

        table_controller
            .add_data(self.data)
            .map_err(ExecutionError::TableControllerError)?;

        Ok(CommandResultString {
            result: format!("{} rows affected", rows_affected),
        })
    }
}

/// Errors that can occur during the execution of [`Insert`].
#[derive(Debug, Display)]
pub enum ExecutionError {
    /// The table controller error.
    #[display(fmt = "{:?}", _0)]
    TableControllerError(TableControllerError),
}

#[cfg(test)]
mod tests {
    use backend::{
        data::DataUnit,
        schema,
        schema::{
            column,
            column::primary_key,
            database,
            r#type::r#enum::{StorageData, StorageDataType},
            table, Column,
        },
    };
    use common::structs::hash_table::MutHashTable;

    use crate::api::command::{
        extract::SchemaExtractionError,
        gateway::{test::TestBackendFacade, GatewayError},
        Gateway,
    };

    use super::{ExecutionError, Insert};

    #[test]
    fn create_table_when_not_exists() {
        let database_name = database::Name::from("db");
        let schema_name = schema::Name::from("test");
        let table_name = table::Name::from("table");
        let column_name = column::Name::from("id");

        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .with_schema(database_name.clone(), schema_name.clone())
            .with_table(
                database_name.clone(),
                schema_name.clone(),
                table_name.clone(),
            )
            .with_column(
                database_name.clone(),
                schema_name.clone(),
                table_name.clone(),
                column_name.clone(),
                StorageDataType::Integer,
            )
            .with_primary_key(
                database_name.clone(),
                schema_name.clone(),
                table_name.clone(),
                column_name.clone(),
            )
            .build();

        let mut data = DataUnit::new(vec![column_name.clone()]);

        data.insert(vec![StorageData::Integer(1.into())].into());
        data.insert(vec![StorageData::Integer(2.into())].into());
        data.insert(vec![StorageData::Integer(3.into())].into());

        let cmd = Insert {
            database_name: Some(database_name.clone()),
            schema_name: Some(schema_name.clone()),
            name: table_name.clone(),
            data,
        };

        let result = facade.send(cmd);
        assert!(result.is_ok());

        let db = facade
            .database_controllers
            .get_mut_value(&database_name)
            .unwrap();
        let schema = db.get_mut_schema(&schema_name).unwrap();
        let table = schema.get_mut_table(&table_name);
        assert!(table.is_some());
        let table = table.unwrap();
    }
}
