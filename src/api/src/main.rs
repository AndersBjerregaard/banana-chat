mod state;
mod routes;
mod handlers;

use std::sync::Arc;
use tokio::sync::broadcast;
use crate::state::{AppState, AppHub};

#[tokio::main]
async fn main() {
    // 1. Initialize State (The Hub)
    let (tx, _) = broadcast::channel(16);
    let state = Arc::new(AppState {
        hub: AppHub { tx },
    });

    // 2. Build Router
    let app = routes::create_router(state);

    // 3. Run Server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server started on 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
