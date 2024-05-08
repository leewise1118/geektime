pub mod cli;
pub mod process;
pub mod utils;
use crate::cli::{
    base64::{Base64DecodeOpts, Base64EncodeOpts, Base64Subcommand},
    csv::CsvOpts,
    genpw::GenPWOpts,
    http::{HttpServeOpts, HttpSubCommand},
    text::{
        TextDecryptOpts, TextEncryptOpts, TextKeyGenerateOpts, TextSignOpts, TextSubCommand,
        TextVerifyOpts,
    },
    SubCommand,
};

use enum_dispatch::enum_dispatch;
pub use process::{csv_convert::process_csv, gen_pass::process_genpasswd};
#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}
