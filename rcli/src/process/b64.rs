use anyhow::Result;
use base64::engine::general_purpose::{STANDARD, URL_SAFE};
use base64::prelude::*;
use std::fs::File;
use std::io::Read;

use crate::cli::base64::Base64Format;

pub fn process_encode(input: &str, format: Base64Format) -> Result<()> {
    // 封装在堆上，然后通过trait对象的方式传递
    // TODO: 为什么要这样做？看trait对象相关的内容
    let mut readr: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    let mut data = Vec::new();
    readr.read_to_end(&mut data)?;

    let encode = match format {
        Base64Format::Standard => STANDARD.encode(&data),
        Base64Format::UrlSafe => URL_SAFE.encode(&data),
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
    let mut data = Vec::new();
    readr.read_to_end(&mut data)?;

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(&data)?,
        Base64Format::UrlSafe => URL_SAFE.decode(&data)?,
    };

    // TODO: decoded data might not be String (but for this example, we assume it is)
    let decoded = String::from_utf8(decoded)?;
    println!("{}", decoded);

    Ok(())
}
