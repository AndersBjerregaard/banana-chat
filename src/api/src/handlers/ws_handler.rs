use axum::{
    extract::{Path, State, ws::{Message, WebSocketUpgrade}},
    response::IntoResponse,
};
use chrono::{DateTime, Local};
use futures_util::{Sink, SinkExt, StreamExt};
use tokio::sync::broadcast::Receiver;
use crate::state::SharedState;
type AxMsg = axum::extract::ws::Message;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
    Path(username): Path<String>
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket.split().0, state, username))
}

async fn handle_socket<S>(
    mut sender: S,
    state: SharedState,
    username: String
)
where S: Sink<AxMsg> + Unpin,
{
    let mut rx: Receiver<String> = state.hub.tx.subscribe();
    let con_local_now: DateTime<Local> = Local::now();
    println!("[{}] User {} established connection.", con_local_now, username);

    while let Ok(msg) = rx.recv().await {
        let local_now: DateTime<Local> = Local::now();
        let full_message: String = format!("[{}]: {}", local_now,  msg);

        if sender.send(Message::Text(full_message.into())).await.is_err() {
            eprintln!("Error receiving message");
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{sync::Arc, time::Duration};
    use futures::channel::mpsc;
    use tokio::{sync::broadcast::{self}, time::timeout};
    use crate::state::{AppHub, AppState};

    #[tokio::test]
    async fn it_receives() {
        let (tx, _) = broadcast::channel(16);

        // Ensure tx.send always has at least 1 subscriber.
        // If a message is sent while there's no subscribers, it will panic.
        let mut _dummy_rx = tx.subscribe();

        let state = Arc::new(AppState {
            hub: AppHub { tx: tx.clone() }
        });

        let (sender, mut receiver) = mpsc::unbounded::<AxMsg>();

        let username = "testuser".to_string();

        tokio::spawn(async move {
            handle_socket(sender, state, username).await;
        });

        // Yield to let spawned task above subscribe to the channel.
        tokio::task::yield_now().await;

        match tx.send("Hello World".to_string()) {
            Ok(subscribers) => subscribers,
            Err(e) => panic!("Error sending: {}", e),
        };

        // Await message being asynchronously broadcasted
        let received: Message = timeout(Duration::from_millis(100), receiver.next())
            .await
            .expect("Timeout waiting for message")
            .expect("Channel closed");

        if let AxMsg::Text(text) = received {
            assert!(text.contains("Hello World"));
        } else {
            panic!("Received wrong message types!");
        }
    }
}
