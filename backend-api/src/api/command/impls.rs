use backend::{
    schema,
    schema::{database, table},
};

use crate::{
    api::command::{
        self, ContextReceiver, DatabaseCommand, OptionalBy, SchemaCommand,
        TableCommand,
    },
    Context,
};

impl<T> ContextReceiver<()> for T {
    fn receive(&mut self, _: &Context) {}
}

impl<T> OptionalBy<database::Name> for T
where
    T: DatabaseCommand,
{
    type Err = command::database::ProvideError;

    fn by(&self) -> Result<database::Name, Self::Err> {
        self.get_db_name()
            .clone()
            .ok_or(command::database::ProvideError::DatabaseNotProvided)
    }
}

impl<T> ContextReceiver<database::Name> for T
where
    T: DatabaseCommand,
{
    fn receive(&mut self, context: &Context) {
        if self.get_db_name().is_none() {
            *self.get_db_name_mut() = context.current_db().cloned();
        }
    }
}

impl<T> OptionalBy<(database::Name, schema::Name)> for T
where
    T: SchemaCommand,
{
    type Err = command::schema::ProvideError;

    fn by(&self) -> Result<(database::Name, schema::Name), Self::Err> {
        let db_name = self
            .get_db_name()
            .clone()
            .ok_or(command::schema::ProvideError::DatabaseNotProvided)?;
        let schema_name = self
            .get_schema_name()
            .clone()
            .ok_or(command::schema::ProvideError::SchemaNotProvided)?;
        Ok((db_name, schema_name))
    }
}

impl<T> ContextReceiver<(database::Name, schema::Name)> for T
where
    T: SchemaCommand,
{
    fn receive(&mut self, context: &Context) {
        if self.get_db_name().is_none() {
            *self.get_db_name_mut() = context.current_db().cloned();
        }
        if self.get_schema_name().is_none() {
            *self.get_schema_name_mut() = context.current_schema().cloned();
        }
    }
}

impl<T> OptionalBy<(database::Name, schema::Name, table::Name)> for T
where
    T: TableCommand,
{
    type Err = command::table::ProvideError;

    fn by(
        &self,
    ) -> Result<(database::Name, schema::Name, table::Name), Self::Err> {
        let db_name = self
            .get_db_name()
            .clone()
            .ok_or(command::table::ProvideError::DatabaseNotProvided)?;
        let schema_name = self
            .get_schema_name()
            .clone()
            .ok_or(command::table::ProvideError::SchemaNotProvided)?;
        let table_name = self.get_table_name().clone();

        Ok((db_name, schema_name, table_name))
    }
}

impl<T> ContextReceiver<(database::Name, schema::Name, table::Name)> for T
where
    T: TableCommand,
{
    fn receive(&mut self, context: &Context) {
        if self.get_db_name().is_none() {
            *self.get_db_name_mut() = context.current_db().cloned();
        }
        if self.get_schema_name().is_none() {
            *self.get_schema_name_mut() = context.current_schema().cloned();
        }
    }
}
