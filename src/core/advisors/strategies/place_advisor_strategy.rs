use crate::core::link_struct::PageLink;

/// Trait that is used for place advisor strategies.
/// Place advisor strategy is used to provide place for new data.
/// It is using EmptyLinkRegistry to find empty places.
/// If there is no empty place, that can be used, it is using last page.
pub trait PlaceAdvisorStrategy {
    /// Provides place for new data.
    /// # Arguments
    /// * `size` - size of data that should be placed
    /// # Returns
    /// * `PageLink` - link to place where data should be placed
    fn provide_place(&self, size: u64) -> PageLink;

    /// Applies place to link.
    /// # Arguments
    /// * `link` - link to place where data should be placed
    /// * `size` - size of data that should be placed
    fn apply_place(&self, link: &PageLink, size: u64);

    /// Returns name of strategy.
    /// # Returns
    /// * `String` - name of strategy
    fn get_name(&self) -> String;
}