use crate::utils::get_reader;
use anyhow::Result;
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::prelude::*;
use std::fs::File;
use std::io::Read;

use crate::cli::base64::Base64Format;

pub fn process_encode(input: &str, format: Base64Format) -> Result<String> {
    // println!("input: {}, format: {:?}", input, format);

    // 封装在堆上，然后通过trait对象的方式传递
    // TODO: 为什么要这样做？看trait对象相关的内容
    let mut reader = get_reader(input)?;

    let mut data = Vec::new();
    reader.read_to_end(&mut data)?;
    data = data[..data.len() - 1].to_vec(); // remove trailing newline

    let encode = match format {
        Base64Format::Standard => STANDARD.encode(&data),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&data),
    };
    Ok(encode)
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<Vec<u8>> {
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

    Ok(decoded)
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

    #[test]
    fn test_process_encode_and_decode_standard() {
        let input = "../fixtures/base64_input.txt";
        let format = Base64Format::Standard;
        println!("input: {}", input);
        let encoded = process_encode(input, format).unwrap();
        // 将encode保存到文件中
        println!("encoded: {}", encoded);
        let output = "../fixtures/standard_test_tmp.b64";
        std::fs::write(output, encoded).unwrap();

        let decoded = process_decode(output, format).unwrap();
        println!("decoded: {:?}", decoded);
        assert_eq!(b"hello world".to_vec(), decoded);
    }
    #[test]
    fn test_process_encode_and_decode_urlsafe() {
        let input = "../fixtures/base64_input.txt";
        let format = Base64Format::UrlSafe;
        println!("input: {}", input);
        let encoded = process_encode(input, format).unwrap();
        // 将encode保存到文件中
        println!("encoded: {}", encoded);
        let output = "../fixtures/urlsafe_test_tmp.b64";
        std::fs::write(output, encoded).unwrap();

        let decoded = process_decode(output, format).unwrap();
        println!("decoded: {:?}", decoded);
        assert_eq!(b"hello world".to_vec(), decoded);
    }
}
