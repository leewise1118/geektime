use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::CmdExecutor;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum JsonWebTokenSubcommand {
    #[command(about = "")]
    Sign(JWTSignOpts),

    #[command(about = "")]
    Verify(JWTVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct JWTSignOpts {
    #[arg(short, long)]
    pub key: String,

    /// 主题 Subject
    #[arg(long)]
    pub sub: String,

    /// 受众 audience
    #[arg(long)]
    pub aud: String,

    /// 过期时间 expiration time
    #[arg(long)]
    pub exp: usize,

    /// 签发时间 Issued At
    #[arg(long)]
    pub iat: usize,

    /// 签发人 issuer
    #[arg(long)]
    pub iss: String,

    /// 生效时间 Not Before
    #[arg(long)]
    pub nbf: usize,
}

#[derive(Debug, Parser)]
pub struct JWTVerifyOpts {}

impl CmdExecutor for JWTSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        todo!()
    }
}

impl CmdExecutor for JWTVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        todo!()
    }
}
