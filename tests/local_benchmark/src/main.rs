use std::{env, time::{Duration, Instant}};
use hdrhistogram::Histogram;
use reqwest::Client;
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, tungstenite::{http::Response}};
use tokio::{net::TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = env::args().collect();

    if args.is_empty() || args.len() == 1 {
        panic!("No arguments were passed! Invoke with an unsigned integer as argument, e.g.: {0}", 
"cargo run -- 1000");
    }

    let ws_url: &str = "http://localhost:3000/ws/subscribe";

    let iterations_string: &String = &args[1];

    let iterations: u64 = match iterations_string.parse() {
        Ok(num) => num,
        Err(_) => panic!("Argument could not be parsed as an unsigned integer!"),
    };

    println!("Pinging server...");

let client = Client::new();

    match client
        .head(ws_url)
        .send()
        .await {
            Ok(resp) if !resp.status().is_success() => {
                panic!("Server did not respond with success status code: {}", resp.status());
            }
            Err(e) => panic!("Error trying to reach server: {0}", e),
            _ => println!("Server up!")
        };

    println!("Starting benchmark with {0} iterations...", iterations_string);

    let full = Instant::now();

    // Histogram over microsecond precision
    let mut hist: Histogram<u64> = Histogram::<u64>::new_with_bounds(1, 60 * 60 * 1_000_000, 3).unwrap();

    for _ in 0..iterations {
        let start = Instant::now();

        let ws_full_url: &str = "ws://localhost:3000/ws/subscribe/testuser";

        let (ws_stream, _): (WebSocketStream<MaybeTlsStream<TcpStream>>, Response<Option<Vec<u8>>>) = tokio_tungstenite::connect_async(ws_full_url).await?;
        let (mut write, mut read) = ws_stream.split();

        let notify_url: &str = "http://localhost:3000/notify/hello";

        match client
            .post(notify_url)
            .header("x-user", "test")
            .send()
            .await {
                Ok(resp) if !resp.status().is_success() => {
                    panic!("Server did not respond with success status code: {}", resp.status());
                }
                Err(e) => panic!("Error trying to reach server: {0}", e),
                _ => {} // Success
            };

        match read.try_next().await {
            Ok(_) => {} // Success,
            Err(e) => panic!("Error from server: {}", e),
        }

        write.close().await?;

        hist.record(start.elapsed().as_micros() as u64)?;
    }

    let elapsed: Duration = full.elapsed();

    println!("Benchmark took {0} ms", elapsed.as_millis());
    println!("p50: {0} µs", hist.value_at_quantile(0.5));
    println!("p95: {0} µs", hist.value_at_quantile(0.95));
    println!("p99: {0} µs", hist.value_at_quantile(0.99));
    println!("max: {0} µs", hist.max());

    Ok(())
}
