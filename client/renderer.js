const form = document.getElementById('loginForm');
const status = document.getElementById('status');

form.addEventListener('submit', async (e) => {
    
    e.preventDefault();

    const ip = document.getElementById('ip').value;
    const username = document.getElementById('username').value;
    const password = document.getElementById('password').value;

    const connection = window.tcpClient.connectToServer(
        ip, username, password,
        (data) => {

            status.textContent = `Server ${data}`;

            console.log('Assigned onMessage:', connection.onMessage); // connection.onMessage is undefined

            if (data.trim().includes('accepted')) {
                status.textContent = 'Connected to server';
                window.connection = connection;
                
                localStorage.setItem("connected", "true");

                localStorage.setItem("ip", ip);
                localStorage.setItem("username", username);
                localStorage.setItem("password", password);
                window.location.href = "chat.html";
            }
        },
        (error) => {
            console.error('Connection error:', error);
            status.textContent = 'Connection error: ' + error.message;
        }
    )
})