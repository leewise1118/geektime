// rcli csv -i input.csv -o output.json --header -d ","

use std::fs;

use anyhow::Result;
use clap::Parser;
use rcli::cli::base64::Base64Subcommand;
use rcli::cli::http::{HttpServeOpts, HttpSubCommand};
use rcli::cli::text::{TextSignFormat, TextSubCommand};
use rcli::cli::{Opts, SubCommand};
use rcli::process::b64::{process_decode, process_encode};
use rcli::process::csv_convert::process_csv;
use rcli::process::gen_pass::process_genpasswd;
use rcli::process::http_serve::process_http_serve;
use rcli::process::text::{
    process_text_decrypt, process_text_encrypt, process_text_generate, process_text_sign,
    process_text_verify,
};
use zxcvbn::zxcvbn;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
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
            let pwd = process_genpasswd(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            print!("{:?}", pwd);

            let estimate = zxcvbn(&pwd, &[])?;
            eprintln!("Password strength: {:?}", estimate.score());
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64Subcommand::Encode(opts) => {
                let encode = process_encode(&opts.input, opts.format)?;
                println!("{}", encode);
            }
            Base64Subcommand::Decode(opts) => {
                let decoded = process_decode(&opts.input, opts.format)?;
                println!("{:?}", String::from_utf8(decoded));
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                let signed = process_text_sign(&opts.input, &opts.key, opts.format)?;
                println!("{:?}", signed);
            }
            TextSubCommand::Verify(opts) => {
                let verified = process_text_verify(&opts.input, &opts.key, &opts.sig, opts.format)?;
                println!("{:?}", verified);
            }
            TextSubCommand::Generate(opts) => {
                let key = process_text_generate(opts.format)?;
                match opts.format {
                    TextSignFormat::Blake3 => {
                        let name = opts.output.join("blake3.txt");
                        fs::write(name, &key[0])?;
                    }
                    TextSignFormat::Ed25519 => {
                        let sk_name = opts.output.join("ed25519.sk");
                        let pk_name = opts.output.join("ed25519.pk");
                        fs::write(sk_name, &key[0])?;
                        fs::write(pk_name, &key[1])?;
                    }
                    TextSignFormat::ChaCha20Poly1305 => {
                        let name = opts.output.join("chacha20poly1305.txt");
                        println!("key = {:?}, length = {:?}", &key[0], key[0].len());
                        fs::write(name, &key[0])?;
                    }
                }
            }
            TextSubCommand::Encrypt(opts) => {
                let encrypted_text = process_text_encrypt(&opts.input, &opts.key)?;
                println!("{:?}", encrypted_text);
            }
            TextSubCommand::Dncrypt(opts) => {
                let plaintext = process_text_decrypt(&opts.input, &opts.key)?;
                println!("{:?}", plaintext);
            }
        },
        SubCommand::Http(subcmd) => match subcmd {
            HttpSubCommand::Serve(opts) => {
                process_http_serve(opts.dir, opts.port).await?;
            }
        },
    }
    Ok(())
}
