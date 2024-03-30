use common::structs::hash_table::{
    backwards_hash_table::BackwardsHashTable, HashTable,
};

use crate::{
    data::id::numeric_id_generator::NumericId, page::link_struct::PageLink,
};

/// A registry for getting [`NumericId`]s from [`PageLink`]s
/// and vice versa.
#[derive(Debug)]
pub struct LinkStorage {
    /// A [`BackwardsHashTable`] for getting [`NumericId`]s from [`PageLink`]s.
    data: BackwardsHashTable<PageLink, NumericId>,
}

impl LinkStorage {
    /// Creates a new [`LinkStorage`].
    /// # Arguments
    /// * `size` - The size of the [`LinkStorage`].
    /// # Returns
    /// * `LinkRegistry` - The new [`LinkStorage`].
    pub fn new(size: usize) -> Self {
        Self {
            data: BackwardsHashTable::new(size),
        }
    }

    /// Gets a [`NumericId`] from a [`PageLink`].
    /// # Arguments
    /// * `link` - The [`PageLink`] to get the [`NumericId`] from.
    /// # Returns
    /// * `Option<NumericId>` - The [`NumericId`] from the [`PageLink`].
    pub fn get_id(&mut self, link: PageLink) -> Option<NumericId> {
        self.data.get(&link)
    }

    /// Gets a [`PageLink`] from a [`NumericId`].
    /// # Arguments
    /// * `id` - The [`NumericId`] to get the [`PageLink`] from.
    /// # Returns
    /// * `Option<PageLink>` - The [`PageLink`] from the [`NumericId`].
    pub fn get_link(&mut self, id: NumericId) -> Option<PageLink> {
        self.data.get_by_value(&id)
    }

    /// Inserts a [`PageLink`] and [`NumericId`] into the [`LinkStorage`].
    /// # Arguments
    /// * `link` - The [`PageLink`] to insert.
    /// * `id` - The [`NumericId`] to insert.
    /// # Returns
    /// * `Option<NumericId>` - The [`NumericId`] that was replaced.
    pub fn insert(
        &mut self,
        link: PageLink,
        id: NumericId,
    ) -> Option<NumericId> {
        self.data.insert(link, id).map(|x| x.value)
    }

    /// Removes a [`PageLink`] and [`NumericId`] from the [`LinkStorage`]
    /// by [`PageLink`].
    /// # Arguments
    /// * `link` - The [`PageLink`] to remove.
    /// # Returns
    /// * `Option<NumericId>` - The [`NumericId`] that was removed.
    pub fn remove_by_link(&mut self, link: PageLink) -> Option<NumericId> {
        self.data.remove(&link)
    }

    /// Removes a [`PageLink`] and [`NumericId`] from the [`LinkStorage`]
    /// by [`NumericId`].
    /// # Arguments
    /// * `id` - The [`NumericId`] to remove.
    /// # Returns
    /// * `Option<PageLink>` - The [`PageLink`] that was removed.
    pub fn remove_by_id(&mut self, id: NumericId) -> Option<PageLink> {
        self.data.remove_by_value(&id)
    }

    /// Returns the number of elements in the [`LinkStorage`].
    /// # Returns
    /// * `usize` - The number of elements in the [`LinkStorage`].
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use common::structs::hash_table::HashTable;

    use crate::{
        data::id::{
            link_storage::LinkStorage, numeric_id_generator::NumericId,
        },
        page::link_struct::PageLink,
    };

    #[test]
    fn test_link_registry_new() {
        let link_registry = LinkStorage::new(10);

        assert_eq!(link_registry.len(), 0);
        assert_eq!(link_registry.data.len(), 0);
    }

    #[test]
    fn test_link_registry_insert() {
        let mut link_registry = LinkStorage::new(10);

        let link = PageLink::new(1, 0, 100);
        let id = NumericId::new(1);

        let replaced_id = link_registry.insert(link, id);

        assert_eq!(link_registry.len(), 1);
        assert_eq!(replaced_id, Some(id));
    }

    #[test]
    fn test_link_registry_get_id() {
        let mut link_registry = LinkStorage::new(10);

        let link = PageLink::new(1, 0, 100);
        let id = NumericId::new(1);

        let replaced_id = link_registry.insert(link, id);

        assert_eq!(link_registry.len(), 1);
        assert_eq!(replaced_id, Some(id));

        let replaced_id = link_registry.get_id(link);

        assert_eq!(link_registry.len(), 1);
        assert_eq!(replaced_id, Some(id));
    }

    #[test]
    fn test_link_registry_get_link() {
        let mut link_registry = LinkStorage::new(10);

        let link = PageLink::new(1, 0, 100);
        let id = NumericId::new(1);

        let replaced_id = link_registry.insert(link, id);

        assert_eq!(link_registry.len(), 1);
        assert_eq!(replaced_id, Some(id));

        let replaced_link = link_registry.get_link(id);

        assert_eq!(link_registry.len(), 1);
        assert_eq!(replaced_link, Some(link));
    }

    #[test]
    fn test_link_registry_insert_replace() {
        let mut link_registry = LinkStorage::new(10);

        let link = PageLink::new(1, 0, 100);
        let id = NumericId::new(1);

        let replaced_id = link_registry.insert(link, id);

        assert_eq!(link_registry.len(), 1);
        assert_eq!(replaced_id, Some(id));

        let link = PageLink::new(2, 0, 100);
        let id = NumericId::new(1);

        let replaced_id = link_registry.insert(link, id);

        assert_eq!(link_registry.len(), 1);
        assert_eq!(replaced_id, Some(id));

        let replaced_id = link_registry.get_id(link);
        let replaced_link = link_registry.get_link(id);

        assert_eq!(replaced_id, Some(id));
        assert_eq!(replaced_link, Some(link));
    }

    #[test]
    fn test_link_registry_remove_by_link() {
        let mut link_registry = LinkStorage::new(10);

        let link = PageLink::new(1, 0, 100);
        let id = NumericId::new(1);

        let replaced_id = link_registry.insert(link, id);

        assert_eq!(link_registry.len(), 1);
        assert_eq!(replaced_id, Some(id));

        let replaced_id = link_registry.remove_by_link(link);

        assert_eq!(link_registry.len(), 0);
        assert_eq!(replaced_id, Some(id));
    }

    #[test]
    fn test_link_registry_remove_by_id() {
        let mut link_registry = LinkStorage::new(10);

        let link = PageLink::new(1, 0, 100);
        let id = NumericId::new(1);

        let replaced_id = link_registry.insert(link, id);

        assert_eq!(link_registry.len(), 1);
        assert_eq!(replaced_id, Some(id));

        let replaced_link = link_registry.remove_by_id(id);

        assert_eq!(link_registry.len(), 0);
        assert_eq!(replaced_link, Some(link));
    }
}
