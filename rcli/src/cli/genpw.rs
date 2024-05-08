use clap::Parser;
use zxcvbn::zxcvbn;

use crate::{process_genpasswd, CmdExecutor};

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

impl CmdExecutor for GenPWOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let pwd = process_genpasswd(
            self.length,
            self.uppercase,
            self.lowercase,
            self.number,
            self.symbol,
        )?;
        println!("{}", pwd);
        let estimate = zxcvbn(&pwd, &[])?;
        eprintln!("Password strength: {:?}", estimate);
        Ok(())
    }
}
