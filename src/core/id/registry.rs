use std::process::id;
use crate::core::id::{
    link_storage::LinkStorage,
    numeric_id_generator::{NumericId, NumericIdGenerator},
    IdGenerator,
};
use crate::core::link_struct::PageLink;

/// A registry for [`Id`]s.
/// It can be used to store [`Id`]s and retrieve them.
struct Registry<G = NumericIdGenerator> {
    id_generator: G,
    link_storage: LinkStorage,
}

impl<G> Registry<G>
where
    G: IdGenerator<NumericId>,
{
    /// Creates a new [`Registry`].
    /// # Returns
    /// * `Self` - [`Registry`].
    pub fn new() -> Self {
        Self {
            id_generator: G::new(),
            link_storage: LinkStorage::new(10),
        }
    }

    /// Adds a new [`PageLink`] to the [`Registry`].
    /// # Arguments
    /// * `link` - The [`PageLink`] to add.
    /// # Returns
    /// * `NumericId` - The [`NumericId`] of the [`PageLink`].
    pub fn add_link(&mut self, link: PageLink) -> NumericId {
        let mut result_id = self.id_generator.get_id();
        let updated_id = self.link_storage.insert(link, result_id);
        if let Some(id) = updated_id {
            if id != result_id {
                self.id_generator.retrieve_id(result_id);
                self.link_storage.insert(link, id);
                result_id = id;
            }
        }
        result_id
    }
}

#[cfg(test)]
mod tests {
    use crate::core::id::{
        numeric_id_generator::NumericIdGenerator, registry::Registry,
        IdGenerator,
    };
    use crate::core::id::numeric_id_generator::NumericId;
    use crate::core::link_struct::PageLink;

    #[test]
    fn test_registry_new() {
        let registry = Registry::<NumericIdGenerator>::new();

        assert_eq!(registry.link_storage.len(), 0);
        assert_eq!(registry.id_generator.get_id_count(), 0);
    }

    #[test]
    fn test_registry_add_link() {
        let mut registry = Registry::<NumericIdGenerator>::new();

        let link = PageLink::new(1, 0, 100);

        let id = registry.add_link(link);

        assert_eq!(registry.link_storage.len(), 1);
        assert_eq!(registry.id_generator.get_id_count(), 1);
        assert_eq!(id, NumericId::new(1));
    }

    #[test]
    fn test_registry_add_link_existing() {
        let mut registry = Registry::<NumericIdGenerator>::new();

        let link = PageLink::new(1, 0, 100);

        let id = registry.add_link(link);

        assert_eq!(registry.link_storage.len(), 1);
        assert_eq!(registry.id_generator.get_id_count(), 1);
        assert_eq!(id, NumericId::new(1));

        let link = PageLink::new(1, 0, 100);

        let id = registry.add_link(link);

        assert_eq!(registry.link_storage.len(), 1);
        assert_eq!(registry.id_generator.get_id_count(), 1);
        assert_eq!(id, NumericId::new(1));
    }

    #[test]
    fn test_registry_add_after_retrieving() {
        let mut registry = Registry::<NumericIdGenerator>::new();

        let link = PageLink::new(1, 0, 100);

        let id = registry.add_link(link);

        assert_eq!(registry.link_storage.len(), 1);
        assert_eq!(registry.id_generator.get_id_count(), 1);
        assert_eq!(id, NumericId::new(1));

        let link = PageLink::new(1, 0, 100);

        let id = registry.add_link(link);

        assert_eq!(registry.link_storage.len(), 1);
        assert_eq!(registry.id_generator.get_id_count(), 1);
        assert_eq!(id, NumericId::new(1));

        let link = PageLink::new(1, 100, 200);

        let id = registry.add_link(link);

        assert_eq!(registry.link_storage.len(), 2);
        assert_eq!(registry.id_generator.get_id_count(), 2);
        assert_eq!(id, NumericId::new(2));
    }
}
