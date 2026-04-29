use axum::Router;
use crate::state::SharedState;

mod websocket;
mod notify;

pub fn create_router(state: SharedState) -> Router {
    Router::new()
        .nest("/ws", websocket::routes())
        .nest("/notify", notify::routes())
        .with_state(state)
}
