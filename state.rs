#[derive(Clone)]
pub struct AppState {
    // Put your shared resources here
    // e.g. pub db: DatabaseConnection
}

impl AppState {
    pub fn new() -> Self {
        Self {
            // initialize state
        }
    }
}
