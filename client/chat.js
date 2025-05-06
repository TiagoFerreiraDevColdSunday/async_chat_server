const chatBox = document.getElementById('chatBox');
const chatForm = document.getElementById('chatForm');
const messageInput = document.getElementById('messageInput');

function appendMessage(from, text) {
  const msg = document.createElement('div');
  msg.textContent = `${from}: ${text}`;
  chatBox.appendChild(msg);
  chatBox.scrollTop = chatBox.scrollHeight;
}

// ⛓ Reconnect using stored login
const ip = localStorage.getItem("ip");
const username = localStorage.getItem("username");
const password = localStorage.getItem("password");

if (!ip || !username || !password) {
  alert("Missing credentials. Please log in again.");
  window.location.href = "index.html";
}

const connection = window.tcpClient.connectToServer(
  ip,
  username,
  password,
  (data) => {
    appendMessage("Server", data);

    if (data.trim().includes("accepted")) {
      connection.onMessage = (msg) => {
        appendMessage("Server", msg);
      };
    }
  },
  (err) => {
    appendMessage("System", "Connection error: " + err);
  }
);

// 📨 Send messages
chatForm.addEventListener('submit', (e) => {
  e.preventDefault();
  const msg = messageInput.value.trim();
  if (!msg) return;
  connection.send(msg);
  appendMessage("You", msg);
  messageInput.value = "";
});
