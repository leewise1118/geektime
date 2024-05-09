use anyhow::Result;
use clap::{FromArgMatches, Parser};
use enum_dispatch::enum_dispatch;
use jsonwebtoken::Algorithm;
use std::str::FromStr;
use std::time::Duration;
use tracing::info;

use crate::{
    process::jsonwebtoken::{process_jwt_sign, process_jwt_verify},
    CmdExecutor,
};

use super::verify_file;

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
    /// key for JWT
    #[arg(short, long, default_value = "-", value_parser = verify_file)]
    pub key: String,

    #[arg(long, default_value = "secret", value_parser=parse_jwt_sign_format)]
    pub format: JwtKeyFormat,

    #[arg(long,default_value= "HS256", value_parser = parse_jwt_algorithm)]
    pub algorithm: Algorithm,

    /// 主题 Subject
    #[arg(long)]
    pub sub: String,

    /// 受众 audience
    #[arg(long)]
    pub aud: String,

    /// 过期时间 expiration time
    #[arg(long, value_parser = humantime::parse_duration)]
    pub exp: Duration,

    /// 签发时间 Issued At
    #[arg(long, value_parser = humantime::parse_duration)]
    pub iat: Duration,

    /// 签发人 issuer
    #[arg(long)]
    pub iss: String,

    /// 生效时间 Not Before
    #[arg(long, value_parser = humantime::parse_duration)]
    pub nbf: Duration,
}
#[derive(Debug, Parser)]
pub struct JWTVerifyOpts {
    #[arg(short, long, default_value = "-", value_parser = verify_file)]
    pub key: String,

    #[arg(long, default_value = "secret", value_parser=parse_jwt_sign_format)]
    pub format: JwtKeyFormat,

    #[arg(long,default_value= "HS256", value_parser = parse_jwt_algorithm)]
    pub algorithm: Algorithm,

    #[arg(long)]
    pub token: String,

    #[arg(long, default_value = "")]
    pub aud: String,
}

impl CmdExecutor for JWTSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let sign = process_jwt_sign(
            &self.key,
            self.format,
            self.algorithm,
            &self.sub,
            &self.aud,
            self.exp,
            self.iat,
            &self.iss,
            self.nbf,
        )?;
        println!("Json Web token is : {:?}", sign);
        Ok(())
    }
}

impl CmdExecutor for JWTVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let claim = process_jwt_verify(
            &self.key,
            self.format,
            self.algorithm,
            &self.token,
            &self.aud,
        );
        println!("{:?}", claim.is_ok());
        let claim = claim.unwrap();
        println!("claim = {:?}", claim);
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum JwtKeyFormat {
    Secret,
    Base64Secret,
    Rsa,
    ECDSA,
    EdDSA,
    RsaDer,
    ECDSADer,
    EdDSADer,
}
impl FromStr for JwtKeyFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "secret" => Ok(Self::Secret),
            "base64-secret" => Ok(Self::Base64Secret),
            "rsa" => Ok(Self::Rsa),
            "ecdsa" => Ok(Self::ECDSA),
            "eddsa" => Ok(Self::EdDSA),
            "rsa-der" => Ok(Self::RsaDer),
            "ecdsa-der" => Ok(Self::ECDSADer),
            "eddsa-der" => Ok(Self::EdDSADer),
            _ => Err(anyhow::anyhow!("invalid key type")),
        }
    }
}

impl From<JwtKeyFormat> for &str {
    fn from(key_type: JwtKeyFormat) -> Self {
        match key_type {
            JwtKeyFormat::Secret => "secret",
            JwtKeyFormat::Base64Secret => "base64Secret",
            JwtKeyFormat::Rsa => "rsa",
            JwtKeyFormat::ECDSA => "ecdsa",
            JwtKeyFormat::EdDSA => "eddsa",
            JwtKeyFormat::RsaDer => "rsaDer",
            JwtKeyFormat::ECDSADer => "ecdsaDer",
            JwtKeyFormat::EdDSADer => "eddsaDer",
        }
    }
}

fn parse_jwt_sign_format(s: &str) -> Result<JwtKeyFormat> {
    s.parse()
}

fn parse_jwt_algorithm(s: &str) -> Result<Algorithm, jsonwebtoken::errors::Error> {
    s.parse()
}
