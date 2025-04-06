extern crate async_chat_server;
use async_chat_server::client_server_utils::{decrypt_password_rsa, get_ipv4};

use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::{Mutex, broadcast};

type Clients = Arc<Mutex<HashMap<String, tokio::sync::mpsc::Sender<String>>>>;

async fn async_server() -> std::io::Result<()> {
    //Call get_machine_ip() to get the IP address of the machine
    let ip_address = get_ipv4().unwrap_or_else(|| "127.0.0.1".to_string());

    // Bind the server to the IP address and port 8080
    let listener = TcpListener::bind(format!("{}:8080", ip_address)).await?;

    // Create a shared state for clients
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    // Creates a broadcast channel for sending messages to all clients
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

            line.clear();

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

            line.clear();

            writer
                .write_all(b"Password for this server:\n")
                .await
                .unwrap();

            // Wait for the client to send their password
            if reader.read_line(&mut line).await.unwrap() > 0 {
                match decrypt_password_rsa(line.trim()) {
                    Ok(true) => {
                        writer
                            .write_all(b"Password accepted. Please enter your username:\n")
                            .await
                            .unwrap();
                    }
                    Ok(false) => {
                        writer
                            .write_all(b"Invalid password. Disconnecting...\n")
                            .await
                            .unwrap();
                        return;
                    }
                    Err(e) => {
                        eprintln!("Error decrypting password: {}", e);
                        writer
                            .write_all(b"An error occurred. Disconnecting...\n")
                            .await
                            .unwrap();
                        return;
                    }
                }
            }

            // Store the client
            let (msg_tx, mut _msg_rx) = tokio::sync::mpsc::channel::<String>(10);
            clients.lock().await.insert(username.clone(), msg_tx);

            // Notify everyone about the new user
            tx.send(format!("{} has joined the chat!\n", username))
                .unwrap();

            // Start listening for messages from the client
            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        match result {
                            Ok(0) => {
                                // Client disconnected
                                println!("User {} disconnected.", username);
                                let _ = tx.send(format!("User {} has left the chat.\n", username));
                                break;
                            }
                            Ok(_) => {
                                // Broadcast the received message
                                println!("Received from {}: {}", username, line.trim());

                                let _ = tx.send(format!("{}: {}\n", username, line.trim())).unwrap();
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
