pub mod base64;
pub mod csv;
pub mod genpw;
use base64::Base64Subcommand;
use clap::Parser;
use csv::CsvOpts;
use genpw::GenPWOpts;

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

    #[command(subcommand)]
    Base64(Base64Subcommand),
}

pub fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if std::path::Path::new(filename).exists() || filename == "-" {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("File does not exist"));
        assert_eq!(
            verify_input_file("not_exist.txt"),
            Err("File does not exist")
        );
    }
}
