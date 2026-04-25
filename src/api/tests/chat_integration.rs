use std::sync::Arc;
use axum::{Router, body::Body, http::{Request, Response, StatusCode}};
use banana_chat::{routes, state::{AppHub, AppState}};
use tokio::sync::broadcast;
use tower::{Service, ServiceExt};

#[tokio::test]
async fn it_broadcasts() {
    let (tx, _): (broadcast::Sender<String>, broadcast::Receiver<String>) = broadcast::channel(16);
    let state = Arc::new(AppState {
        hub: AppHub { tx },
    });

    let mut app: Router = routes::create_router(state);

    let request: Request<Body> = Request::builder()
        .method("POST")
        .uri("http://localhost:3000/notify/hello")
        .body(Body::empty())
        .unwrap();

    let response: Response<Body> = app.as_service().ready().await.unwrap().call(request).await.unwrap();

    let status: StatusCode = response.status();

    assert!(status.is_success());
}
