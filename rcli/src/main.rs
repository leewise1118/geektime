// rcli csv -i input.csv -o output.json --header -d ","

use anyhow::Result;
use clap::Parser;
use rcli::{cli::Opts, CmdExecutor};
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let rcli = Opts::parse();
    rcli.cmd.execute().await?;

    Ok(())
}
