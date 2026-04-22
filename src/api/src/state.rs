use tokio::sync::broadcast;
use std::sync::Arc;

pub struct AppHub {
    pub tx: broadcast::Sender<String>,
}

pub struct AppState {
    pub hub: AppHub,
}

pub type SharedState = Arc<AppState>;
