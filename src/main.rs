use std::{io::{stdin, Read}, num::ParseIntError, str::Utf8Error};

use base64::{prelude::BASE64_STANDARD, DecodeError, Engine};
use clap::ArgMatches;
use cli::create;
use error::SeedPhraserError;
use gen::AdvancedMnemonic;
use hex::FromHexError;
use lang::{LanguageTool, IoFormat};

pub mod cli;
pub mod lang;
pub mod gen;
pub mod error;


pub fn generate(sub_matches: &ArgMatches) -> Result<(), SeedPhraserError> {
    let language = LanguageTool::parse(sub_matches)?;
    let bits = sub_matches
        .get_one::<String>("bits")
        .map(|f| str::parse::<usize>(f))
        .expect("Required argument.")?;
    let output = IoFormat::parse("output", sub_matches)?;

    AdvancedMnemonic::generate(bits, language, true)?.output(output)?;

 
    Ok(())
}


fn read_all_stdin() -> Result<Vec<u8>, SeedPhraserError> {
    let mut total = Vec::new();
    let mut buf = [0u8; 256];
    loop {
        let pos = stdin().read(&mut buf)?;
        if pos == 0 {
            break
        }
        total.extend_from_slice(&buf[..pos]);
    }
    Ok(total)
}

pub fn decode_sequence_direct(
    phrase: &String,
    sub_matches: &ArgMatches
) -> Result<(), SeedPhraserError> {
    let language = LanguageTool::parse(sub_matches)?;
    let output = IoFormat::parse("output", sub_matches)?;

    AdvancedMnemonic::from_phrase(phrase.trim(), language)?.output(output)?;
    Ok(())
}

pub fn decode_sequence(sub_matches: &ArgMatches) -> Result<(), SeedPhraserError> {
    let language = LanguageTool::parse(sub_matches)?;
    let input = IoFormat::parse("input", sub_matches)?;
    let output = IoFormat::parse("output", sub_matches)?;


    let buf = read_all_stdin()?;
    let buffer = &buf[..buf.len() - 2];

    let entropy = match input {
        IoFormat::Text => {
            let data = std::str::from_utf8(&buffer)?;
            // println!("Data: {:?}", data);
            let mnec = AdvancedMnemonic::from_phrase(data, language)?;
            mnec.output(output)?;
            return Ok(())
        },
        IoFormat::Base64 => BASE64_STANDARD.decode(buffer)?,
        IoFormat::Base64UrlSafe => BASE64_STANDARD.decode(buffer)?,
        IoFormat::Binary => buffer.to_vec(),
        IoFormat::Hex => hex::decode(buffer)?
    };

    AdvancedMnemonic::from_entropy(&entropy, language, true)?.output(output)?;
    // output.output(Mnemonic::from_entropy(&entropy, language)?)?;
    Ok(())
}

pub fn main() -> Result<(), SeedPhraserError> {
    let matches = create().get_matches();
    match matches.subcommand() {
        Some(("generate", sub_matches)) => generate(sub_matches)?,
        Some(("decode", sub_matches)) => match sub_matches.get_one::<String>("phrase") {
            Some(phrase) => decode_sequence_direct(phrase, sub_matches)?,
            None => decode_sequence(sub_matches)?
        },
        None => (),
        _ => (),
    }
    Ok(())
}
