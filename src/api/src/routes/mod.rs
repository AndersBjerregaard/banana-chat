use axum::Router;
use crate::state::SharedState;

mod websocket;

pub fn create_router(state: SharedState) -> Router {
    Router::new()
        .nest("/ws", websocket::routes()) // Nesting for organization
        .route("/notify/{msg}", axum::routing::post(trigger_notification))
        .with_state(state)
}

async fn trigger_notification(
    axum::extract::Path(msg): axum::extract::Path<String>,
    axum::extract::State(state): axum::extract::State<SharedState>,
) -> () {
    println!("Broadcasting message...");
    let _ = state.hub.tx.send(msg);
}
