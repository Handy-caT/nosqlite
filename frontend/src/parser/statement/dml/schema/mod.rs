mod alter_schema;
mod create_schema;
mod drop_schema;

pub use alter_schema::AlterSchema;
pub use create_schema::CreateSchema;
pub use drop_schema::DropSchema;

/// Represents an AST node for a schema operation.
#[derive(Debug, PartialEq, Clone)]
pub enum SchemaNode {
    /// Represents a `DROP SCHEMA ...` statement.
    DropSchema(DropSchema),

    /// Represents a `CREATE SCHEMA ...` statement.
    CreateSchema(CreateSchema),

    /// Represents a `ALTER SCHEMA ...` statement.
    AlterSchema(AlterSchema),
}
