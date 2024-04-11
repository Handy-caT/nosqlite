mod create_schema;
mod drop_schema;
mod alter_schema;

pub use create_schema::CreateSchema;
pub use drop_schema::DropSchema;
pub use alter_schema::AlterSchema;

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