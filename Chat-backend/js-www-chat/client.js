const net = require('net');
const readline = require('readline');

// Create a readline interface for user input
const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

// Connect to the server
const client = net.createConnection({ port: process.argv[2] || 3000, host: process.argv[3] || '127.0.0.1' }, () => {
  console.log('Connected to the server.');
  rl.setPrompt('You: ');
  rl.prompt();

  // Send user input to the server
  rl.on('line', (input) => {
    client.write(input);
    rl.prompt();
  });
});

// Handle incoming messages from the server
client.on('data', (data) => {
  // Convert the buffer to string
  const message = data.toString().trim();
  console.log(`\n${message}`);
  rl.prompt();
});

// Handle server disconnection
client.on('end', () => {
  console.log('Disconnected from the server.');
  rl.close();
});

// Handle errors
client.on('error', (err) => {
  console.error('Connection error:', err.message);
  rl.close();
});
