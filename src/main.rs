mod server;
use server::async_server;

#[tokio::main]
async fn main() {
    // Start the async server
    if let Err(e) = async_server().await {
        eprintln!("Server error: {}", e);
    }
}
