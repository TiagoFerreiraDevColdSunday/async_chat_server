use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::{Mutex, broadcast};

type Clients = Arc<Mutex<HashMap<String, tokio::sync::mpsc::Sender<String>>>>;

async fn async_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    let (tx, _rx) = broadcast::channel(10);

    loop {
        let (mut socket, _addr) = listener.accept().await?;
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        let clients = Arc::clone(&clients);

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();
            let mut username = String::new();

            // Wait for the client to send their nickname
            if reader.read_line(&mut line).await.unwrap() > 0 {
                if line.starts_with("Username ") {
                    username = line[9..].trim().to_string();
                } else {
                    writer
                        .write_all(b"Invalid username format. Use: Username your_name\n")
                        .await
                        .unwrap();
                    return;
                }
            }

            // Store the client
            let (msg_tx, mut _msg_rx) = tokio::sync::mpsc::channel::<String>(10);
            clients.lock().await.insert(username.clone(), msg_tx);

            // Notify everyone about the new user
            tx.send(format!("{} has joined the chat!", username))
                .unwrap();

            // Start listening for messages from the client
            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        match result {
                            Ok(0) => {
                                // Client disconnected
                                println!("User {} disconnected.", username);
                                let _ = tx.send(format!("User {} has left the chat.", username));
                                break;
                            }
                            Ok(_) => {
                                // Broadcast the received message
                                println!("Received from {}: {}", username, line.trim());

                                let _ = tx.send(format!("{}: {}", username, line.trim()));
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
