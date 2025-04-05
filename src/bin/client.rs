extern crate async_chat_server;
use async_chat_server::client_server_utils::get_machine_ip;

use std::io;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::sync::mpsc;

async fn async_client() -> std::io::Result<()> {
    // Get the machine's IP address
    let ip_address = get_machine_ip().unwrap_or_else(|| "127.0.0.1".to_string());

    // Connects to the server and waits for it to accept the connection
    let stream = TcpStream::connect(format!("{}:8080", ip_address)).await?;

    let (reader, writer) = stream.into_split();
    let reader = BufReader::new(reader);
    let writer = Arc::new(Mutex::new(writer));

    let (tx, mut rx) = mpsc::channel::<String>(10);

    let mut username = String::new();
    let prefix = "Username ";

    println!("Welcome to Tanjeranja! Please enter your username:");
    io::stdin().read_line(&mut username).unwrap();
    username = prefix.to_string() + username.trim();

    writer
        .lock()
        .await
        .write_all(format!("{}\n", username).as_bytes())
        .await?;

    tokio::spawn(async move {
        let stdin = io::stdin();
        let mut input = String::new();

        loop {
            input.clear();
            if stdin.read_line(&mut input).is_ok() {
                if tx.send(input.clone()).await.is_err() {
                    break;
                }
            }
        }
    });

    let mut server_reader = reader.lines();
    tokio::spawn(async move {
        while let Ok(Some(line)) = server_reader.next_line().await {
            println!("{}", line);
        }
        println!("Disconnected from the server.");
    });

    while let Some(msg) = rx.recv().await {
        let mut writer = writer.lock().await;

        if !msg.ends_with('\n') {
            writer.write_all(format!("{}\n", msg).as_bytes()).await?;
        } else {
            writer.write_all(msg.as_bytes()).await?;
        }
    }

    return Ok(());
}

#[tokio::main]
async fn main() {
    if let Err(e) = async_client().await {
        eprintln!("Client error: {}", e);
    }
}
