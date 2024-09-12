const fs = require('fs'), CryptoJS = require('crypto-js'), rl = require('readline').createInterface({ input: process.stdin, output: process.stdout });
const generateSalt = len => require('crypto').randomBytes(len).toString('hex');
const generateKey = (pwd, salt, iter) => CryptoJS.PBKDF2(pwd, salt, { keySize: 256 / 32, iterations: iter }).toString();
const generateHMAC = (data, key) => CryptoJS.HmacSHA256(data, key).toString();

rl.question('Enter the path to the JSON file: ', filePath => {
  rl.question('Enter your username: ', username => {
    rl.question('Enter your password: ', password => {
      fs.readFile(filePath, 'utf8', (err, data) => {
        if (err) return console.error("Error reading the file:", err), rl.close();
        const salt = generateSalt(16), secretKey = generateKey(username + password, salt, 10000);
        const encryptedJson = CryptoJS.AES.encrypt(data, secretKey).toString();
        console.log("Encrypted Hash (with Salt and HMAC):", `${salt}:${encryptedJson}:${generateHMAC(encryptedJson, secretKey)}`);
        rl.close();
      });
    });
  });
});
