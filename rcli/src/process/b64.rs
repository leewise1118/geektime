use crate::utils::get_reader;
use anyhow::Result;
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::prelude::*;
use std::fs::File;
use std::io::Read;

use crate::cli::base64::Base64Format;

pub fn process_encode(input: &str, format: Base64Format) -> Result<()> {
    // println!("input: {}, format: {:?}", input, format);

    // 封装在堆上，然后通过trait对象的方式传递
    // TODO: 为什么要这样做？看trait对象相关的内容
    let mut reader = get_reader(input)?;

    let mut data = Vec::new();
    reader.read_to_end(&mut data)?;

    let encode = match format {
        Base64Format::Standard => STANDARD.encode(&data),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&data),
    };
    println!("{}", encode);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<()> {
    let mut readr: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    let mut data = String::new();
    readr.read_to_string(&mut data)?;

    // avoid accidental newlines
    let data = data.trim();

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(data)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(data)?,
    };

    // TODO: decoded data might not be String (but for this example, we assume it is)
    let decoded = String::from_utf8(decoded)?;
    println!("{}", decoded);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format1 = Base64Format::Standard;
        assert!(process_encode(input, format1).is_ok());

        let format2 = Base64Format::UrlSafe;

        assert!(process_encode(input, format2).is_ok());
    }

    #[test]
    fn test_process_decode_standard() {
        let input = "../fixtures/standard_tmp.b64";
        let format = Base64Format::Standard;
        assert!(process_decode(input, format).is_ok());
    }

    #[test]
    fn test_process_decode_urlsafe() {
        let input = "../fixtures/urlsafe_tmp.b64";
        let format = Base64Format::UrlSafe;
        assert!(process_decode(input, format).is_ok());
    }
}
