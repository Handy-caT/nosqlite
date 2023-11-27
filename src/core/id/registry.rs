use crate::core::{
    id::{
        link_storage::LinkStorage,
        numeric_id_generator::{NumericId, NumericIdGenerator},
        IdGenerator,
    },
    link_struct::PageLink,
};

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

    /// Removes a [`PageLink`] from the [`Registry`].
    /// # Arguments
    /// * `link` - The [`PageLink`] to remove.
    /// # Returns
    /// * `Result<(), RegistryError>` - Ok if the [`PageLink`] was removed.
    /// Err if the [`PageLink`] was not found.
    /// # Errors
    /// * `RegistryError::LinkNotFound` - The [`PageLink`] was not found.
    pub fn remove_link(&mut self, link: PageLink) -> Result<(), RegistryError> {
        let result = self.link_storage.remove_by_link(link);

        match result {
            Some(id) => {
                self.id_generator.retrieve_id(id);
                Ok(())
            }
            None => Err(RegistryError::LinkNotFound),
        }
    }

    /// Updates a [`PageLink`] in the [`Registry`] by [`NumericId`].
    /// # Arguments
    /// * `id` - The [`NumericId`] of the [`PageLink`] to update.
    /// * `link` - The new [`PageLink`].
    /// # Returns
    /// * `Result<(), RegistryError>` - Ok if the [`PageLink`] was updated.
    /// Err if the [`NumericId`] was not found.
    /// # Errors
    /// * `RegistryError::IdNotFound` - The [`NumericId`] was not found.
    pub fn update_link(
        &mut self,
        id: NumericId,
        link: PageLink,
    ) -> Result<(), RegistryError> {
        let link_res = self.link_storage.get_link(id);

        match link_res {
            Some(_) => {
                self.link_storage.insert(link, id);
                Ok(())
            }
            None => Err(RegistryError::IdNotFound),
        }
    }

    /// Gets a [`PageLink`] from the [`Registry`] by [`NumericId`].
    /// # Arguments
    /// * `id` - The [`NumericId`] of the [`PageLink`] to get.
    /// # Returns
    /// * `Option<PageLink>` - The [`PageLink`] that was found.
    /// None if the [`NumericId`] was not found.
    pub fn get_link(&mut self, id: NumericId) -> Option<PageLink> {
        self.link_storage.get_link(id)
    }

    /// Gets a [`NumericId`] from the [`Registry`] by [`PageLink`].
    /// # Arguments
    /// * `link` - The [`PageLink`] of the [`NumericId`] to get.
    /// # Returns
    /// * `Option<NumericId>` - The [`NumericId`] that was found.
    /// None if the [`PageLink`] was not found.
    pub fn get_id(&mut self, link: PageLink) -> Option<NumericId> {
        self.link_storage.get_id(link)
    }
}

/// Errors that can occur when using a [`Registry`].
pub enum RegistryError {
    LinkNotFound,
    IdNotFound,
}

#[cfg(test)]
mod tests {
    use crate::core::{
        id::{
            numeric_id_generator::{NumericId, NumericIdGenerator},
            registry::Registry,
            IdGenerator,
        },
        link_struct::PageLink,
    };

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

    #[test]
    fn test_registry_remove_link() {
        let mut registry = Registry::<NumericIdGenerator>::new();

        let link = PageLink::new(1, 0, 100);

        let id = registry.add_link(link);

        assert_eq!(registry.link_storage.len(), 1);
        assert_eq!(registry.id_generator.get_id_count(), 1);
        assert_eq!(id, NumericId::new(1));

        let result = registry.remove_link(link);

        assert!(result.is_ok());

        assert_eq!(registry.link_storage.len(), 0);
        assert_eq!(registry.id_generator.get_id_count(), 0);
    }

    #[test]
    fn test_registry_remove_link_not_found() {
        let mut registry = Registry::<NumericIdGenerator>::new();

        let link = PageLink::new(1, 0, 100);

        let id = registry.add_link(link);

        assert_eq!(registry.link_storage.len(), 1);
        assert_eq!(registry.id_generator.get_id_count(), 1);
        assert_eq!(id, NumericId::new(1));

        let link = PageLink::new(2, 0, 100);

        let result = registry.remove_link(link);

        assert!(result.is_err());

        assert_eq!(registry.link_storage.len(), 1);
        assert_eq!(registry.id_generator.get_id_count(), 1);
    }

    #[test]
    fn test_registry_update_link() {
        let mut registry = Registry::<NumericIdGenerator>::new();

        let link = PageLink::new(1, 0, 100);

        let id = registry.add_link(link);

        assert_eq!(registry.link_storage.len(), 1);
        assert_eq!(registry.id_generator.get_id_count(), 1);
        assert_eq!(id, NumericId::new(1));

        let link = PageLink::new(1, 100, 200);

        let result = registry.update_link(id, link);

        assert!(result.is_ok());

        assert_eq!(registry.link_storage.len(), 1);
        assert_eq!(registry.id_generator.get_id_count(), 1);
    }

    #[test]
    fn test_registry_update_link_not_found() {
        let mut registry = Registry::<NumericIdGenerator>::new();

        let link = PageLink::new(1, 0, 100);

        let id = registry.add_link(link);

        assert_eq!(registry.link_storage.len(), 1);
        assert_eq!(registry.id_generator.get_id_count(), 1);
        assert_eq!(id, NumericId::new(1));

        let link = PageLink::new(2, 100, 200);

        let result = registry.update_link(NumericId::new(2), link);

        assert!(result.is_err());

        assert_eq!(registry.link_storage.len(), 1);
        assert_eq!(registry.id_generator.get_id_count(), 1);
    }

    #[test]
    fn test_registry_get_link() {
        let mut registry = Registry::<NumericIdGenerator>::new();

        let link = PageLink::new(1, 0, 100);
        let id = registry.add_link(link);

        let link = registry.get_link(id);

        assert_eq!(link, Some(PageLink::new(1, 0, 100)));
    }

    #[test]
    fn test_registry_get_link_not_found() {
        let mut registry = Registry::<NumericIdGenerator>::new();

        let link = PageLink::new(1, 0, 100);
        let id = registry.add_link(link);

        let link = registry.get_link(NumericId::new(2));

        assert_eq!(link, None);
    }
}
