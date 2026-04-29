use axum::Router;
use crate::handlers::notify_handler::notify_handler;
use crate::state::SharedState;

pub fn routes() -> Router<SharedState> {
    Router::new().route("/{msg}", axum::routing::post(notify_handler))
}
