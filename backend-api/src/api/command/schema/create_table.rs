use crate::{
    api::{
        command::{Command, ContextReceiver, OptionalBy},
        CommandResultString,
    },
    Context,
};
use backend::{
    controller,
    controller::table::TableControllerError,
    schema,
    schema::{
        column, column::primary_key::PrimaryKey, database, table, Column,
    },
};
use derive_more::Display;

/// [`Command`] to create a new table in a database.
#[derive(Debug, Clone, PartialEq)]
pub struct CreateTable {
    /// The name of the database where the table will be created.
    pub database_name: Option<database::Name>,

    /// The name of the schema where the table will be created.
    pub schema_name: Option<schema::Name>,

    /// The name of the table to create.
    pub name: table::Name,

    /// The columns of the table.
    pub columns: Vec<(column::Name, Column)>,

    /// The primary key of the table.
    pub primary_key: PrimaryKey,
}

impl OptionalBy<(database::Name, schema::Name)> for CreateTable {
    fn by(&self) -> Option<(database::Name, schema::Name)> {
        self.database_name.as_ref().and_then(|db_name| {
            self.schema_name
                .as_ref()
                .map(|schema_name| (db_name.clone(), schema_name.clone()))
        })
    }
}

impl ContextReceiver for CreateTable {
    fn receive(&mut self, context: &Context) {
        if self.database_name.is_none() {
            self.database_name = context.current_db().cloned();
        }
        if self.schema_name.is_none() {
            self.schema_name = context.current_schema().cloned();
        }
    }
}

impl<const NODE_SIZE: u8> Command<controller::Schema<NODE_SIZE>>
    for CreateTable
{
    type Ok = CommandResultString;
    type Err = ExecutionError;

    fn execute(
        self,
        schema_controller: &mut controller::Schema<NODE_SIZE>,
    ) -> Result<Self::Ok, Self::Err> {
        if schema_controller.has_table(&self.name) {
            return Err(ExecutionError::TableAlreadyExists(
                self.database_name.expect("exists"),
                self.schema_name.expect("exists"),
                self.name,
            ));
        }

        let mut table = controller::Table::new(self.name.clone());
        for (column_name, column) in self.columns {
            table.add_column(column_name, column);
        }
        table
            .set_primary_key(self.primary_key)
            .map_err(ExecutionError::TableControllerError)?;

        if schema_controller.add_table(table) {
            Ok(CommandResultString {
                result: format!(
                    "Table `{}`.`{}`.`{}` created",
                    self.database_name.expect("exists"),
                    self.schema_name.expect("exists"),
                    self.name
                ),
            })
        } else {
            Err(ExecutionError::TableAlreadyExists(
                self.database_name.expect("exists"),
                self.schema_name.expect("exists"),
                self.name,
            ))
        }
    }
}

/// Errors that can occur during the execution of [`CreateTable`].
#[derive(Debug, Display)]
pub enum ExecutionError {
    /// The schema already exists in the database.
    #[display(fmt = "Table `{}`.`{}`.`{}` already exists", _0, _1, _2)]
    TableAlreadyExists(database::Name, schema::Name, table::Name),

    /// The table controller error.
    #[display(fmt = "{:?}", _0)]
    TableControllerError(TableControllerError),
}

#[cfg(test)]
mod tests {
    use backend::{
        schema,
        schema::{
            column, column::primary_key, database,
            r#type::r#enum::StorageDataType, table, Column,
        },
    };
    use common::structs::hash_table::MutHashTable;

    use crate::api::command::{
        extract::SchemaExtractionError,
        gateway::{test::TestBackendFacade, GatewayError},
        Gateway,
    };

    use super::{CreateTable, ExecutionError};

