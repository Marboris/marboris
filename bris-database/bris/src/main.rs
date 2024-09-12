use aes_gcm::{Aes256Gcm, KeyInit, Nonce}; // Or `Aes128Gcm`
use aes_gcm::aead::{Aead, OsRng};
use argon2::{Argon2, PasswordHasher, PasswordVerifier, SaltString};
use rand::RngCore;

fn encrypt(data: &str, username: &str, password: &str) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    // تولید salt برای Argon2
    let salt = SaltString::generate(&mut rand::thread_rng());
    
    // تولید کلید با استفاده از Argon2
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();
    let key = password_hash.hash.unwrap();
    
    // تولید nonce تصادفی برای AES-GCM
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    // ایجاد cipher با استفاده از کلید
    let cipher = Aes256Gcm::new_from_slice(&key).unwrap();
    
    // رمزگذاری داده‌ها
    let ciphertext = cipher.encrypt(nonce, data.as_ref()).unwrap();
    
    (ciphertext, nonce_bytes.to_vec(), salt.as_bytes().to_vec())
}

fn decrypt(ciphertext: &[u8], nonce: &[u8], username: &str, password: &str, salt: &[u8]) -> String {
    // بازیابی کلید با استفاده از Argon2
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &SaltString::b64_encode(salt).unwrap()).unwrap();
    let key = password_hash.hash.unwrap();
    
    // ایجاد cipher با استفاده از کلید
    let cipher = Aes256Gcm::new_from_slice(&key).unwrap();
    
    // رمزگشایی داده‌ها
    let decrypted_data = cipher.decrypt(Nonce::from_slice(nonce), ciphertext).unwrap();
    
    String::from_utf8(decrypted_data).unwrap()
}

fn main() {
    let username = "user123";
    let password = "securepassword";
    let data = "Hello, World!";
    
    // رمزگذاری
    let (ciphertext, nonce, salt) = encrypt(data, username, password);
    
    // رمزگشایی
    let decrypted_data = decrypt(&ciphertext, &nonce, username, password, &salt);
    
    println!("Original Data: {}", data);
    println!("Decrypted Data: {}", decrypted_data);
}
