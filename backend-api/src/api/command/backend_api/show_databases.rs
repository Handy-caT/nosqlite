use std::convert::Infallible;

use common::structs::hash_table::VecFunctions;
use prettytable::{format, row};

use crate::api::{
    command::Command, facade::BackendFacade, CommandResultString,
};

/// Command to show databases.
#[derive(Debug, Clone, PartialEq)]
pub struct ShowDatabases {}

impl<const NODE_SIZE: u8> Command<BackendFacade<NODE_SIZE>> for ShowDatabases {
    type Ok = CommandResultString;
    type Err = ExecutionError;

    fn execute(
        self,
        backend: &mut BackendFacade<NODE_SIZE>,
    ) -> Result<Self::Ok, Self::Err> {
        let databases = backend.database_controllers.get_keys();

        let mut table = prettytable::Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(row!["databases"]);

        for db in databases {
            table.add_row(row![db.0.as_str()]);
        }

        Ok(CommandResultString {
            result: table.to_string(),
        })
    }
}

/// Errors that can occur when executing the [`ShowDatabases`] command.
pub type ExecutionError = Infallible;

#[cfg(test)]
mod tests {
    use backend::schema::database;

    use crate::api::command::{gateway::test::TestBackendFacade, Gateway};

    use super::ShowDatabases;

    #[test]
    fn show_databases_when_exists() {
        let name = database::Name::from("test");
        let mut facade = TestBackendFacade::<4>::new()
            .with_database(name.clone())
            .build();
        let cmd = ShowDatabases {};
        let result = facade.send(cmd);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.result.contains(name.0.as_str()));
    }
}
