const net = require('net');

// Number of clients to simulate
const clientCount = 10000;
const clients = [];

// Connect multiple clients to the server
for (let i = 0; i < clientCount; i++) {
  const client = net.createConnection({ port: 3000, host: '127.0.0.1' }, () => {
    console.log(`Client ${i} connected.`);
    clients.push(client);

    // Send a message from each client
    client.write(`Hello from client ${i}`);
  });

  // Handle data from server
  client.on('data', (data) => {
    console.log(`Server to client ${i}: ${data}`);
  });

  // Handle client disconnection
  client.on('end', () => {
    console.log(`Client ${i} disconnected.`);
  });

  // Handle client errors
  client.on('error', (err) => {
    console.error(`Error in client ${i}: ${err.message}`);
  });
}
