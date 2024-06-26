use crate::{
    api::{
        command::{Extract, TryExtract},
        facade::BackendFacade,
    },
    Context,
};
use backend::{
    controller, schema,
    schema::{database, table},
};
use common::structs::hash_table::MutHashTable;
use derive_more::Display;

impl<const NODE_SIZE: u8> Extract<Context> for BackendFacade<NODE_SIZE> {
    fn extract_mut(&mut self) -> &mut Context {
        &mut self.context
    }
}

impl<const NODE_SIZE: u8> Extract<Self> for BackendFacade<NODE_SIZE> {
    fn extract_mut(&mut self) -> &mut Self {
        self
    }
}

impl<const NODE_SIZE: u8> TryExtract<controller::Database<NODE_SIZE>>
    for BackendFacade<NODE_SIZE>
{
    type Err = DatabaseExtractionError;
    type By = database::Name;

    fn try_extract_mut(
        &mut self,
        name: database::Name,
    ) -> Result<&mut controller::Database<NODE_SIZE>, Self::Err> {
        self.database_controllers
            .get_mut_value(&name)
            .ok_or(DatabaseExtractionError::DatabaseNotFound(name))
    }
}

/// Represents an error that occurred during the extraction of a database.
#[derive(Debug, Display, PartialEq, Clone)]
pub enum DatabaseExtractionError {
    /// The database was not found.
    #[display(fmt = "Database with {} name not found", _0)]
    DatabaseNotFound(database::Name),
}

impl<const NODE_SIZE: u8> TryExtract<controller::Schema<NODE_SIZE>>
    for BackendFacade<NODE_SIZE>
{
    type Err = SchemaExtractionError;
    type By = (database::Name, schema::Name);

    fn try_extract_mut(
        &mut self,
        (db_name, schema_name): (database::Name, schema::Name),
    ) -> Result<&mut controller::Schema<NODE_SIZE>, Self::Err> {
        let db_controller =
            self.database_controllers.get_mut_value(&db_name).ok_or(
                SchemaExtractionError::DatabaseNotFound(db_name.clone()),
            )?;

        db_controller
            .get_mut_schema(&schema_name)
            .ok_or(SchemaExtractionError::SchemaNotFound(schema_name, db_name))
    }
}

/// Represents an error that occurred during the extraction of a schema.
#[derive(Debug, Display, PartialEq, Clone)]
pub enum SchemaExtractionError {
    /// The database was not found.
    #[display(fmt = "Database with {} name not found", _0)]
    DatabaseNotFound(database::Name),

    /// The schema was not found.
    #[display(fmt = "Schema with {} name not found in {} db", _0, _1)]
    SchemaNotFound(schema::Name, database::Name),
}

impl<const NODE_SIZE: u8> TryExtract<controller::Table<NODE_SIZE>>
    for BackendFacade<NODE_SIZE>
{
    type Err = TableExtractionError;
    type By = (database::Name, schema::Name, table::Name);

    fn try_extract_mut(
        &mut self,
        (db_name, schema_name, table_name): (
            database::Name,
            schema::Name,
            table::Name,
        ),
    ) -> Result<&mut controller::Table<NODE_SIZE>, Self::Err> {
        let db_controller =
            self.database_controllers
                .get_mut_value(&db_name)
                .ok_or(TableExtractionError::Database(db_name.clone()))?;

        let schema_controller = db_controller
            .get_mut_schema(&schema_name)
            .ok_or(TableExtractionError::Schema(
                schema_name.clone(),
                db_name.clone(),
            ))?;

        schema_controller.get_mut_table(&table_name).ok_or(
            TableExtractionError::Table(table_name, schema_name, db_name),
        )
    }
}

/// Represents an error that occurred during the extraction of a table.
#[derive(Debug, Display, PartialEq, Clone)]
pub enum TableExtractionError {
    /// The database was not found.
    #[display(fmt = "Database with {} name not found", _0)]
    Database(database::Name),

    /// The schema was not found.
    #[display(fmt = "Schema with {} name not found in {} db", _0, _1)]
    Schema(schema::Name, database::Name),

    /// The table was not found.
    #[display(
        fmt = "Table with {} name not found in {} schema in {} db",
        _0,
        _1,
        _2
    )]
    Table(table::Name, schema::Name, database::Name),
}
