// HTML Objects
const chatBox = document.getElementById('chatBox');
const chatForm = document.getElementById('chatForm');
const messageInput = document.getElementById('messageInput');

// User credentials
const ip = localStorage.getItem("ip");
const username = localStorage.getItem("username");
const password = localStorage.getItem("password");

// User shall not receive from the server words
const should_not_receive_from_server_words = [username, "left", "joined"]

function appendMessage(from, text) {
  const msg = document.createElement('div');
  msg.textContent = `${from}: ${text}`;
  chatBox.appendChild(msg);
  chatBox.scrollTop = chatBox.scrollHeight;
}


const connection = window.tcpClient.connectToServer(
  ip,
  username,
  password,
  (data) => {

    // Check if the message contains any of the words that should not be received
    const should_receive_condition = should_not_receive_from_server_words.some((word) =>
      data.trim().includes(word)
    );

    if (!should_receive_condition) {
      appendMessage("", data);
    }
    
    if (data.trim().includes("accepted")) {
      connection.onMessage = (msg) => {
        appendMessage("Server", msg);
      };
    }
  },
  (err) => {
    appendMessage("System", "Connection error: " + err);
    window.location.href = "index.html";
  }
);

// Send messages
chatForm.addEventListener('submit', (e) => {
  e.preventDefault();
  const msg = messageInput.value.trim();
  if (!msg) return;
  connection.send(msg);
  appendMessage("You", msg);
  messageInput.value = "";
});