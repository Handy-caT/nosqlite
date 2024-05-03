use backend::{controller, schema::database};
use prettytable::{format, row};
use std::convert::Infallible;

use crate::api::{
    command::{Command, DatabaseCommand, OptionalBy},
    CommandResultString,
};

/// [`Command`] to show schemas from a database.
#[derive(Debug, Clone, PartialEq)]
pub struct ShowSchemas {
    /// The name of the database to show the schemas from.
    pub database_name: Option<database::Name>,
}

impl DatabaseCommand for ShowSchemas {
    fn get_db_name(&self) -> Option<database::Name> {
        self.database_name.clone()
    }

    fn get_db_name_mut(&mut self) -> &mut Option<database::Name> {
        &mut self.database_name
    }
}

impl<const NODE_SIZE: u8> Command<controller::Database<NODE_SIZE>>
    for ShowSchemas
{
    type Ok = CommandResultString;
    type Err = ExecutionError;

    fn execute(
        self,
        db_controller: &mut controller::Database<NODE_SIZE>,
    ) -> Result<Self::Ok, Self::Err> {
        let schemas = db_controller.get_schema_names();

        let mut table = prettytable::Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(row!["schemas"]);

        for schema in schemas {
            table.add_row(row![schema.0.as_str()]);
        }

        Ok(CommandResultString {
            result: table.to_string(),
        })
    }
}

/// Errors that can occur during the execution of [`DropSchema`].
pub type ExecutionError = Infallible;

#[cfg(test)]
mod tests {
    use backend::{schema, schema::database};

    use crate::api::command::{gateway::test::TestBackendFacade, Gateway as _};

    use super::ShowSchemas;

    #[test]
    fn show_schemas_when_exists() {
        let database_name = database::Name::from("test");
        let schema_name = schema::Name::from("schema");

        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .with_schema(database_name.clone(), schema_name.clone())
            .build();
        let cmd = ShowSchemas {
            database_name: Some(database_name.clone()),
        };
        let result = facade.send(cmd);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.result.contains(&schema_name.0));
    }
}
