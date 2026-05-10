use axum::{
    extract::{Path, State, ws::{Message, WebSocketUpgrade}},
    response::IntoResponse,
};
use chrono::{DateTime, Local};
use futures::Stream;
use futures_util::{Sink, SinkExt, StreamExt};
use tokio::sync::broadcast::Receiver;
use crate::state::SharedState;
type AxMsg = axum::extract::ws::Message;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
    Path(username): Path<String>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| async move {
        let (sender, receiver) = socket.split();

        handle_socket(
            sender,
            receiver,
            state,
            username,
        )
        .await;
    })
}

async fn handle_socket<S, R>(
    mut sender: S,
    mut receiver: R,
    state: SharedState,
    username: String
) where 
    S: Sink<AxMsg> + Unpin,
    R: Stream<Item = Result<AxMsg, axum::Error>> + Unpin
{
    let mut rx: Receiver<String> = state.hub.tx.subscribe();

    let con_local_now: DateTime<Local> = Local::now();

    println!(
        "[{}] User {} established connection.",
        con_local_now,
        username
    );

    loop {
        tokio::select! {
            // Messages coming FROM the client
            ws_msg = receiver.next() => {
                match ws_msg {
                    Some(Ok(Message::Close(_))) => {
                        println!("{} disconnected gracefully", username);
                        break;
                    }
                    Some(Ok(Message::Ping(payload))) => {
                        if sender.send(Message::Pong(payload)).await.is_err() {
                            break;
                        }
                    }
                    Some(Ok(_)) => {
                        // Ignore other incoming messages
                    }
                    Some(Err(err)) => {
                        eprintln!("websocket error for {}: {}", username, err);
                        break;
                    }
                    None => {
                        // Stream ended
                        println!("{} connection closed", username);
                        break;
                    }
                }
            }
            // Messages coming FROM broadcast hub
            hub_msg = rx.recv() => {
                match hub_msg {
                    Ok(msg) => {
                        let local_now: DateTime<Local> = Local::now();

                        let full_message = format!(
                            "[{}] {}",
                            local_now,
                            msg
                        );

                        if sender
                            .send(Message::Text(full_message.into()))
                            .await
                            .is_err()
                        {
                            println!("failed sending to {}", username);
                            break;
                        }
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(skipped)) => {
                        eprintln!(
                            "{} lagged behind and missed {} messages",
                            username,
                            skipped
                        );
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                        println!("broadcast channel closed");
                        break;
                    }
                }
            }
        }
    }

    println!("Cleaning up websocket for {}", username);
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

        let (sender, mut sent_messages) = mpsc::unbounded::<AxMsg>();

        let receiver = futures_util::stream::pending::<Result<AxMsg, axum::Error>>();

        let username = "testuser".to_string();

        tokio::spawn(async move {
            handle_socket(sender, receiver, state, username).await;
        });

        // Yield to let spawned task above subscribe to the channel.
        tokio::task::yield_now().await;

        match tx.send("Hello World".to_string()) {
            Ok(subscribers) => subscribers,
            Err(e) => panic!("Error sending: {}", e),
        };

        // Await message being asynchronously broadcasted
        let received: Message = timeout(Duration::from_millis(100), sent_messages.next())
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
