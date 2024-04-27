use std::str::FromStr;

use super::verify_file;
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
    #[arg(short, long)]
    pub output: String,

    #[arg( long, default_value="blake3", value_parser=parse_text_sign_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
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
            _ => Err(anyhow::anyhow!("Invalid text sign format")),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(f: TextSignFormat) -> &'static str {
        match f {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}
