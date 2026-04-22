use axum::{
    extract::{Path, State, ws::{Message, WebSocket, WebSocketUpgrade}},
    response::IntoResponse,
};
use futures_util::{StreamExt, SinkExt};
use crate::state::SharedState;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
    Path(username): Path<String>
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state, username))
}

async fn handle_socket(
    socket: WebSocket,
    state: SharedState,
    username: String
) {
    let (mut sender, _) = socket.split();
    let mut rx = state.hub.tx.subscribe();
    println!("User {} established connection.", username);

    while let Ok(msg) = rx.recv().await {
        if sender.send(Message::Text(msg.into())).await.is_err() {
            break;
        }
    }
}