    #[test]
    fn create_table_when_not_exists() {
        let database_name = database::Name::from("db");
        let schema_name = schema::Name::from("test");
        let table_name = table::Name::from("table");
        let column_name = column::Name::from("id");

        let mut test_cases = Vec::new();

        let facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .with_schema(database_name.clone(), schema_name.clone())
            .build();

        let column = Column::new(StorageDataType::Integer);

        let pk_name = primary_key::Name::from("pk");
        let primary_key =
            primary_key::PrimaryKey::new(pk_name.clone(), column_name.clone());

        let cmd = CreateTable {
            database_name: Some(database_name.clone()),
            schema_name: Some(schema_name.clone()),
            name: table_name.clone(),
            columns: vec![(column_name.clone(), column.clone())],
            primary_key: primary_key.clone(),
        };

        test_cases.push((facade, cmd));

        let facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .with_schema(database_name.clone(), schema_name.clone())
            .with_db_in_context(database_name.clone())
            .build();

        let cmd = CreateTable {
            database_name: None,
            schema_name: Some(schema_name.clone()),
            name: table_name.clone(),
            columns: vec![(column_name.clone(), column.clone())],
            primary_key: primary_key.clone(),
        };

        test_cases.push((facade, cmd));

        let facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .with_schema(database_name.clone(), schema_name.clone())
            .with_db_in_context(database_name.clone())
            .with_schema_in_context(schema_name.clone())
            .build();

        let cmd = CreateTable {
            database_name: None,
            schema_name: None,
            name: table_name.clone(),
            columns: vec![(column_name.clone(), column.clone())],
            primary_key: primary_key.clone(),
        };

        test_cases.push((facade, cmd));

        for (mut facade, cmd) in test_cases.into_iter() {
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

            assert_eq!(table.get_name(), &table_name);
            let column = table.get_column(&column_name);
            assert!(column.is_some());
            let column = column.unwrap();
            assert_eq!(column.get_type(), StorageDataType::Integer);

            let primary_key = table.get_primary_key();
            assert!(primary_key.is_some());
            let primary_key = primary_key.as_ref().unwrap();
            assert_eq!(primary_key.get_name(), &pk_name);
            assert_eq!(primary_key.get_column(), &column_name);
        }
    }

    #[test]
    fn returns_error_when_table_exists() {
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
            .build();

        let column = Column::new(StorageDataType::Integer);

        let pk_name = primary_key::Name::from("pk");
        let primary_key =
            primary_key::PrimaryKey::new(pk_name.clone(), column_name.clone());

        let cmd = CreateTable {
            database_name: Some(database_name.clone()),
            schema_name: Some(schema_name.clone()),
            name: table_name.clone(),
            columns: vec![(column_name.clone(), column.clone())],
            primary_key: primary_key.clone(),
        };

        let result = facade.send(cmd);
        assert!(result.is_err());

        match result {
            Err(GatewayError::CommandError(
                ExecutionError::TableAlreadyExists(db_name, schema_name, name),
            )) => {
                assert_eq!(db_name, database_name);
                assert_eq!(schema_name, schema_name);
                assert_eq!(name, table_name);
            }
            _ => panic!("Expected `TableAlreadyExists` found {:?}", result),
        }
    }

    #[test]
    fn returns_error_when_db_not_exists() {
        let database_name = database::Name::from("db");
        let schema_name = schema::Name::from("test");
        let table_name = table::Name::from("table");
        let column_name = column::Name::from("id");

        let mut facade = TestBackendFacade::<4>::new().build();

        let column = Column::new(StorageDataType::Integer);

        let pk_name = primary_key::Name::from("pk");
        let primary_key =
            primary_key::PrimaryKey::new(pk_name.clone(), column_name.clone());

        let cmd = CreateTable {
            database_name: Some(database_name.clone()),
            schema_name: Some(schema_name.clone()),
            name: table_name.clone(),
            columns: vec![(column_name.clone(), column.clone())],
            primary_key: primary_key.clone(),
        };
        let result = facade.send(cmd);
        assert!(result.is_err());

        match result {
            Err(GatewayError::ExtractionError(
                SchemaExtractionError::DatabaseNotFound(name),
            )) => {
                assert_eq!(name, database_name);
            }
            _ => panic!("Expected `DatabaseNotFound` found {:?}", result),
        }
    }

    #[test]
    fn returns_error_when_schema_not_exists() {
        let database_name = database::Name::from("db");
        let schema_name = schema::Name::from("test");
        let table_name = table::Name::from("table");
        let column_name = column::Name::from("id");

        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .build();

        let column = Column::new(StorageDataType::Integer);

        let pk_name = primary_key::Name::from("pk");
        let primary_key =
            primary_key::PrimaryKey::new(pk_name.clone(), column_name.clone());

        let cmd = CreateTable {
            database_name: Some(database_name.clone()),
            schema_name: Some(schema_name.clone()),
            name: table_name.clone(),
            columns: vec![(column_name.clone(), column.clone())],
            primary_key: primary_key.clone(),
        };
        let result = facade.send(cmd);
        assert!(result.is_err());

        match result {
            Err(GatewayError::ExtractionError(
                SchemaExtractionError::SchemaNotFound(name, db_name),
            )) => {
                assert_eq!(db_name, database_name);
                assert_eq!(name, schema_name);
            }
            _ => panic!("Expected `SchemaNotFound` found {:?}", result),
        }
    }
}
