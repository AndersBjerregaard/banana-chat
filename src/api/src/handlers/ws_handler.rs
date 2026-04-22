use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
    response::IntoResponse,
};
use futures_util::{StreamExt, SinkExt};
use crate::state::SharedState;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: SharedState) {
    let (mut sender, _) = socket.split();
    let mut rx = state.hub.tx.subscribe();

    while let Ok(msg) = rx.recv().await {
        if sender.send(Message::Text(msg.into())).await.is_err() {
            break;
        }
    }
}
