use tokio::time::{ sleep, Duration };
use reqwest;

#[tokio::main]
async fn main() {
    // A list of URLs to fetch concurrently
    let urls = vec![
        "https://www.example.com",
        "https://www.rust-lang.org",
        "https://www.github.com",
        // "https://localhost:8080",
        "https://youtube.com"
    ];

    // Create a vector to store the spawned task handles
    let mut handles = Vec::new();

    // Spawn asynchronous tasks to fetch each URL concurrently
    for url in urls {
        let handle = tokio::spawn(fetch_url(url));
        handles.push(handle);
    }

    // Wait for all spawned tasks to complete
    tokio::join!(
        async {
            // Concurrently fetch URLs
            futures::future::join_all(handles).await;
        },
        async {
            // Perform other asynchronous tasks concurrently
            println!("Doing other asynchronous tasks concurrently...");
            sleep(Duration::from_secs(3)).await;
            println!("Other tasks completed!");
        }
    );
}

async fn fetch_url(url: &str) {
    // Asynchronously fetch a URL using reqwest
    let response = reqwest::get(url).await;

    // Print the result or an error message
    match response {
        Ok(resp) => println!("Successfully fetched {}: {:?}", url, resp.status()),
        Err(err) => eprintln!("Failed to fetch {}: {}", url, err),
    }
}
