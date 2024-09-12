const CryptoJS = require('crypto-js'), rl = require('readline').createInterface({ input: process.stdin, output: process.stdout });
const generateKey = (pwd, salt, iter) => CryptoJS.PBKDF2(pwd, salt, { keySize: 256 / 32, iterations: iter }).toString();
const generateHMAC = (data, key) => CryptoJS.HmacSHA256(data, key).toString();

rl.question('Enter the encrypted hash (with Salt and HMAC): ', input => {
  rl.question('Enter your username: ', username => {
    rl.question('Enter your password: ', password => {
      let [salt, encryptedHash, providedHMAC] = input.split(':');
      const secretKey = generateKey(username + password, salt, 10000);
      const validHMAC = generateHMAC(encryptedHash, secretKey);

      if (validHMAC === providedHMAC) {
        try {
          let decryptedJson = CryptoJS.AES.decrypt(encryptedHash, secretKey).toString(CryptoJS.enc.Utf8);
          decryptedJson ? console.log("Decrypted JSON Content:", decryptedJson) : console.log("Invalid username or password. Decryption failed.");
        } catch (error) {
          console.error("Error during decryption:", error.message);
        }
      } else console.log("HMAC validation failed. Data integrity compromised.");
      rl.close();
    });
  });
});
