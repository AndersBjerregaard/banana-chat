use axum::http::HeaderMap;

use crate::state::SharedState;

pub async fn notify_handler(
    axum::extract::Path(msg): axum::extract::Path<String>,
    headers: HeaderMap,
    axum::extract::State(state): axum::extract::State<SharedState>,
) -> () {
    handle_notification(msg, headers, state).await;
}

async fn handle_notification(
    msg: String,
    headers: HeaderMap,
    state: SharedState
) {
    let user: &str = match headers.get("x-user") {
        Some(h) => h.to_str().unwrap(),
        None => "anon",
    };
    println!("Broadcasting message...");
    let usr_msg: String = format!("{}: {}", user, msg);
    let _ = state.hub.tx.send(usr_msg);
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc};
    use axum::http::HeaderMap;
    use tokio::sync::broadcast;
    use crate::{handlers::notify_handler::handle_notification, state::{AppHub, AppState}};

    #[tokio::test]
    async fn it_sends_username() {
        // Arrange
        let (tx, _): (broadcast::Sender<String>, broadcast::Receiver<String>) = broadcast::channel::<String>(2);

        let mut rx: broadcast::Receiver<String> = tx.subscribe();

        let state = Arc::new(AppState {
            hub: AppHub { tx: tx.clone() }
        });

        let msg: String = String::from("Hello, World!");

        let username: String = String::from("testuser");

        let mut headers: HeaderMap = HeaderMap::new();

        headers.insert("x-user", username.parse().unwrap());

        // Act
        handle_notification(msg.clone(), headers, state).await;

        // Assert
        match rx.try_recv() {
            Ok(broadcast) => assert_eq!(broadcast, format!("{}: {}", username, msg)),
            Err(e) => panic!("Error receiving: {}", e),
        }
    }

    #[tokio::test]
    async fn it_defaults_to_anon() {
        // Arrange
        let (tx, _): (broadcast::Sender<String>, broadcast::Receiver<String>) = broadcast::channel::<String>(2);

        let mut rx: broadcast::Receiver<String> = tx.subscribe();

        let state = Arc::new(AppState {
            hub: AppHub { tx: tx.clone() }
        });

        let msg: String = String::from("Hello, World!");

        let headers: HeaderMap = HeaderMap::new();

        // Act
        handle_notification(msg.clone(), headers, state).await;

        // Assert
        match rx.try_recv() {
            Ok(broadcast) => assert_eq!(broadcast, format!("{}: {}", "anon", msg)),
            Err(e) => panic!("Error receiving: {}", e),
        }
    }
}
