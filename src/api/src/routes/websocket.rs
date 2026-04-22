use axum::{routing::get, Router};
use crate::handlers::ws_handler::ws_handler;
use crate::state::SharedState;

pub fn routes() -> Router<SharedState> {
    Router::new().route("/subscribe/{username}", get(ws_handler))
}
