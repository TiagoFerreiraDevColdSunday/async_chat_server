const { contextBridge } = require('electron');
const { stat } = require('fs');
const net = require('net');


contextBridge.exposeInMainWorld('tcpClient', {
    connectToServer: (ip, username, password, onData, onError) => {

        console.log(`Attempting to connect to server at ${ip}:8080 with username: ${username}`);

        const client = new net.Socket()

        const state = {
            onMessage: null,
        }

        client.connect(8080, ip.trim(), () => {
            console.log('User was able to connect to the server');

            client.write(`Username ${username.trim()}\n`);
            client.write(`${password.trim()}\n`);

            text = client.read(1000)
        });

        client.on('data', (data) => {
            const text = data.toString();
            onData(data.toString());

            if (state.onMessage) state.onMessage(text);
        });

        client.on('error', (err) => {
            onError(err);
        });

        return {
            send: (msg) => {
                if (!msg.endsWith('\n')) msg += '\n';
                client.write(msg);
            },
            close: () => client.destroy(),

            set onMessage(fn) {
                state.onMessage = fn;
            }
            
        }
    }
})