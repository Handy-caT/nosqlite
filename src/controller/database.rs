use crate::{controller, schema, schema as info};
use common::structs::hash_table::scalable::ScalableHashTable;

/// Controller for a single table.
/// Is used to change the table's schema and data.
#[derive(Debug)]
pub struct Database<const NODE_SIZE: u8> {
    /// The database information.
    info: info::Database,

    /// The schemas in the database.
    tables: ScalableHashTable<schema::Name, controller::Schema<NODE_SIZE>>,
}
