use openssl::symm::{Cipher, Crypter, Mode};
use openssl::hash::MessageDigest;
use openssl::hmac::Hmac;
use sha2::Sha256;
use std::error::Error;
use rand::Rng;

fn derive_key_and_iv(username: &str, password: &str) -> (Vec<u8>, Vec<u8>) {
    // Combine username and password and hash them to create a key
    let mut hasher = Sha256::new();
    hasher.update(username.as_bytes());
    hasher.update(password.as_bytes());
    let key = hasher.finalize().to_vec();

    // Generate a random IV
    let mut iv = vec![0; 16];
    rand::thread_rng().fill(&mut iv[..]);

    (key, iv)
}

fn encrypt(username: &str, password: &str, data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let (key, iv) = derive_key_and_iv(username, password);

    let cipher = Cipher::aes_256_cbc();
    let mut encrypter = Crypter::new(cipher, Mode::Encrypt, &key, Some(&iv))?;
    
    let block_size = cipher.block_size();
    let mut encrypted_data = vec![0; data.len() + block_size];
    let count = encrypter.update(data, &mut encrypted_data)?;
    let rest = encrypter.finalize(&mut encrypted_data[count..])?;
    encrypted_data.truncate(count + rest);

    // Calculate HMAC for the encrypted data
    let mut hmac = Hmac::new(MessageDigest::sha256(), &key);
    hmac.update(&encrypted_data);
    let tag = hmac.finish().into_bytes();

    // Concatenate IV, encrypted data, and HMAC tag
    let mut result = iv;
    result.extend(encrypted_data);
    result.extend(tag);

    Ok(result)
}

fn decrypt(username: &str, password: &str, encrypted_data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let (key, iv) = derive_key_and_iv(username, password);

    // Split encrypted data into IV, encrypted part, and HMAC tag
    let (iv, rest) = encrypted_data.split_at(16);
    let (encrypted_data, tag) = rest.split_at(rest.len() - 32);

    // Verify HMAC
    let mut hmac = Hmac::new(MessageDigest::sha256(), &key);
    hmac.update(encrypted_data);
    let expected_tag = hmac.finish().into_bytes();

    if tag != expected_tag.as_slice() {
        return Err("Invalid HMAC tag".into());
    }

    // Decrypt data
    let cipher = Cipher::aes_256_cbc();
    let mut decrypter = Crypter::new(cipher, Mode::Decrypt, &key, Some(iv))?;
    
    let block_size = cipher.block_size();
    let mut decrypted_data = vec![0; encrypted_data.len() + block_size];
    let count = decrypter.update(encrypted_data, &mut decrypted_data)?;
    let rest = decrypter.finalize(&mut decrypted_data[count..])?;
    decrypted_data.truncate(count + rest);

    Ok(decrypted_data)
}

fn main() -> Result<(), Box<dyn Error>> {
    let username = "user";
    let password = "password";
    let data = b"Sensitive data";

    let encrypted_data = encrypt(username, password, data)?;
    println!("Encrypted Data: {:?}", hex::encode(&encrypted_data));

    match decrypt(username, password, &encrypted_data) {
        Ok(decrypted_data) => println!("Decrypted Data: {:?}", String::from_utf8(decrypted_data)?),
        Err(e) => println!("Failed to decrypt data: {}", e),
    }

    Ok(())
}
