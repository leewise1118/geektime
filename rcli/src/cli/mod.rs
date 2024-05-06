pub mod base64;
pub mod csv;
pub mod genpw;
pub mod http;
pub mod text;
use std::path::{Path, PathBuf};

use base64::Base64Subcommand;
use clap::Parser;
use csv::CsvOpts;
use genpw::GenPWOpts;
use http::HttpSubCommand;

use self::text::TextSubCommand;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author,about,long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),

    #[command(name = "genpw", about = " Generate a random password")]
    GenPW(GenPWOpts),

    #[command(name = "base64", subcommand)]
    Base64(Base64Subcommand),

    #[command(name = "text", subcommand)]
    Text(TextSubCommand),

    #[command(name = "http", subcommand)]
    Http(HttpSubCommand),
}

pub fn verify_file(filename: &str) -> Result<String, &'static str> {
    if std::path::Path::new(filename).exists() || filename == "-" {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

pub fn verify_path(filename: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(filename);
    if p.exists() && p.is_dir() {
        Ok(p.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File does not exist"));
        assert_eq!(verify_file("not_exist.txt"), Err("File does not exist"));
    }
}
