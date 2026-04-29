use axum::http::HeaderMap;

use crate::state::SharedState;

pub async fn notify_handler(
    axum::extract::Path(msg): axum::extract::Path<String>,
    headers: HeaderMap,
    axum::extract::State(state): axum::extract::State<SharedState>,
) -> () {
    let user: &str = match headers.get("x-user") {
        Some(h) => h.to_str().unwrap(),
        None => "anon",
    };
    println!("Broadcasting message...");
    let usr_msg: String = format!("{}: {}", user, msg);
    let _ = state.hub.tx.send(usr_msg);
}
