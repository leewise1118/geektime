use std::{fs, io::Read, path::Path};

use crate::{cli::text::TextSignFormat, process_genpasswd, utils::get_reader};
use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

pub struct Blake3 {
    key: [u8; 32],
}

struct Ed25519Signer {
    key: SigningKey,
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
    };
    Ok(verified)
}

pub fn process_text_generate(format: TextSignFormat) -> Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

pub fn process_text_encrypt(input: &str, key: &str) -> Result<String> {
    todo!()
}
pub fn process_text_decrypt(input: &str, key: &str) -> Result<bool> {
    todo!()
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
