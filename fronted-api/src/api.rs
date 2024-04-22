use backend_api::api::facade::BackendFacade;
use frontend::FrontendApi;

/// Represents the API.
#[derive(Debug, Default)]
pub struct Api<const NODE_SIZE: u8> {
    /// The backend API.
    pub backend_api: BackendFacade<NODE_SIZE>,

    /// The frontend API.
    pub frontend_api: FrontendApi,
    
    /// The context.
    pub context: Context,
}

impl<const NODE_SIZE: u8> Api<NODE_SIZE> {
    /// Checks if the REPL should quit.
    pub fn need_quit(&self) -> bool {
        self.context.quit
    }
}

/// Represents the context.
#[derive(Debug, Default)]
pub struct Context {
    /// Represents if the REPL should quit.
    pub quit: bool,
}