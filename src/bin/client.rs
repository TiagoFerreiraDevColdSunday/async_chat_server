extern crate async_chat_server;

use std::io;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::sync::mpsc;

async fn async_client() -> std::io::Result<()> {
    let mut server_ip = String::new();
    println!("Ip address:");
    io::stdin().read_line(&mut server_ip).unwrap();
    server_ip = server_ip.trim().to_string(); // Trim newline and whitespace

    let stream = TcpStream::connect(format!("{}:8080", server_ip)).await?;

    print!("Connected to the server at {}:8080\n", server_ip);
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
