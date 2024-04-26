// rcli csv -i input.csv -o output.json --header -d ","

use anyhow::Result;
use clap::Parser;
use rcli::cli::base64::Base64Subcommand;
use rcli::cli::text::{TextSignFormat, TextSubCommand};
use rcli::cli::{Opts, SubCommand};
use rcli::process::b64::{process_decode, process_encode};
use rcli::process::csv_convert::process_csv;
use rcli::process::gen_pass::process_genpasswd;
use rcli::process::text::process_sign;

fn main() -> Result<()> {
    let rcli = Opts::parse();
    match rcli.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                format!("{}.{}", output.clone(), opts.format)
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPW(opts) => {
            process_genpasswd(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64Subcommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?;
            }
            Base64Subcommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => match opts.format {
                TextSignFormat::Blake3 => {
                    process_sign(&opts.input, &opts.key, opts.format)?;
                }
                TextSignFormat::Ed25519 => {
                    todo!()
                }
            },
            TextSubCommand::Verify(opts) => {
                println!("Verify: {:?}", opts);
            }
        },
    }
    Ok(())
}
