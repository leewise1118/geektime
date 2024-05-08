use std::{fs, path::PathBuf, str::FromStr};

use crate::{
    process::text::{
        process_text_decrypt, process_text_encrypt, process_text_generate, process_text_sign,
        process_text_verify,
    },
    CmdExecutor,
};

use super::{verify_file, verify_path};
use anyhow::Result;
use clap::{command, Parser};
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum TextSubCommand {
    #[command(about = "Sign a message with a private/shared key")]
    Sign(TextSignOpts),

    #[command(about = "Verify a signed message with a public/shared key")]
    Verify(TextVerifyOpts),

    #[command(about = "Generate a new key")]
    Generate(TextKeyGenerateOpts),

    #[command(about = "Text encryption")]
    Encrypt(TextEncryptOpts),

    #[command(about = "Text decryption")]
    Dncrypt(TextDecryptOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verify_file)]
    pub key: String,

    #[arg( long, default_value="blake3", value_parser=parse_text_sign_format)]
    pub format: TextSignFormat,
}
impl CmdExecutor for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let signed = process_text_sign(&self.input, &self.key, self.format)?;
        println!("{:?}", signed);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verify_file )]
    pub key: String,

    #[arg(short, long)]
    pub sig: String,

    #[arg( long, default_value="blake3", value_parser=parse_text_sign_format)]
    pub format: TextSignFormat,
}
impl CmdExecutor for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let verified = process_text_verify(&self.input, &self.key, &self.sig, self.format);
        println!("{:?}", verified);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(long, default_value="blake3", value_parser=parse_text_sign_format)]
    pub format: TextSignFormat,

    #[arg(short, long,value_parser = verify_path)]
    pub output: PathBuf,
}

impl CmdExecutor for TextKeyGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = process_text_generate(self.format)?;
        match self.format {
            TextSignFormat::Blake3 => {
                let name = self.output.join("blake3.txt");
                fs::write(name, &key[0])?;
            }
            TextSignFormat::Ed25519 => {
                let sk_name = self.output.join("ed25519.sk");
                let pk_name = self.output.join("ed25519.pk");
                fs::write(sk_name, &key[0])?;
                fs::write(pk_name, &key[1])?;
            }
            TextSignFormat::ChaCha20Poly1305 => {
                let name = self.output.join("chacha20poly1305.txt");
                println!("key = {:?}, length = {:?}", &key[0], key[0].len());
                fs::write(name, &key[0])?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct TextEncryptOpts {
    /// 需要加密的内容，可是文件，也可以是标准输入流
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short,long, value_parser = verify_file)]
    pub key: String,
}
impl CmdExecutor for TextEncryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let encrypted_text = process_text_encrypt(&self.input, &self.key)?;
        println!("{:?}", encrypted_text);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct TextDecryptOpts {
    /// 需要解密的内容，可是文件，也可以是标准输入流
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short,long, value_parser = verify_file)]
    pub key: String,
}

impl CmdExecutor for TextDecryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let plaintext = process_text_decrypt(&self.input, &self.key)?;
        println!("{:?}", plaintext);
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
    ChaCha20Poly1305,
}

fn parse_text_sign_format(s: &str) -> Result<TextSignFormat> {
    s.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            "chacha20poly1305" => Ok(TextSignFormat::ChaCha20Poly1305),
            _ => Err(anyhow::anyhow!("Invalid text sign format")),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(f: TextSignFormat) -> &'static str {
        match f {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
            TextSignFormat::ChaCha20Poly1305 => "chacha20poly1305",
        }
    }
}
