use crate::{
    api::command::{
        database::{CreateSchema, DropSchema},
        Command, ContextReceiver, OptionalBy,
    },
    Context,
};
use backend::{
    controller,
    data::data_storage::DataStorage,
    schema,
    schema::{column, database, table, Column},
};

/// [`Command`] to create a new schema in a database.
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
    type Ok = ();
    type Err = ExecutionError;

    fn execute(
        self,
        schema_controller: &mut controller::Schema<NODE_SIZE>,
    ) -> Result<Self::Ok, Self::Err> {
        if schema_controller.has_table(&self.name) {
            return Err(ExecutionError::TableAlreadyExists(self.name));
        }

        // let table = controller::Table::new(self.name.clone());
        Ok(())
    }
}

/// Errors that can occur during the execution of [`CreateTable`].
#[derive(Debug)]
pub enum ExecutionError {
    /// The schema already exists in the database.
    TableAlreadyExists(table::Name),
}
