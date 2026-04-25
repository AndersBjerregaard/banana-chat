use std::{borrow::Cow, io};

use futures_util::{StreamExt};
use reqwest::StatusCode;
use tokio_tungstenite::{connect_async, tungstenite::{client::IntoClientRequest, http::Request}};
use urlencoding::encode;

#[tokio::main]
async fn main() {
    let ws_url: String = String::from("ws://localhost:3000/ws/subscribe");
    let base_notify_url: String = String::from("http://localhost:3000/notify");

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

    let client = reqwest::Client::new();

    loop {
        println!("Enter a message to broadcast: ");

        let mut message: String = String::new();

        io::stdin()
            .read_line(&mut message)
            .expect("Failed to read line");

        let message_str: &str = message.trim();

        if message_str == "quit" || message_str == "exit" {
            break;
        }

        println!("Broadcasting '{}' ...", message_str);

        let encoded: Cow<str> = encode(message_str);

        let notify_url: String = format!("{}/{}", base_notify_url, encoded);

        let response = client.post(notify_url)
            .send()
            .await
            .expect("Error broadcasting message");

        let status: StatusCode = response.status();

        if !status.is_success() {
            println!("Error response: {response:#?}");
        }
    }

    println!("Exiting...");
}
