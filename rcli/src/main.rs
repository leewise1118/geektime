// rcli csv -i input.csv -o output.json --header -d ","

use anyhow::Result;
use clap::Parser;
use rcli::process::csv_convert::process_csv;
use rcli::process::gen_pass::process_genpasswd;
use rcli::{Opts, SubCommand};

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
    }
    Ok(())
}
