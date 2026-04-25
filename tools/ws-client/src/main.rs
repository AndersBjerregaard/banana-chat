use futures_util::StreamExt;
use std::io::{self, Write};
use tokio::io::{AsyncBufReadExt, BufReader};
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws_url = "ws://localhost:3000/ws/subscribe";
    let base_notify_url = "http://localhost:3000/notify";
    let client = Client::new();

    println!("🛠️ Interactive Hub Client Started");
    print!("✍️ Enter a username: ");
    io::stdout().flush()?; // Ensure prompt appears before input

    let mut stdin_reader = BufReader::new(tokio::io::stdin());
    let mut username = String::new();
    stdin_reader.read_line(&mut username).await?;
    let username = username.trim();

    let ws_full_url = format!("{}/{}", ws_url, username);
    println!("🔗 Connecting to {}...", ws_full_url);

    let (ws_stream, _) = tokio_tungstenite::connect_async(&ws_full_url).await?;
    let (mut _write, mut read) = ws_stream.split();

    // Task for incoming messages
    tokio::spawn(async move {
        while let Some(message) = read.next().await {
            match message {
                Ok(msg) => println!("\n📥 [Broadcast Received]: {}", msg),
                Err(e) => {
                    eprintln!("\n🔌 Connection error: {}", e);
                    break;
                }
            }
        }
        println!("\n🔌 Server closed the connection. Press Enter to exit.");
    });

    // Main Input Loop
    loop {
        print!("✍️ Enter message to broadcast: ");
        io::stdout().flush()?;

        let mut input = String::new();
        stdin_reader.read_line(&mut input).await?;
        let input = input.trim();

        if input.is_empty() || input == "exit" || input == "quit" {
            break;
        }

        let notify_url = format!("{}/{}", base_notify_url, urlencoding::encode(input));

        match client.post(&notify_url).send().await {
            Ok(resp) if !resp.status().is_success() => {
                eprintln!("⚠️ Failed to notify: {}", resp.status());
            }
            Err(e) => eprintln!("❌ Fetch error: {}", e),
            _ => {} // Success
        }
    }

    println!("👋 Closing connection...");
    Ok(())
}
