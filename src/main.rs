use std::{io::{stdin, stdout, BufRead, Read, Write}, num::ParseIntError, str::Utf8Error};

use base64::{prelude::{BASE64_STANDARD, BASE64_URL_SAFE}, Engine};
use bip39::{Mnemonic, MnemonicType};
use clap::ArgMatches;
use cli::create;
use lang::{LanguageTool, OutputFormat};

pub mod cli;
pub mod lang;

#[derive(Debug)]
pub enum SeedPhraserError {
    MissingArgument(String),
    FailedParsingArguments(String),
    UnrecognizableLanguage(String),
    FailedGeneratingMnemonic(String),
    FailedToReadOutputFormat(String),
    IOError(std::io::Error),
    Utf8Error(std::str::Utf8Error)
}

impl From<Utf8Error> for SeedPhraserError {
    fn from(value: Utf8Error) -> Self {
        Self::Utf8Error(value)
    }
}

impl From<ParseIntError> for SeedPhraserError {
    fn from(value: ParseIntError) -> Self {
        Self::FailedParsingArguments(value.to_string())
    }
}

impl From<bip39::ErrorKind> for SeedPhraserError {
    fn from(value: bip39::ErrorKind) -> Self {
        Self::FailedGeneratingMnemonic(value.to_string())
    }
}

impl From<std::io::Error> for SeedPhraserError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

pub fn generate(sub_matches: &ArgMatches) -> Result<(), SeedPhraserError> {
    let language = LanguageTool::parse(sub_matches)?;
    let bits = sub_matches
        .get_one::<String>("bits")
        .map(|f| str::parse::<usize>(f))
        .expect("Required argument.")?;
    let output = OutputFormat::parse("output", sub_matches)?;

    let mnemonic = Mnemonic::new(MnemonicType::for_key_size(bits)?, language);
    output.output(mnemonic)?;
    
    Ok(())
}


fn read_all_stdin() -> Result<Vec<u8>, SeedPhraserError> {
    let mut total = Vec::new();
    let mut buf = [0u8; 256];
    let mut pos = 0;
    loop {
        pos = stdin().read(&mut buf)?;
        if pos == 0 {
            break
        }
        total.extend_from_slice(&buf[..pos]);
    }
    Ok(total)
}

pub fn decode_sequence(sub_matches: &ArgMatches) -> Result<(), SeedPhraserError> {
    let language = LanguageTool::parse(sub_matches)?;
    let input = OutputFormat::parse("input", sub_matches)?;
    let output = OutputFormat::parse("output", sub_matches)?;

    println!("Input: {input:?}, Output: {output:?}");

    match input {
        OutputFormat::Text => {
            let data = std::str::from_utf8(&read_all_stdin()?)?;
        },
        _ => unreachable!()
    }

    Ok(())
}

pub fn main() -> Result<(), SeedPhraserError> {
    let matches = create().get_matches();
    match matches.subcommand() {
        Some(("generate", sub_matches)) => generate(sub_matches)?,
        Some(("decode", sub_matches)) => decode_sequence(sub_matches)?,
        None => (),
        _ => (),
    }
    Ok(())
}
