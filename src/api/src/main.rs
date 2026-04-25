use std::sync::Arc;
use banana_chat::{routes, state::{AppHub, AppState}};
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel(16);
    let state = Arc::new(AppState {
        hub: AppHub { tx },
    });

    let app = routes::create_router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server started on 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
