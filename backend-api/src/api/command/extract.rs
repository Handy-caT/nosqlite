use crate::{
    api::{
        command::{Extract, TryExtractBy},
        facade::BackendFacade,
    },
    Context,
};
use backend::{controller, schema, schema::database};
use common::structs::hash_table::MutHashTable;
use derive_more::Display;

impl<const NODE_SIZE: u8> Extract<Context> for BackendFacade<NODE_SIZE> {
    fn extract(&mut self) -> &mut Context {
        &mut self.context
    }
}

impl<const NODE_SIZE: u8> Extract<Self> for BackendFacade<NODE_SIZE> {
    fn extract(&mut self) -> &mut Self {
        self
    }
}

impl<const NODE_SIZE: u8> TryExtractBy<controller::Database<NODE_SIZE>>
    for BackendFacade<NODE_SIZE>
{
    type Err = DatabaseExtractionError;
    type By = database::Name;

    fn try_extract(
        &mut self,
        name: &database::Name,
    ) -> Result<&mut controller::Database<NODE_SIZE>, Self::Err> {
        self.database_controllers
            .get_mut_value(name)
            .ok_or(DatabaseExtractionError::DatabaseNotFound(name.clone()))
    }
}

/// Represents an error that occurred during the extraction of a database.
#[derive(Debug, Display, PartialEq, Clone)]
pub enum DatabaseExtractionError {
    /// The database was not found.
    #[display(fmt = "Database with {} name not found", _0)]
    DatabaseNotFound(database::Name),
}

impl<const NODE_SIZE: u8> TryExtractBy<controller::Schema<NODE_SIZE>>
    for BackendFacade<NODE_SIZE>
{
    type Err = SchemaExtractionError;
    type By = (database::Name, schema::Name);

    fn try_extract(
        &mut self,
        (db_name, schema_name): &(database::Name, schema::Name),
    ) -> Result<&mut controller::Schema<NODE_SIZE>, Self::Err> {
        let db_controler = self
            .database_controllers
            .get_mut_value(db_name)
            .ok_or(SchemaExtractionError::DatabaseNotFound(db_name.clone()))?;

        db_controler.get_mut_schema(schema_name).ok_or(
            SchemaExtractionError::SchemaNotFound(
                schema_name.clone(),
                db_name.clone(),
            ),
        )
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
