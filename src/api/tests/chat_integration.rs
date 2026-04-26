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

    assert_eq!("hello", value);
}
