use std::{sync::Arc};
use axum::{Router, body::Body, http::{Request, Response}};
use banana_chat::{routes, state::{AppHub, AppState}};
use tokio::sync::broadcast;
use tower::{ServiceExt};

#[tokio::test]
async fn it_broadcasts() {
    let (tx, _): (broadcast::Sender<String>, broadcast::Receiver<String>) = broadcast::channel(16);
    let state = Arc::new(AppState {
        hub: AppHub { tx },
    });
    let mut rx: broadcast::Receiver<String> = state.hub.tx.subscribe();
    let app: Router = routes::create_router(state);

    let request: Request<Body> = Request::builder()
        .method("POST")
        .uri("/notify/hello")
        .body(Body::empty())
        .unwrap();

    let response: Response<Body> = app.oneshot(request).await.unwrap();

    assert!(response.status().is_success());

    let value: String = rx.try_recv().unwrap();

    // Expect user to be set as 'anon' because no username was set
    assert_eq!("anon: hello", value);
}

#[tokio::test]
async fn it_uses_websocket_protocol() {
    let (tx, _): (broadcast::Sender<String>, broadcast::Receiver<String>) = broadcast::channel(16);
    let state = Arc::new(AppState { hub: AppHub { tx } });
    
    // Create router with only the route that is being tested
    let app: Router = Router::new()
        .route("/ws/subscribe/{username}", axum::routing::get(banana_chat::handlers::ws_handler::ws_handler))
        .with_state(state);

    // Bind to port 0 (the OS will pick any available port)
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr: std::net::SocketAddr = listener.local_addr().unwrap();

    // Spawn the server so it doesn't block the test
    tokio::spawn(async move {
        axum::serve(
            listener,
            app
        )
        .await
        .unwrap();
    });

    // Use a real WS client to connect
    let url: String = format!("ws://{}/ws/subscribe/test", addr);
    let (_, response): (tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, Response<Option<Vec<u8>>>) = tokio_tungstenite::connect_async(url).await.expect("Failed to connect");

    assert_eq!(response.status(), 101);
}
