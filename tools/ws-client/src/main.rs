use std::{io};

use futures_util::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::{client::IntoClientRequest, http::Request}};

#[tokio::main]
async fn main() {
    let ws_url: String = String::from("ws://localhost:3000/ws/subscribe");
    // let base_notify_url: String = String::from("http://localhost:3000/notify");

    println!("Interactive Hub Client Started");

    println!("Enter a username: ");

    let mut username: String = String::new();

    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");

    let ws_full_url: String = format!("{}/{}", ws_url, username.trim());

    println!("Connecting to {}", ws_full_url);

    // Connect to websocket
    let request: Request<()> = ws_full_url.as_str().into_client_request().unwrap();

    let (stream, _) = connect_async(request).await.expect("Failed to connect");
    let (_, read) = stream.split();

    tokio::spawn(async move {
        read.for_each(|message| async {
            match message {
                Ok(msg) => println!("\n [Broadcast Received]: {}", msg),
                Err(e) => eprintln!("Error: {}", e),
            }
        }).await;
    });

    println!("Ready!");
}
