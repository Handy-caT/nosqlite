use backend::{schema, schema::database};

/// The context of the current session.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Context {
    /// The current database.
    current_db: Option<database::Name>,

    /// The current schema.
    current_schema: Option<schema::Name>,
}

impl Context {
    /// Sets the current database.
    pub fn set_current_db(&mut self, db: database::Name) {
        self.current_db = Some(db);
    }

    /// Sets the current schema.
    pub fn set_current_schema(&mut self, schema: schema::Name) {
        self.current_schema = Some(schema);
    }

    /// Gets the current database.
    pub fn current_db(&self) -> Option<&database::Name> {
        self.current_db.as_ref()
    }

    /// Gets the current schema.
    pub fn current_schema(&self) -> Option<&schema::Name> {
        self.current_schema.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_new() {
        let context = Context::default();
        assert_eq!(context.current_db(), None);
        assert_eq!(context.current_schema(), None);
    }

    #[test]
    fn test_context_set_current_db() {
        let mut context = Context::default();
        context.set_current_db(database::Name("db1".into()));
        assert_eq!(context.current_db(), Some(&database::Name("db1".into())));
    }

    #[test]
    fn test_context_set_current_schema() {
        let mut context = Context::default();
        context.set_current_schema(schema::Name("schema1".into()));
        assert_eq!(
            context.current_schema(),
            Some(&schema::Name("schema1".into()))
        );
    }
}
