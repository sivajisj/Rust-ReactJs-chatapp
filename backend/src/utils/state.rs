use tokio::sync::{broadcast, Mutex};

pub struct AppState {
    pub tx: broadcast::Sender<String>,
    pub users: Mutex<Vec<String>>,
}

impl AppState {
    pub fn new(tx: broadcast::Sender<String>, users: Mutex<Vec<String>>) -> Self {
        AppState { tx, users }
    }
}
