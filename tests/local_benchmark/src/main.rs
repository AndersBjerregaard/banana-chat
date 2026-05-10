use std::{env, thread::sleep, time::{Duration, Instant}, u64};

use hdrhistogram::Histogram;
use reqwest::Client;

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

    let mut hist: Histogram<u64> = Histogram::<u64>::new_with_bounds(1, 60 * 60 * 100, 2).unwrap();

    for _ in 0..iterations {
        let start = Instant::now();

        // Do some magic
        let dur: Duration = Duration::from_millis(16);

        sleep(dur);

        hist.record(start.elapsed().as_micros() as u64)?;
    }

    let elapsed: Duration = full.elapsed();

    println!("Benchmark took {0} ms", elapsed.as_millis());
    println!("p50: {0}", hist.value_at_quantile(0.5));
    println!("p95: {0}", hist.value_at_quantile(0.95));
    println!("p99: {0}", hist.value_at_quantile(0.99));
    println!("max: {0}", hist.max());

    Ok(())
}
