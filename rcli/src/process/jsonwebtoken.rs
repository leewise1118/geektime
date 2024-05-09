use std::{io::Read, ops::Add, time::SystemTime};

use anyhow::{Ok, Result};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::info;

use crate::{cli::jwt::JwtKeyFormat, utils::get_reader};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    nbf: usize, // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}
impl Claims {
    pub fn new(aud: &str, exp: usize, iat: usize, iss: &str, nbf: usize, sub: &str) -> Self {
        Self {
            aud: aud.to_string(),
            exp,
            iat,
            iss: iss.to_string(),
            nbf,
            sub: sub.to_string(),
        }
    }
}

pub fn process_jwt_sign(
    key: &str,
    format: JwtKeyFormat,
    algorithm: Algorithm,
    sub: &str,
    aud: &str,
    exp: Duration,
    iat: Duration,
    iss: &str,
    nbf: Duration,
) -> Result<String> {
    let mut reader = get_reader(key)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();
    let buf = buf.as_bytes().to_vec();

    let encoded_key = match format {
        JwtKeyFormat::Secret => EncodingKey::from_secret(buf.as_ref()),
        JwtKeyFormat::Base64Secret => EncodingKey::from_base64_secret(std::str::from_utf8(&buf)?)?,
        JwtKeyFormat::Rsa => EncodingKey::from_rsa_pem(buf.as_ref())?,
        JwtKeyFormat::ECDSA => EncodingKey::from_ec_pem(buf.as_ref())?,
        JwtKeyFormat::EdDSA => EncodingKey::from_ed_pem(buf.as_ref())?,
        JwtKeyFormat::RsaDer => EncodingKey::from_rsa_der(buf.as_ref()),
        JwtKeyFormat::ECDSADer => EncodingKey::from_ec_der(buf.as_ref()),
        JwtKeyFormat::EdDSADer => EncodingKey::from_ed_der(buf.as_ref()),
    };
    let exp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .add(exp)
        .as_secs();
    let iat = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .add(iat)
        .as_secs();
    let nbf = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .add(nbf)
        .as_secs();

    let my_claims = Claims::new(aud, exp as usize, iat as usize, iss, nbf as usize, sub);
    info!("{:?}", my_claims);
    let token = encode(&Header::new(algorithm), &my_claims, &encoded_key)?;
    Ok(token)
}

pub fn process_jwt_verify(
    key: &str,
    format: JwtKeyFormat,
    algorithm: Algorithm,
    token: &str,
    aud: &str,
) -> Result<Claims> {
    let mut reader = get_reader(key)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();
    let buf = buf.as_bytes().to_vec();

    let decoded_key = match format {
        JwtKeyFormat::Secret => DecodingKey::from_secret(buf.as_ref()),
        JwtKeyFormat::Base64Secret => DecodingKey::from_base64_secret(std::str::from_utf8(&buf)?)?,
        JwtKeyFormat::Rsa => DecodingKey::from_rsa_pem(buf.as_ref())?,
        JwtKeyFormat::ECDSA => DecodingKey::from_ec_pem(buf.as_ref())?,
        JwtKeyFormat::EdDSA => DecodingKey::from_ed_pem(buf.as_ref())?,
        JwtKeyFormat::RsaDer => DecodingKey::from_rsa_der(buf.as_ref()),
        JwtKeyFormat::ECDSADer => DecodingKey::from_ec_der(buf.as_ref()),
        JwtKeyFormat::EdDSADer => DecodingKey::from_ed_der(buf.as_ref()),
    };
    let mut valid = Validation::new(algorithm);
    if aud.is_empty() {
        valid.validate_aud = false;
    } else {
        valid.set_audience(&[aud]);
    }
    let my_claim = decode::<Claims>(&token, &decoded_key, &valid)?;
    info!("{:?}", my_claim.claims);

    Ok(my_claim.claims)
}

#[cfg(test)]
mod tests {
    use super::*;
    use humantime::parse_duration;

    #[test]
    fn test_jwt_sign_verify() {
        let key = "./fixtures/jwt.txt";
        let format = JwtKeyFormat::Secret;
        let algorithm = Algorithm::HS256;
        let sub = "user";
        let aud = "admin";
        let exp = parse_duration("14d").unwrap();
        let iat = parse_duration("13d").unwrap();
        let iss = "lee";
        let nbf = parse_duration("12d").unwrap();

        let signed =
            process_jwt_sign(key, format, algorithm, sub, aud, exp, iat, iss, nbf).unwrap();
        println!("{:?}", signed);

        let claim = process_jwt_verify(key, format, algorithm, &signed, aud);
        assert!(claim.is_ok());
        let claim = claim.unwrap();
        assert_eq!(claim.aud, aud);
        assert_eq!(claim.sub, sub);
        assert_eq!(claim.iss, iss);
    }
}
