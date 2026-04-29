use crate::state::SharedState;

pub async fn notify_handler(
    axum::extract::Path(msg): axum::extract::Path<String>,
    axum::extract::State(state): axum::extract::State<SharedState>,
) -> () {
    println!("Broadcasting message...");
    let _ = state.hub.tx.send(msg);
}
