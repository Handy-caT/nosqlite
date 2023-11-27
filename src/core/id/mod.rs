mod link_storage;
pub mod numeric_id_generator;
mod registry;

pub trait IdGenerator<Id> {
    /// Creates a new [`IdGenerator`].
    /// # Returns
    /// * `Self` - [`IdGenerator`].
    fn new() -> Self;

    /// Get's a new [`Id`]
    /// # Returns
    /// * `dyn Id` - New [`Id`]
    fn get_id(&mut self) -> Id;

    /// Returns an [`Id`] to the [`IdGenerator`]
    /// # Arguments
    /// * `id` - [`Id`] to return
    fn retrieve_id(&mut self, id: Id);

    /// Returns the number of [`Id`]s that have been returned
    /// from the [`IdGenerator`].
    fn get_id_count(&self) -> u64;
}
