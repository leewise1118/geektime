use std::{path::PathBuf, str::FromStr};

use super::{verify_file, verify_path};
use anyhow::Result;
use clap::{command, Parser};
#[derive(Debug, Parser)]
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

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(long, default_value="blake3", value_parser=parse_text_sign_format)]
    pub format: TextSignFormat,

    #[arg(short, long,value_parser = verify_path)]
    pub output: PathBuf,
}

#[derive(Debug, Parser)]
pub struct TextEncryptOpts {
    /// 需要加密的内容，可是文件，也可以是标准输入流
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short,long, value_parser = verify_file)]
    pub key: String,
}

#[derive(Debug, Parser)]
pub struct TextDecryptOpts {
    /// 需要解密的内容，可是文件，也可以是标准输入流
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short,long, value_parser = verify_file)]
    pub key: String,
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
