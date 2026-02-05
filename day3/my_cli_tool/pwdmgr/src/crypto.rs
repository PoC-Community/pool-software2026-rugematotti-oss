use aes_gcm::{Aes256Gcm, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use rand::RngCore;
use sha2::{Sha256, Digest};

pub struct CryptoManager {
    cipher: Aes256Gcm,
}

impl CryptoManager {
    pub fn new(master_password: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(master_password.as_bytes());
        let key = hasher.finalize();

        let cipher = Aes256Gcm::new_from_slice(&key).unwrap();
        Self { cipher }
    }

    pub fn encrypt(&self, plaintext: &str) -> Vec<u8> {
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = self
            .cipher
            .encrypt(nonce, plaintext.as_bytes())
            .expect("encryption failed");

        [nonce_bytes.to_vec(), ciphertext].concat()
    }
}
