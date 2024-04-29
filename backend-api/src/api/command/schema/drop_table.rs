use std::convert::Infallible;

use backend::{
    controller, schema,
    schema::{database, table},
};

use crate::{
    api::{
        command::{
            schema::{CreateTable, ProvideError},
            Command, ContextReceiver, OptionalBy,
        },
        CommandResultString,
    },
    Context,
};

/// [`Command`] to drop a new table in a database.
#[derive(Debug, Clone, PartialEq)]
pub struct DropTable {
    /// The name of the database where the table will be created.
    pub database_name: Option<database::Name>,

    /// The name of the schema where the table will be created.
    pub schema_name: Option<schema::Name>,

    /// The name of the table to create.
    pub name: table::Name,
}

impl OptionalBy<(database::Name, schema::Name)> for DropTable {
    type Err = ProvideError;

    fn by(&self) -> Result<(database::Name, schema::Name), Self::Err> {
        let db_name = self
            .database_name
            .clone()
            .ok_or(ProvideError::DatabaseNotProvided)?;
        let schema_name = self
            .schema_name
            .clone()
            .ok_or(ProvideError::SchemaNotProvided)?;

        Ok((db_name, schema_name))
    }
}

impl ContextReceiver for DropTable {
    fn receive(&mut self, context: &Context) {
        if self.database_name.is_none() {
            self.database_name = context.current_db().cloned();
        }
        if self.schema_name.is_none() {
            self.schema_name = context.current_schema().cloned();
        }
    }
}

impl<const NODE_SIZE: u8> Command<controller::Schema<NODE_SIZE>> for DropTable {
    type Ok = CommandResultString;
    type Err = ExecutionError;

    fn execute(
        self,
        schema_controller: &mut controller::Schema<NODE_SIZE>,
    ) -> Result<Self::Ok, Self::Err> {
        if !schema_controller.has_table(&self.name) {
            return Ok(CommandResultString::default());
        }

        let _ = schema_controller.remove_table(&self.name);
        Ok(CommandResultString {
            result: format!(
                "Table `{}`.`{}`.`{}` dropped",
                self.database_name.expect("exists"),
                self.schema_name.expect("exists"),
                self.name
            ),
        })
    }
}

/// Errors that can occur during the execution of [`DropTable`].
pub type ExecutionError = Infallible;

#[cfg(test)]
mod tests {
    use backend::{schema, schema::database};
    use common::structs::hash_table::MutHashTable;

    use crate::api::command::{
        database::DropSchema,
        gateway::{test::TestBackendFacade, GatewayError},
        schema::drop_table::DropTable,
        Gateway,
    };

    #[test]
    fn drops_table_when_exists() {
        let database_name = database::Name::from("test");
        let schema_name = schema::Name::from("schema");
        let table_name = schema::table::Name::from("table");

        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .with_schema(database_name.clone(), schema_name.clone())
            .with_table(
                database_name.clone(),
                schema_name.clone(),
                table_name.clone(),
            )
            .build();

        let cmd = DropTable {
            database_name: Some(database_name.clone()),
            schema_name: Some(schema_name.clone()),
            name: table_name.clone(),
        };
        let result = facade.send(cmd);
        assert!(result.is_ok());

        let db = facade
            .database_controllers
            .get_mut_value(&database_name)
            .unwrap();
        let schema = db.get_mut_schema(&schema_name).unwrap();
        assert!(!schema.has_table(&table_name));
    }

    #[test]
    fn drops_table_when_exists_with_db_in_context() {
        let database_name = database::Name::from("test");
        let schema_name = schema::Name::from("schema");
        let table_name = schema::table::Name::from("table");

        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .with_schema(database_name.clone(), schema_name.clone())
            .with_table(
                database_name.clone(),
                schema_name.clone(),
                table_name.clone(),
            )
            .with_db_in_context(database_name.clone())
            .build();

        let cmd = DropTable {
            database_name: None,
            schema_name: Some(schema_name.clone()),
            name: table_name.clone(),
        };
        let result = facade.send(cmd);
        assert!(result.is_ok());

        let db = facade
            .database_controllers
            .get_mut_value(&database_name)
            .unwrap();
        let schema = db.get_mut_schema(&schema_name).unwrap();
        assert!(!schema.has_table(&table_name));
    }

    #[test]
    fn drops_table_when_exists_with_schema_in_context() {
        let database_name = database::Name::from("test");
        let schema_name = schema::Name::from("schema");
        let table_name = schema::table::Name::from("table");

        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .with_schema(database_name.clone(), schema_name.clone())
            .with_table(
                database_name.clone(),
                schema_name.clone(),
                table_name.clone(),
            )
            .with_db_in_context(database_name.clone())
            .with_schema_in_context(schema_name.clone())
            .build();

        let cmd = DropTable {
            database_name: None,
            schema_name: None,
            name: table_name.clone(),
        };
        let result = facade.send(cmd);
        assert!(result.is_ok());

        let db = facade
            .database_controllers
            .get_mut_value(&database_name)
            .unwrap();
        let schema = db.get_mut_schema(&schema_name).unwrap();
        assert!(!schema.has_table(&table_name));
    }

    #[test]
    fn returns_error_when_db_not_provided() {
        let database_name = database::Name::from("test");
        let schema_name = schema::Name::from("schema");
        let table_name = schema::table::Name::from("table");

        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .with_schema(database_name.clone(), schema_name.clone())
            .with_table(
                database_name.clone(),
                schema_name.clone(),
                table_name.clone(),
            )
            .build();

        let cmd = DropTable {
            database_name: None,
            schema_name: None,
            name: table_name.clone(),
        };
        let result = facade.send(cmd);
        assert!(result.is_err());

        match result {
            Err(GatewayError::ByNotProvided) => {}
            _ => panic!("Expected `ByNotProvided` found {:?}", result),
        }
    }

    #[test]
    fn not_errors_when_table_not_exists() {
        let database_name = database::Name::from("test");
        let schema_name = schema::Name::from("schema");
        let table_name = schema::table::Name::from("table");

        let mut facade = TestBackendFacade::<4>::new()
            .with_database(database_name.clone())
            .with_schema(database_name.clone(), schema_name.clone())
            .build();

        let cmd = DropTable {
            database_name: Some(database_name.clone()),
            schema_name: Some(schema_name.clone()),
            name: table_name.clone(),
        };
        let result = facade.send(cmd);
        assert!(result.is_ok());
    }
}
