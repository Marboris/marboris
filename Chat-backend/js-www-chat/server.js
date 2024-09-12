const net = require('net');

// List of connected clients
let clients = [];

// Function to log memory usage
function logMemoryUsage() {
  const memoryUsage = process.memoryUsage();
  console.log('Memory Usage:');
  console.log(`RSS: ${(memoryUsage.rss / 1024 / 1024).toFixed(2)} MB`);
  console.log(`Heap Total: ${(memoryUsage.heapTotal / 1024 / 1024).toFixed(2)} MB`);
  console.log(`Heap Used: ${(memoryUsage.heapUsed / 1024 / 1024).toFixed(2)} MB`);
  console.log(`External: ${(memoryUsage.external / 1024 / 1024).toFixed(2)} MB`);
}

// Create the server
const server = net.createServer((socket) => {
  socket.setEncoding('utf8');

  // Assign an ID to each client
  const clientId = clients.length;
  clients.push(socket);
  console.log(`Client ${clientId} connected.`);

  // Log memory usage after each client connects
  logMemoryUsage();

  // Broadcast message to all other clients
  socket.on('data', (data) => {
    const message = `Client ${clientId}: ${data}`;
    console.log(message.trim());
    clients.forEach((client, index) => {
      if (index !== clientId) {
        client.write(message);
      }
    });
  });

  // Handle client disconnection
  socket.on('end', () => {
    console.log(`Client ${clientId} disconnected.`);
    clients = clients.filter((_, index) => index !== clientId);
  });

  // Handle errors
  socket.on('error', (err) => {
    console.error(`Error with client ${clientId}:`, err.message);
  });
});

// Start listening on the specified port
const port = process.argv[2] || 3000;
server.listen(port, () => {
  console.log(`Server listening on port ${port}`);
});
