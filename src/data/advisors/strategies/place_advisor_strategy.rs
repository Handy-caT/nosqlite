use crate::page::link_struct::PageLink;

/// Trait that is used for place advisor strategies.
/// Place advisor strategy is used to provide place for new data.
/// It is using [`EmptyLinkRegistry`] to find empty places.
pub trait PlaceAdvisorStrategy {
    /// Provides place for new data.
    /// # Arguments
    /// * `size` - size of data that should be placed
    /// # Returns
    /// * `Option<PageLink>` - link to place where data should be placed,
    /// None if there is no place
    fn provide_place(&mut self, size: u16) -> Option<PageLink>;

    /// Applies place to link.
    /// # Arguments
    /// * `link` - link to place where data should be placed
    /// * `size` - size of data that should be placed
    fn apply_place(&mut self, link: &PageLink, size: u16);

    /// Returns name of strategy.
    /// # Returns
    /// * `String` - name of strategy
    fn get_name(&self) -> String;
}
