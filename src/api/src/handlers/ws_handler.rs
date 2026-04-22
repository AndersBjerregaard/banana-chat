use axum::{
    extract::{Path, State, ws::{Message, WebSocket, WebSocketUpgrade}},
    response::IntoResponse,
};
use chrono::{DateTime, Local};
use futures_util::{SinkExt, StreamExt, stream::{SplitSink, SplitStream}};
use tokio::sync::broadcast::Receiver;
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
    let (mut sender, _): (SplitSink<WebSocket, Message>, SplitStream<WebSocket>) = socket.split();
    let mut rx: Receiver<String> = state.hub.tx.subscribe();
    let con_local_now: DateTime<Local> = Local::now();
    println!("[{}] User {} established connection.", con_local_now, username);

    while let Ok(msg) = rx.recv().await {
        let local_now: DateTime<Local> = Local::now();
        let full_message: String = format!("[{}] {}: {}", local_now, username, msg);
        if sender.send(Message::Text(full_message.into())).await.is_err() {
            break;
        }
    }
}
