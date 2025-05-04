use std::io;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::sync::mpsc;

pub async fn async_client(
    mut server_ip: String,
    mut username: String,
    password: String,
) -> std::io::Result<(
    BufReader<tokio::net::tcp::OwnedReadHalf>, // Reader for incoming messages
    Arc<Mutex<tokio::net::tcp::OwnedWriteHalf>>, // Writer for outgoing messages
    mpsc::Receiver<String>,                    // Receiver for messages from the client
)> {
    server_ip = server_ip.trim().to_string(); // Trim newline and whitespace

    // Connect to the server
    let stream = TcpStream::connect(format!("{}:8080", server_ip)).await?;

    println!("Connected to the server at {}:8080", server_ip);
    let (reader, writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let writer = Arc::new(Mutex::new(writer));

    let (tx, rx) = mpsc::channel::<String>(10);

    let prefix = "Username ";

    println!("Welcome to Tanjeranja!");

    username = prefix.to_string() + username.trim();

    // Send username
    writer
        .lock()
        .await
        .write_all(format!("{}\n", username).as_bytes())
        .await?;

    print!("User name sent: {}\n", username);

    // Sent password
    writer
        .lock()
        .await
        .write_all(format!("{}\n", password).as_bytes())
        .await?;

    print!("Password sent: {}\n", password);

    // Read the server's response
    let mut response = String::new();
    reader.read_line(&mut response).await?;

    print!("Server response: {}\n", response.trim());

    // Check the server's response
    if response.trim() == "Password accepted." {
        println!("Password accepted by the server.");
    } else {
        println!("Server response: {}", response.trim());
        return Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "Invalid password or server error.",
        ));
    }

    // Return the reader, writer, and rx
    Ok((reader, writer, rx))
}

pub async fn send_message_to_server(
    reader: BufReader<tokio::net::tcp::OwnedReadHalf>, // Take ownership of the reader
    writer: Arc<Mutex<tokio::net::tcp::OwnedWriteHalf>>,
    mut rx: mpsc::Receiver<String>, // Take ownership of the receiver
) -> std::io::Result<()> {
    // Spawn a task to read messages from the server
    tokio::spawn(async move {
        let mut server_reader = reader.lines();
        while let Ok(Some(line)) = server_reader.next_line().await {
            println!("{}", line);
        }
        println!("Disconnected from the server.");
    });

    // Process messages from the receiver and send them to the server
    while let Some(msg) = rx.recv().await {
        let mut writer = writer.lock().await;

        if !msg.ends_with('\n') {
            writer.write_all(format!("{}\n", msg).as_bytes()).await?;
        } else {
            writer.write_all(msg.as_bytes()).await?;
        }
    }

    Ok(())
}
