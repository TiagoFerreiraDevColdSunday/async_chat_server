use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

async fn async_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let (tx, _rx) = broadcast::channel(10);

    print!("Welcome to Tanjeranja.");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        match result {
                            Ok(0) => {
                                // Client disconnected
                                println!("User {} disconnected.", addr);
                                let _ = tx.send(format!("User {} has left the chat.", addr));
                                break;
                            }
                            Ok(_) => {
                                // Broadcast the received message
                                println!("Received from {}: {}", addr, line.trim());

                                let _ = tx.send(format!("{}: {}", addr, line.trim()));
                                line.clear();
                            }
                            Err(e) => {
                                eprintln!("Error reading from socket: {}", e);
                                break;
                            }
                        }
                    }
                    Ok(msg) = rx.recv() => {
                        // Send the broadcast message to the current client
                        if writer.write_all(msg.as_bytes()).await.is_err() {
                            break; // Stop if there's an error writing to the client
                        }
                    }
                }
            }
        });
    }
}

#[tokio::main]
async fn main() {
    if let Err(e) = async_server().await {
        eprintln!("Server error: {}", e);
    }
}
