use std::fmt::Display;
use std::str::FromStr;

use anyhow::Result;
use clap::Parser;

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
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long,value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long)] // ``` "output.json".into() ```
    pub output: Option<String>,

    #[arg(long,value_parser = parse_format)]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

#[derive(Debug, Parser)]
pub struct GenPWOpts {
    #[arg(short, long, default_value_t = 8)]
    pub length: u8,

    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(short, long, default_value_t = true)]
    pub number: bool,

    #[arg(short, long, default_value_t = true)]
    pub symbol: bool,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if std::path::Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

fn parse_format(format: &str) -> Result<OutputFormat> {
    format.parse::<OutputFormat>()
}

impl From<OutputFormat> for &'static str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}
impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            v => anyhow::bail!("Unsupported format {}", v),
        }
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
