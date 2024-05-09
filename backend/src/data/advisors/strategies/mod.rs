pub mod best_fit_advisor;
pub mod place_advisor_strategy;
pub mod worst_fit_advisor;

/// Enum that represents type of [`PlaceAdvisorStrategy`].
/// # Values
/// * `BestFit` - [`BestFitAdvisor`]
/// * `WorstFit` - [`WorstFitAdvisor`]
pub enum PlaceAdvisorStrategyType {
    /// [`BestFitAdvisor`] strategy.
    BestFit,
    /// [`WorstFitAdvisor`] strategy.
    WorstFit,
}

/// Trait that is used for types that are used
/// as context for [`PlaceAdvisorStrategy`].
pub trait PlaceAdvisorContext {
    /// Sets [`PlaceAdvisorStrategy`] for context.
    /// # Arguments
    /// * `strategy` - [`PlaceAdvisorStrategyType`] to set.
    fn set_strategy(&mut self, strategy: PlaceAdvisorStrategyType);

    /// Gets [`PlaceAdvisorStrategy`] for context.
    /// # Returns
    /// * `PlaceAdvisorStrategyType` - [`PlaceAdvisorStrategyType`] for context.
    fn get_strategy(&self) -> PlaceAdvisorStrategyType;
}
