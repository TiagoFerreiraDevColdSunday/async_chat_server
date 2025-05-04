use async_chat_server::client::{async_client, send_message_to_server};
use eframe::{App, egui};
use std::sync::Arc;
use tokio::io::BufReader;
use tokio::sync::Mutex;
use tokio::sync::mpsc;

pub struct GUI {
    ip: String,
    password: String,
    username: String,
    state: AppState,
    messages: Vec<String>,            // Stores chat messages
    tx: Option<mpsc::Sender<String>>, // Sender for outgoing messages
    input: String,                    // Stores the current input text
    reader: Option<BufReader<tokio::net::tcp::OwnedReadHalf>>, // Reader for incoming messages
    writer: Option<Arc<Mutex<tokio::net::tcp::OwnedWriteHalf>>>, // Writer for outgoing messages
    rx: Option<mpsc::Receiver<String>>, // Receiver for messages from the client
}

#[derive(Clone)]
enum AppState {
    Login,
    Chatroom,
}

impl Default for GUI {
    fn default() -> Self {
        Self {
            ip: String::new(),
            password: String::new(),
            username: String::new(),
            state: AppState::Login,
            messages: Vec::new(),
            tx: None,
            input: String::new(), // Initialize input as an empty string
            reader: None,
            writer: None,
            rx: None,
        }
    }
}

impl GUI {
    fn ui(&mut self, ui: &mut egui::Ui) {
        match self.state {
            AppState::Login => self.ui_login(ui),
            AppState::Chatroom => self.ui_chatroom(ui),
        }
    }

    fn ui_login(&mut self, ui: &mut egui::Ui) {
        ui.label("Welcome to Tanjeranja!");
        ui.horizontal(|ui| {
            ui.label("IP Address:");
            ui.text_edit_singleline(&mut self.ip);
        });
        ui.horizontal(|ui| {
            ui.label("Password:");
            ui.text_edit_singleline(&mut self.password);
        });
        ui.horizontal(|ui| {
            ui.label("Username:");
            ui.text_edit_singleline(&mut self.username);
        });
    
        // Button
        if ui.button("Connect").clicked() {
            println!("Connect button clicked!");
            let ip = self.ip.clone();
            let password = self.password.clone();
            let username = self.username.clone();
    
            // Wrap `self.state` in an Arc<Mutex<T>> to share it safely
            let state: Arc<Mutex<AppState>> = Arc::new(Mutex::new(self.state.clone()));
    
            tokio::spawn({
                let state = Arc::clone(&state);
                async move {
                    if let Err(e) = async_client(ip, username, password).await {
                        eprintln!("Error: {}", e);
                        let mut state = state.lock().await;
                        *state = AppState::Login; // Update the state safely
                    } else {
                        println!("Connection successful!");
                        let mut state = state.lock().await;
                        *state = AppState::Chatroom; // Update the state safely
                    }
                }
            });
        }
    }

    fn ui_chatroom(&mut self, ui: &mut egui::Ui) {
        ui.label("Chatroom");

        // Display messages
        egui::ScrollArea::vertical().show(ui, |ui| {
            for message in &self.messages {
                ui.label(message);
            }
        });

        // Input box for sending messages
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.input);
            if ui.button("Send").clicked() {
                if let Some(tx) = &self.tx {
                    let _ = tx.try_send(self.input.clone());
                }

                // Call send_message_to_server
                if let (Some(mut reader), Some(writer), Some(rx)) =
                    (self.reader.take(), self.writer.take(), self.rx.take())
                {
                    let input = self.input.clone();
                    tokio::spawn(async move {
                        if let Err(e) = send_message_to_server(reader, writer, rx).await {
                            eprintln!("Error sending message: {}", e);
                        }
                    });
                }

                self.messages.push(format!("You: {}", self.input));
                self.input.clear(); // Clear the input field after sending
            }
        });
    }
}

impl App for GUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ui);
        });
    }
}

pub fn main() -> Result<(), eframe::Error> {
    // Create a Tokio runtime
    let runtime = tokio::runtime::Runtime::new().unwrap();

    // Run the eframe application within the Tokio runtime
    runtime.block_on(async {
        let options = eframe::NativeOptions::default();
        eframe::run_native(
            "Async Chat Client",
            options,
            Box::new(|_cc| Box::new(GUI::default())),
        )
    })
}
