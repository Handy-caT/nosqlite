use backend_api::api::facade::BackendFacade;
use frontend::FrontendApi;

/// Represents the API.
#[derive(Debug, Default)]
pub struct Api<const NODE_SIZE: u8> {
    /// The backend API.
    pub backend_api: BackendFacade<NODE_SIZE>,

    /// The frontend API.
    pub frontend_api: FrontendApi,
}
