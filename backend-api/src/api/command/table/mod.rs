mod insert;

use derive_more::Display;

/// Errors that can occur when executing the [`TableCommand`].
#[derive(Debug, Display)]
pub enum ProvideError {
    /// The database was not provided.
    #[display(fmt = "Database not provided in the `Context`.\n\
                     Use the `USE DATABASE` command to set the current \
                     database or use `db_name`.`schema_name`.`table_name` to \
                     specify the database and schema names.")]
    DatabaseNotProvided,

    /// The schema was not provided.
    #[display(fmt = "Schema not provided in the `Context`\n\
                     Use the `USE SCHEMA` command to set the current schema \
                     or use `schema_name`.`table_name` to specify the schema \
                     name.")]
    SchemaNotProvided,
}
