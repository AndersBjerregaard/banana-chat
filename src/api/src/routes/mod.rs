use axum::Router;
use tower_http::cors::CorsLayer;
use crate::state::SharedState;

mod websocket;
mod notify;

pub fn create_router(state: SharedState, cors: CorsLayer) -> Router {
    Router::new()
        .nest("/ws", websocket::routes())
        .nest("/notify", notify::routes())
        .layer(cors)
        .with_state(state)
}
