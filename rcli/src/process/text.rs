use std::{fs, io::Read, path::Path};

use crate::{cli::text::TextSignFormat, process_genpasswd, utils::get_reader};
use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit},
    ChaCha20Poly1305, ChaChaPoly1305, Key, Nonce,
};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

use rand::rngs::OsRng;

pub struct Blake3 {
    key: [u8; 32],
}

struct Ed25519Signer {
    key: SigningKey,
}

pub struct Chacha20poly1305 {
    key: Key,
}
struct Ed25519Verifier {
    key: VerifyingKey,
}
pub trait TextSign {
    /// Sign the data from the reader and return the signature
    // &[u8] implements Read, so we can test with &[u8] instead of File
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}
trait TextVerify {
    fn verify(&self, reader: impl Read, signature: &[u8]) -> Result<bool>;
}

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> Result<String> {
    let mut reader = get_reader(input)?;

    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::ChaCha20Poly1305 => {
            return Err(anyhow::anyhow!("Use function process_text_encrypt"));
        }
    };

    let signed = URL_SAFE_NO_PAD.encode(signed);
    Ok(signed)
}

pub fn process_text_verify(
    input: &str,
    key: &str,
    signature: &str,
    format: TextSignFormat,
) -> Result<bool> {
    let mut reader = get_reader(input)?;
    let signature = URL_SAFE_NO_PAD.decode(signature)?;

    let verified = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verify(&mut reader, &signature)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verify(&mut reader, &signature)?
        }
        TextSignFormat::ChaCha20Poly1305 => {
            return Err(anyhow::anyhow!("Use function process_text_dncrypt"));
        }
    };
    Ok(verified)
}

pub fn process_text_generate(format: TextSignFormat) -> Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
        TextSignFormat::ChaCha20Poly1305 => Chacha20poly1305::generate(),
    }
}

pub fn process_text_encrypt(input: &str, key: &str) -> Result<String> {
    let mut reader = get_reader(input)?;

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let key = Chacha20poly1305::load(key)?;
    let cipher = ChaCha20Poly1305::new(&key.key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let encrypted_text = cipher.encrypt(&nonce, buf.as_ref()).unwrap();

    let encrypted_text = URL_SAFE_NO_PAD.encode(encrypted_text);
    Ok(encrypted_text)
}
pub fn process_text_decrypt(input: &str, key: &str, encrypted_text: &str) -> Result<bool> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encrypted_text = URL_SAFE_NO_PAD.decode(encrypted_text)?;

    let key = Chacha20poly1305::load(key)?;
    let cipher = ChaCha20Poly1305::new(&key.key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let plaintext = cipher.decrypt(&nonce, encrypted_text.as_ref()).unwrap();

    println!("{:?}", String::from_utf8(plaintext.clone()));

    Ok(input.as_bytes().to_vec() == plaintext)
}
impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}
impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf: Vec<u8> = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signature = self.key.sign(&buf).to_bytes().to_vec();
        Ok(signature)
    }
}
impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl Read, signature: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        let binding = blake3::keyed_hash(&self.key, &buf);
        let hash = binding.as_bytes();
        println!("hash = {:?}", String::from_utf8(hash.to_vec()));

        Ok(hash == signature)
    }
}
impl TextVerify for Ed25519Verifier {
    fn verify(&self, mut reader: impl Read, signature: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        println!("verify signature: {:?}", signature);
        let sig = Signature::from_bytes(signature.try_into()?);
        let ret = self.key.verify(&buf, &sig).is_ok();
        println!("ret = {:?}", ret);
        Ok(ret)
    }
}
impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into().unwrap();
        let signer = Blake3::new(key);
        Ok(signer)
    }
}

impl Chacha20poly1305 {
    pub fn new(key: [u8; 32]) -> Self {
        let key = Key::from_slice(&key);
        Self { key: *key }
    }
    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = Key::from_slice(key);
        Ok(Self { key: *key })
    }
}
impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32].try_into()?;
        let key = SigningKey::from_bytes(key);
        let signer = Ed25519Signer::new(key);
        Ok(signer)
    }
}
impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32].try_into()?;
        let key = VerifyingKey::from_bytes(key)?;
        let verifier = Ed25519Verifier::new(key);
        Ok(verifier)
    }
}

pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized;
}
impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}
impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}
impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}
impl KeyLoader for Chacha20poly1305 {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}
pub trait KeyGenerator {
    fn generate() -> Result<Vec<Vec<u8>>>;
}

impl KeyGenerator for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let key = process_genpasswd(32, true, true, true, true)?;
        let key = key.into_bytes();
        Ok(vec![key])
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);
        let pk = signing_key.verifying_key().to_bytes().to_vec();
        let sk = signing_key.to_bytes().to_vec();
        Ok(vec![sk, pk])
    }
}

impl KeyGenerator for Chacha20poly1305 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let key = ChaCha20Poly1305::generate_key(&mut chacha20poly1305::aead::OsRng);
        let key = key.to_vec();
        Ok(vec![key])
    }
}

impl KeyGenerator for Ed25519Verifier {
    fn generate() -> Result<Vec<Vec<u8>>> {
        todo!()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3_sign_verify() {
        // 密匙
        let signer = Blake3::load("../fixtures/blake3.txt").unwrap();

        // 哈希数据
        let data = b"hello world!";

        // 求出来的哈希值
        let sig = signer.sign(&mut &data[..]).unwrap();

        println!("sig = {:?}", URL_SAFE_NO_PAD.encode(&sig));

        assert!(signer.verify(&mut &data[..], &sig).unwrap());
    }

    #[test]
    fn test_ed25519_sign_verify() {
        let sk = Ed25519Signer::load("../fixtures/ed25519.sk").unwrap();
        let pk = Ed25519Verifier::load("../fixtures/ed25519.pk").unwrap();

        let data = b"hello!";
        let sig = sk.sign(&mut &data[..]).unwrap();
        println!("sig = {:?}", URL_SAFE_NO_PAD.encode(&sig));
        assert!(pk.verify(&mut &data[..], &sig).unwrap());
    }

    #[test]
    fn test_chacha20poly1305_encrypt_dncrypt() {
        let key = ChaCha20Poly1305::generate_key(&mut OsRng);
        let cipher = ChaCha20Poly1305::new(&key);
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
        let ciphertext = cipher
            .encrypt(&nonce, b"plaintext message".as_ref())
            .unwrap();
        let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref()).unwrap();
        assert_eq!(&plaintext, b"plaintext message");
    }

    #[test]
    fn test_process_text_sign_and_verify_blake3() {
        let text = "../fixtures/text.txt";
        let signed =
            process_text_sign(text, "../fixtures/blake3.txt", TextSignFormat::Blake3).unwrap();
        println!("signed: {:?}", signed);
        let verified = process_text_verify(
            text,
            "../fixtures/blake3.txt",
            &signed,
            TextSignFormat::Blake3,
        );

        assert!(verified.unwrap());
    }

    #[test]
    fn test_process_text_sign_and_verify_ed25519() {
        let text = "../fixtures/text.txt";
        let signed =
            process_text_sign(text, "../fixtures/ed25519.sk", TextSignFormat::Ed25519).unwrap();
        let verified = process_text_verify(
            text,
            "../fixtures/ed25519.pk",
            &signed,
            TextSignFormat::Ed25519,
        );

        assert!(verified.unwrap());
    }
}
